use std::fmt;
use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{PromiseOrValue, serde_json};
use near_sdk::AccountId;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub enum Actions {
    Multisend {
        recipients: Vec<AccountId>,
        amounts: Vec<WBalance>,
    }
}

impl fmt::Display for Actions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for Multisender {


    /// Transfers positive `amount` of tokens from the `env::predecessor_account_id` to `receivers`.
    /// This function is executed when user call `ft_transfer_call` on a fungible token contracts.
    /// See [ft_transfer_call](https://github.com/near/NEPs/blob/master/neps/nep-0141.md#ft_transfer_call).
    /// Requirements: - `msg` argument must follow this format `"msg": "bob.testnet:20#alice.testnet:50"`.
    /// This means `bob.testnet` receive 20 tokens and `alice.testnet` receive 50 tokens.
    /// - Both `bob` and `alice` must register storage for token contract in advance.
    /// - `sender_id` balance must be greater or equal to the total amount sent to each receiver.
    /// - `sender_id` must pay service fee to get enough quota to transfer near. Arguments: - `sender_id`: the account id of sender.
    /// - `amount`: the amount of token that sender transfer to this contract by calling `ft_transfer_call`.
    /// - `msg` - a string message that includes receivers and amount.

    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let action: Actions = serde_json::from_str(&msg).expect("Incorrect command in transfer");

        let mut final_amount = 0 as Balance;
        let mut taxes = 0 as Balance;

        match action {
            Actions::Multisend { recipients, amounts } => {
                for (recipient, amount) in recipients.iter().zip(amounts.iter()) {
                    require!(*amount > U128(0));
                    assert!(
                        env::is_valid_account_id(recipient.as_bytes()),
                        "Account @{} is invalid",
                        recipient
                    );

                    let fee = amount.0 * self.percentage / 1000;
                    taxes += fee;

                    final_amount += amount.0;
                    final_amount += fee;
                }

                assert!(amount >= U128(final_amount),
                        "Not enough supplied tokens to run multi-sender (Supplied: {}. Demand: {})",
                        amount.0,
                        final_amount);
                PromiseOrValue::Value(U128(0))
                //
                // for (recipient, amount) in recipients.iter().zip(amounts.iter()){
                //     Promise::new(recipient.clone())
                //         .transfer(amount.0);


                // paying out the taxes for the bank account in NEAR token in yocto precision
                // Promise::new(self.bank.clone())
                //     .transfer(taxes);
            }
        }
    }
}





