// dclv2_storage_unregister (json)
//
// Unregisters storage on the DCL v2 contract, returning locked NEAR.
// Attaches 1 yocto.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin dclv2_storage_unregister_bin_json -- [dcl_id]
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::dclv2::dclv2_storage_unregister_fun_json::storage_unregister;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let dcl_id = args.get(1).map(String::as_str).unwrap_or("dclv2.ref-dev.testnet");
    let near = NEAR_KIT_CLIENT::from_env()?;
    println!("Unregistering storage on `{dcl_id}`...");
    let result = storage_unregister(&near, dcl_id).await?;
    println!("✅ storage_unregister complete. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near