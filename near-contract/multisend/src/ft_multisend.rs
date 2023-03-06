use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::json_types::U128;

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    ext_contract, log, near_bindgen,
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
    pub amount: U128,
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
        log!(format!("sender_id {sender_id}, msg {msg}"));

        // Get the transfers from the input parameter
        let transfers: MultiSendInput =
            near_sdk::serde_json::from_str(&msg).expect("Invalid input in the msg");

        // Check that the token ID matches the expected token ID
        assert_eq!(
            env::predecessor_account_id(),
            transfers.token_id,
            "The call should come from token account"
        );

        let mut taxes: Balance = 0;

        // Calculate the total transfer amount with taxes
        let total_transfer_amount: Balance = transfers
            .transfers
            .iter()
            .map(|transfer| {
                // calculating the general taxes
                taxes += transfer.amount.0 * self.percentage / 1000;
                transfer.amount.0 + transfer.amount.0 * self.percentage / 1000
            }) // adding amount and considering the fee
            .sum();

        assert_eq!(
            total_transfer_amount, amount.0,
            "Not enough supplied tokens to run multi-sender (Supplied: {}. Demand: {})",
            amount.0, total_transfer_amount
        );

        // Distribute the tokens to the recipients
        for transfer in transfers.transfers {
            // non zero amounts and correct recipient address
            let transfer_amount = transfer.amount;

            require!(transfer_amount.0 > 0);

            let recipient = transfer.recipient;

            require!(env::is_valid_account_id(recipient.as_bytes()));

            Promise::new(transfers.token_id.clone()).function_call(
                "ft_transfer".to_string(),
                // Arguments are encoded as a JSON string
                format!(
                    r#"{{"receiver_id": "{}", "amount": "{}"}}"#,
                    recipient, transfer_amount.0
                )
                .as_bytes()
                .to_vec(),
                // Deposit for transferring tokens, this amount should be the same as the amount transferred
                1,
                // Gas limit for the function call
                Gas(1_000_000_000_000),
            );
        }

        Promise::new(transfers.token_id)
            .function_call(
                "ft_transfer".to_string(),
                // Arguments are encoded as a JSON string
                format!(
                    r#"{{"receiver_id": "{}", "amount": "{}"}}"#,
                    self.bank, taxes
                )
                .as_bytes()
                .to_vec(),
                // Deposit for transferring tokens, this amount should be the same as the amount transferred
                1,
                // Gas limit for the function call
                Gas(1_000_000_000_000),
            )
            .into()
    }
}
