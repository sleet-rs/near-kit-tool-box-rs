// rhea_dclv2_storage_withdraw (json)
//
// Withdraws excess storage NEAR from the DCL v2 contract. Attaches 1 yocto.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin rhea_dclv2_storage_withdraw_bin_json -- [dcl_id]
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_storage_withdraw_fun_json::storage_withdraw;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let dcl_id = args.get(1).map(String::as_str).unwrap_or("dclv2.ref-dev.testnet");
    let near = NEAR_KIT_CLIENT::from_env()?;
    println!("Withdrawing storage NEAR from `{dcl_id}`...");
    let result = storage_withdraw(&near, dcl_id).await?;
    println!("✅ storage_withdraw complete. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near