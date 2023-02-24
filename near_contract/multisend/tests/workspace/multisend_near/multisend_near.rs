use near_sdk::AccountId;
use crate::utils::*;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use workspaces::network::Sandbox;
use workspaces::{Account, Worker};


async fn multisend_fixture(
    bank: &AccountId,
    owner: &Account,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<
    workspaces::Contract
    ,
    anyhow::Error,
> {
    ////////////////////////////////////////////////////////////////////////////
    // Stage 1: Deploy contract
    ////////////////////////////////////////////////////////////////////////////

    let multisend_near_contract = deploy_multisend_near(bank, owner, worker).await?;

    let bank_address: String = worker
        .view(
            multisend_near_contract.id(),
            "get_bank_address",
        )
        .await?
        .json()?;

    let percentage: U128 = worker
        .view(
            multisend_near_contract.id(),
            "get_percentage",
        )
        .await?
        .json()?;

    assert_eq!(bank_address, bank.to_string());
    assert_eq!(percentage, U128(10));

    Ok(multisend_near_contract)
}


#[tokio::test]
async fn test_successful_multisend_near() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let owner = worker.root_account()?;
    let bank: AccountId = "some.bank.account.near".parse().unwrap();
    let multisend_near_account = multisend_fixture(&bank, &owner, &worker).await?;

    Ok(())
}