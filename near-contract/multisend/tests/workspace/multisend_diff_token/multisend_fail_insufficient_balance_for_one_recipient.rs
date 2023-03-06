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
async fn test_multisend_fail_insufficient_balance_for_one_recipient() -> anyhow::Result<()> {
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

    let (multisend_near_account, underlying) =
        multisend_diff_token_fixture(&bank, &owner, &worker).await?;

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
            "account_id": underlying.id()
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
            "amount": U128::from(20000000000000000000000000000000)
        }))
        .max_gas()
        .transact()
        .await?;
    let _ = underlying
        .call("mint")
        .args_json(json!({
            "account_id": bank.id(),
            "amount": U128::from(2000000000000000000000000000)
        }))
        .max_gas()
        .transact()
        .await?;
    let _ = underlying
        .call("mint")
        .args_json(json!({
            "account_id": alice.id(),
            "amount": U128::from(2000000000000000000000000000)
        }))
        .max_gas()
        .transact()
        .await?;
    let _ = underlying
        .call("mint")
        .args_json(json!({
            "account_id": bob.id(),
            "amount": U128::from(2000000000000000000000000000)
        }))
        .max_gas()
        .transact()
        .await?;

    let _ = underlying
        .call("mint")
        .args_json(json!({
            "account_id": underlying.id(),
            "amount": U128::from(20000000000000000000000000000000)
        }))
        .max_gas()
        .transact()
        .await?;

    let bank_ft_balance_of_before: U128 = worker
        .view(underlying.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": bank.id(),
        }))
        .await?
        .json()?;

    let alice_ft_balance_of_before: U128 = worker
        .view(underlying.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": alice.id(),
        }))
        .await?
        .json()?;

    let bob_ft_balance_of_before: U128 = worker
        .view(underlying.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": bob.id(),
        }))
        .await?
        .json()?;

    let amount_to_transfer_to_alice: WBalance = U128::from(100000000000000000000000000000); // 100000 tokens
    let amount_to_transfer_to_bob: WBalance = U128::from(10000000000000000000000000); // 10 tokens
    let _taxes: WBalance =
        U128::from((amount_to_transfer_to_alice.0 + amount_to_transfer_to_bob.0) * 10 / 1000);

    let msg = json!(
    {
      "token_id": underlying.id(),
      "transfers": [
        {"recipient": alice.id(),
         "amount": amount_to_transfer_to_alice.0.to_string()},
        {"recipient": bob.id(),
         "amount": amount_to_transfer_to_bob.0.to_string()}
    ]
    })
    .to_string();

    let _multisend_transaction = owner
        .call(underlying.id(), "ft_transfer_call")
        .args_json(json!({
            "receiver_id": multisend_near_account.id(),
            "amount": U128::from(20200000000000000000000000),
            "msg": msg,
        }))
        .max_gas()
        .deposit(1)
        .transact()
        .await?;

    let bank_ft_balance_of_after: U128 = worker
        .view(underlying.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": bank.id(),
        }))
        .await?
        .json()?;

    let alice_ft_balance_of_after: U128 = worker
        .view(underlying.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": alice.id(),
        }))
        .await?
        .json()?;

    let bob_ft_balance_of_after: U128 = worker
        .view(underlying.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": bob.id(),
        }))
        .await?
        .json()?;

    assert_eq!(bank_ft_balance_of_before.0, bank_ft_balance_of_after.0);
    assert_eq!(alice_ft_balance_of_before.0, alice_ft_balance_of_after.0);
    assert_eq!(bob_ft_balance_of_before.0, bob_ft_balance_of_after.0);

    Ok(())
}
