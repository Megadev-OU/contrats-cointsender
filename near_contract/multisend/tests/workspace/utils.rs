use near_sdk::json_types::U128;
use near_sdk::{AccountId, serde_json};
use near_sdk::serde_json::json;
use workspaces::network::Sandbox;
use workspaces::{Account, Worker};

const MULTISEND_NEAR: &str = "target/wasm32-unknown-unknown/release/multisend.wasm";


pub async fn deploy_multisend_near(
    bank: &AccountId,
    owner: &Account,
    worker: &Worker<Sandbox>,
) -> Result<workspaces::Contract, workspaces::error::Error> {
    let wasm = std::fs::read(MULTISEND_NEAR);
    let multisend_near_contract = worker.dev_deploy(&wasm.unwrap()).await?;


    let _ = multisend_near_contract
        .call("initialize")
        .args_json(json!({
        "bank": bank,
        "admin": owner.id()
        }))
        .max_gas()
        .transact()
        .await?;

    Ok(multisend_near_contract)
}