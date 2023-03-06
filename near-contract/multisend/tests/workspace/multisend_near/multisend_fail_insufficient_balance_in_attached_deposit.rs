use crate::utils::*;
use multisend::WBalance;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use workspaces::network::Sandbox;
use workspaces::{Account, Worker};

async fn multisend_fixture(
    bank: &Account,
    owner: &Account,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<workspaces::Contract, anyhow::Error> {
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

    Ok(multisend_near_contract)
}

#[tokio::test]
async fn test_multisend_fail_insufficient_balance_in_attached_deposit() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let owner = worker.root_account()?;
    let account = worker.dev_create_account().await?;

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

    let bank = account
        .create_subaccount("bank")
        .transact()
        .await?
        .into_result()?;

    let bank_balance_before = worker.view_account(bank.id()).await?;
    let alice_balance_before = worker.view_account(alice.id()).await?;
    let bob_balance_before = worker.view_account(bob.id()).await?;

    let amount_to_transfer_to_alice: WBalance = U128::from(100000000000000000000000000); // 100 NEAR
    let amount_to_transfer_to_bob: WBalance = U128::from(100000000000000000000000000); // 100 NEAR
    let _taxes: WBalance =
        U128::from((amount_to_transfer_to_alice.0 + amount_to_transfer_to_bob.0) * 10 / 1000);

    let multisend_near_account = multisend_fixture(&bank, &owner, &worker).await?;

    let multisend_transaction = owner
        .call(
            multisend_near_account.id(),
            "multi_send_from_attached_deposit_near",
        )
        .args_json(json!({
            "recipients": [alice.id(), bob.id()],
            "amounts": [amount_to_transfer_to_alice, amount_to_transfer_to_bob],
        }))
        .max_gas()
        .deposit(10100000000000) // insufficient amount for attached deposit
        .transact()
        .await?;

    assert!(multisend_transaction.is_failure());

    let bank_balance_after = worker.view_account(bank.id()).await?;
    let alice_balance_after = worker.view_account(alice.id()).await?;
    let bob_balance_after = worker.view_account(bob.id()).await?;

    assert_eq!(bank_balance_before.balance, bank_balance_after.balance);
    assert_eq!(alice_balance_before.balance, alice_balance_after.balance);
    assert_eq!(bob_balance_before.balance, bob_balance_after.balance);

    Ok(())
}
