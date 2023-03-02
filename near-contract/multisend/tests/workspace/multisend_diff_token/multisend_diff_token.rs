use crate::utils::*;
use multisend::WBalance;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use workspaces::network::Sandbox;
use workspaces::{Account, Worker};

const DECIMALS: u8 = 24;

async fn multisend_diff_token_fixture(
    bank: &Account,
    owner: &Account,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<(workspaces::Contract, workspaces::Contract), anyhow::Error> {
    let underlying_token = deploy_underlying(owner, worker, DECIMALS).await?;
    let multisend_near_contract = deploy_multisend_near(bank, owner, worker).await?;


    let bank_address: String = worker
        .view(multisend_near_contract.id(), "get_bank_address")
        .await?
        .json()?;

    let percentage: U128 = worker
        .view(multisend_near_contract.id(), "get_percentage")
        .await?
        .json()?;

    assert_eq!(bank_address, bank.id().to_string());
    assert_eq!(percentage, U128(10));

    Ok((multisend_near_contract, underlying_token))
}

#[tokio::test]
async fn test_successful_multisend_diff_token() -> anyhow::Result<()> {


    ////////////////////////////////////////////////////////////////////////////
    // Stage 1: Deploy contracts such as underlying token and multi send contract
    ////////////////////////////////////////////////////////////////////////////

    let worker = workspaces::sandbox().await?;
    let owner = worker.root_account()?;
    let account = worker.dev_create_account().await?;

    let bank = account
        .create_subaccount("bank")
        .transact()
        .await?
        .into_result()?;

    let alice = account
        .create_subaccount("alice")
        .transact()
        .await?
        .into_result()?;

    let bob = account
        .create_subaccount("bob")
        .transact()
        .await?
        .into_result()?;

    let (multisend_near_account, underlying) = multisend_diff_token_fixture(&bank, &owner, &worker).await?;

    ////////////////////////////////////////////////////////////////////////////////////////////
    // Stage 2: Deposit the storage for contract, each participant in multi transaction and
    // contract itself, mint some tokens to be able to operate
    ////////////////////////////////////////////////////////////////////////////////////////////

    let _ = underlying
        .call("storage_deposit")
        .args_json(json!({
            "account_id": bank.id()
        }))
        .max_gas()
        .deposit(25 * 10u128.pow(23))
        .transact()
        .await?;


    let _ = underlying
        .call("storage_deposit")
        .args_json(json!({
            "account_id": alice.id()
        }))
        .max_gas()
        .deposit(25 * 10u128.pow(23))
        .transact()
        .await?;

    let _ = underlying
        .call("storage_deposit")
        .args_json(json!({
            "account_id": bob.id()
        }))
        .max_gas()
        .deposit(25 * 10u128.pow(23))
        .transact()
        .await?;

    let _ = underlying
        .call("storage_deposit")
        .args_json(json!({
            "account_id": multisend_near_account.id()
        }))
        .max_gas()
        .deposit(25 * 10u128.pow(23))
        .transact()
        .await?;

    let _ = underlying
        .call("mint")
        .args_json(json!({
            "account_id": owner.id(),
            "amount": U128::from(2000000000000000000000000000)
        }))
        .max_gas()
        .transact()
        .await?;

    let bank_ft_balance_of_before: U128 = worker
        .view(
            underlying.id(),
            "ft_balance_of",
        )
        .await?
        .json()?;


    // let multisend_transaction = owner
    //     .call(
    //         multisend_near_account.id(),
    //         "multi_send_from_attached_deposit_near",
    //     )
    //     .args_json(json!({
    //         "recipients": [alice.id(), bob.id()],
    //         "amounts": [amount_to_transfer_to_alice, amount_to_transfer_to_bob],
    //     }))
    //     .max_gas()
    //     .deposit(1010000000000000000000000000)
    //     .transact()
    //     .await?;
    //
    // assert!(multisend_transaction.is_success());


    // assert_eq!(
    //     bank_balance_before_transfer.balance + taxes.0,
    //     bank_balance_after_transfer.balance
    // );
    // assert_eq!(
    //     alice_balance_before_transfer.balance + amount_to_transfer_to_alice.0,
    //     alice_balance_after_transfer.balance
    // );
    // assert_eq!(
    //     bob_balance_before_transfer.balance + amount_to_transfer_to_bob.0,
    //     bob_balance_after_transfer.balance
    // );

    Ok(())
}
