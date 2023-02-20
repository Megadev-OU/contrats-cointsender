use near_sdk::{env, require, AccountId, Balance};

use crate::*;

#[near_bindgen]
impl Multisender {
    pub fn get_admin(&self) -> AccountId {
        self.admin.clone()
    }

    pub fn set_admin(&mut self, account: AccountId) {
        require!(
            self.is_valid_admin_call(),
            "This functionality is allowed to be called by admin or contract only"
        );
        self.admin = account;
    }

    pub fn get_bank_address(&self) -> AccountId {
        self.bank.clone()
    }

    pub fn change_bank_address(&mut self, bank: AccountId) {
        require!(
            self.is_valid_admin_call(),
            "This functionality is allowed to be called by admin or contract only"
        );

        self.bank = bank;
    }

    pub fn get_percentage(&self) -> Balance {
        self.percentage
    }

    pub fn change_percentage(&mut self, percent: U128) {
        require!(
            self.is_valid_admin_call(),
            "This functionality is allowed to be called by admin or contract only"
        );

        self.percentage = percent.0;
    }
}

impl Multisender {
    pub fn is_valid_admin_call(&self) -> bool {
        env::signer_account_id() == self.admin
            || env::signer_account_id() == env::current_account_id()
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::test_utils::test_env::{alice, bob};
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};
    use super::*;

    pub fn get_context(is_view: bool, current: AccountId, signer: AccountId) -> VMContext {
        VMContextBuilder::new()
            .current_account_id(current)
            .signer_account_id(signer)
            .is_view(is_view)
            .build()
    }


    #[test]
    fn test_change_bank_address_success() {
        // setting up correct admin that are allowed to change bank account
        let mut multisend_contract = Multisender::initialize(alice(), alice());

        testing_env!(get_context(false, alice(), alice()));

        assert_eq!(multisend_contract.get_bank_address(), alice());

        multisend_contract.change_bank_address(bob());

        assert_eq!(multisend_contract.get_bank_address(), bob());
    }

    #[test]
    #[should_panic(expected = "This functionality is allowed to be called by admin or contract only")]
    fn test_change_bank_address_fail() {
        // setting up wrong admin that are NOT allowed to change bank account
        let mut multisend_contract = Multisender::initialize(alice(), alice());

        testing_env!(get_context(false, alice(), bob()));

        assert_eq!(multisend_contract.get_bank_address(), alice());

        multisend_contract.change_bank_address(bob());
    }


    #[test]
    fn test_change_percentage_success() {
        // setting up correct admin that are allowed to change percentage
        let mut multisend_contract = Multisender::initialize(alice(), alice());

        testing_env!(get_context(false, alice(), alice()));

        assert_eq!(multisend_contract.get_percentage(), 10); // represents 0.1%

        multisend_contract.change_percentage(U128::from(50)); // represents 0.5%

        assert_eq!(multisend_contract.get_percentage(), 50); // represents 0.5%
    }

    #[test]
    #[should_panic(expected = "This functionality is allowed to be called by admin or contract only")]
    fn test_change_percentage_fail() {
        // setting up wrong admin that are NOT allowed to change percentage
        let mut multisend_contract = Multisender::initialize(alice(), alice());

        testing_env!(get_context(false, alice(), bob()));
        assert_eq!(multisend_contract.get_percentage(), 10); // represents 0.1%

        multisend_contract.change_percentage(U128::from(50)); // represents 0.5%
    }
}

