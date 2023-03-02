pub mod admin;
mod upgrade;
mod ft_multisend;

use near_sdk::{env, near_bindgen, AccountId, require, Balance, Promise};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::env::attached_deposit;

pub type WBalance = U128;

pub fn ntoy(near_amount: Balance) -> Balance {
    // near to yocto convert
    near_amount * 10u128.pow(24)
}

pub fn yton(yocto_amount: Balance) -> Balance {
    // yocto to near convert
    yocto_amount / 10u128.pow(24)
}

#[derive(BorshDeserialize, BorshSerialize)]
#[near_bindgen]
pub struct Multisender {
    /// fee that charges from sender ( by default 0.1% )
    percentage: u128,

    /// account we are sending money from
    bank: AccountId,

    /// Contract owner account (contract itself by default)
    pub admin: AccountId,
}


impl Default for Multisender {
    fn default() -> Self {
        env::panic_str("Token contract should be initialized before usage")
    }
}

#[near_bindgen]
impl Multisender {
    /// Initializes the contract. Needs to be called once.
    #[init]
    pub fn initialize(bank: AccountId, admin: AccountId) -> Self {
        require!(!env::state_exists(), "Already initialized");

        Self {
            percentage: 10,
            bank,
            admin,
        }
    }

    #[payable]
    pub fn multi_send_from_attached_deposit_near(&mut self, recipients: Vec<AccountId>, amounts: Vec<WBalance>) {
        require!(recipients.len() > 0);

        let mut final_amount = 0 as Balance;
        let mut taxes = 0 as Balance;

        for (recipient, amount) in recipients.iter().zip(amounts.iter()) {
            // non zero amounts and correct recipient address
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

        assert!(
            attached_deposit() >= final_amount,
            "Not enough attached tokens to run multi-sender (Supplied: {}. Demand: {})",
            attached_deposit(),
            final_amount
        );

        let mut logs: String = "".to_string();

        for (recipient, amount) in recipients.iter().zip(amounts.iter()){
            let log = format!("Sending {} yNEAR (~{} NEAR) to account @{}\n", amount.0, yton(amount.0), recipient);
            logs.push_str(&log);

            Promise::new(recipient.clone())
                .transfer(amount.0);
        }

        env::log_str(&*format!("Done!\n{}", logs));

        // paying out the taxes for the bank account in NEAR token in yocto precision
        Promise::new(self.bank.clone())
            .transfer(taxes);
    }
}