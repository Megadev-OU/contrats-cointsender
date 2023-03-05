use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::{serde_json, AccountId};
use workspaces::network::Sandbox;
use workspaces::{Account, Worker};

const MULTISEND_WASM: &str = "../target/wasm32-unknown-unknown/release/multisend.wasm";
const UNDERLYING_WASM: &str = "../target/wasm32-unknown-unknown/release/mock_token.wasm";

pub async fn deploy_multisend_near(
    bank: &Account,
    owner: &Account,
    worker: &Worker<Sandbox>,
) -> Result<workspaces::Contract, workspaces::error::Error> {
    let wasm = std::fs::read(MULTISEND_WASM);
    let multisend_near_contract = worker.dev_deploy(&wasm.unwrap()).await?;

    let _ = multisend_near_contract
        .call("initialize")
        .args_json(json!({
        "bank": bank.id(),
        "admin": owner.id()
        }))
        .max_gas()
        .transact()
        .await?;

    Ok(multisend_near_contract)
}

pub async fn deploy_underlying(
    owner: &Account,
    worker: &Worker<Sandbox>,
    decimals: u8,
) -> Result<workspaces::Contract, workspaces::error::Error> {
    let wasm = std::fs::read(UNDERLYING_WASM);
    let underlying = worker.dev_deploy(&wasm.unwrap()).await?;

    let _ = underlying
        .call("new_default_meta")
        .args_json(json!({
            "owner_id": owner.id(),
            "name": "Wrapped Ethereum",
            "symbol": "WETH",
            "total_supply": "1000000000000000000000000000",
            "decimals": decimals,
        }))
        .max_gas()
        .transact()
        .await?;

    Ok(underlying)
}
