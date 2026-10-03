// rhea_dclv2_storage_deposit (json)
//
// Registers storage on the DCL v2 contract for the signer. Attaches 0.5 NEAR
// (the amount that actually satisfies the first liquidity/order slot — the
// bare storage_balance_bounds minimum is rejected with E102).
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin rhea_dclv2_storage_deposit_bin_json -- [dcl_id]
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_storage_deposit_fun_json::storage_deposit;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let dcl_id = args.get(1).map(String::as_str).unwrap_or("dclv2.ref-dev.testnet");
    let near = NEAR_KIT_CLIENT::from_env()?;
    println!("Registering storage on `{dcl_id}`...");
    let result = storage_deposit(&near, dcl_id).await?;
    println!("✅ storage_deposit complete. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near