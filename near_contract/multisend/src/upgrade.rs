use crate::*;
use near_sdk::{Gas, PanicOnDefault};

// One Tera gas, which is 10^12 gas units.
pub const ONE_TERA: Gas = Gas(1_000_000_000_000);

pub trait Upgradable {
    /// function to migrate state with or without new field.
    /// make sure you are using the same method name in upgrade function
    fn migrate() -> Self;

    /// contract versioning
    fn get_version(&self) -> String;

    /// upgrade feature to be called on new deployed contract and read the state of previous contract
    /// using migrate function
    #[cfg(target_arch = "wasm32")]
    fn upgrade(self);
}


#[near_bindgen]
impl Upgradable for Multisender {
    #[init(ignore_state)]
    #[private]
    fn migrate() -> Self {
        #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
        /// Copy of the scheme before upgrade
        struct PreviousStorageSchema {
            /// fee that charges from sender ( by default 0.1% )
            percentage: u128,

            /// account we are sending money from
            bank: AccountId,

            /// Contract owner account (contract itself by default)
            pub admin: AccountId,
        }

        // read and parse previos storage
        let contract: PreviousStorageSchema =
            env::state_read().expect("Contract is not initialized");

        // initialize new storage from an old schema
        Multisender {
            percentage: contract.percentage,
            bank: contract.bank,
            admin: contract.admin,
        }
    }

    fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    #[cfg(target_arch = "wasm32")]
    fn upgrade(self) {
        const GAS_FOR_UPGRADE: u64 = 20 * ONE_TERA.0; //gas occupied by this fn

        //after upgrade we call *pub fn migrate()* on the NEW CODE
        let current_id = env::current_account_id();

        let migrate_method_name = "migrate".as_bytes().to_vec();
        let attached_gas = env::prepaid_gas().0 - env::used_gas().0 - GAS_FOR_UPGRADE;
        unsafe {
            // Load input (new contract code) into register 0
            near_sys::input(0);

            // prepare self-call promise
            let promise_id = near_sys::promise_batch_create(
                current_id.as_bytes().len() as _,
                current_id.as_bytes().as_ptr() as _,
            );

            //1st action, deploy/upgrade code (takes code from register 0)
            near_sys::promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);

            // 2nd action, schedule a call to "migrate()".
            // Will execute on the new code
            near_sys::promise_batch_action_function_call(
                promise_id,
                migrate_method_name.len() as _,
                migrate_method_name.as_ptr() as _,
                0 as _,
                0 as _,
                0 as _,
                attached_gas,
            );
        }
    }
}