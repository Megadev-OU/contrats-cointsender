use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::json_types::U128;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    ext_contract, near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, Balance, Gas, Promise, PromiseOrValue,
};

#[ext_contract]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TransferInput {
    pub amount: Balance,
    pub recipient: AccountId,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MultiSendInput {
    pub token_id: AccountId,
    pub transfers: Vec<TransferInput>,
}

#[near_bindgen]
impl FungibleTokenReceiver for Multisender {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // Get the token ID from the sender ID
        let token_id = sender_id.clone();

        // Get the transfers from the input parameter
        let transfers: MultiSendInput =
            near_sdk::serde_json::from_str(&msg).expect("Invalid input");

        // Check that the token ID matches the expected token ID
        assert_eq!(transfers.token_id, token_id, "Invalid token ID");

        let mut taxes: Balance = 0;

        // Calculate the total transfer amount with taxes
        let total_transfer_amount: Balance = transfers
            .transfers
            .iter()
            .map(|transfer| {
                // calculating the general taxes
                taxes += transfer.amount * self.percentage / 1000;
                transfer.amount + transfer.amount * self.percentage / 1000
            }) // adding amount and considering the fee
            .sum();

        assert_eq!(
            total_transfer_amount, amount.0,
            "Not enough supplied tokens to run multi-sender (Supplied: {}. Demand: {})",
            amount.0, total_transfer_amount
        );

        // Distribute the tokens to the recipients
        for transfer in transfers.transfers {
            let recipient = transfer.recipient;
            let transfer_amount = transfer.amount;
            Promise::new(token_id.clone()).function_call(
                "ft_transfer".to_string(),
                near_sdk::serde_json::to_vec(&recipient).unwrap(),
                transfer_amount,
                Gas(1_000_000_000_000), // gas attached
            );
        }

        // Distribute taxes to the bank account
        Promise::new(token_id.clone())
            .function_call(
                "ft_transfer".to_string(),
                near_sdk::serde_json::to_vec(&self.bank).unwrap(),
                taxes,
                Gas(1_000_000_000_000), // gas attached
            )
            .into()
    }
}
