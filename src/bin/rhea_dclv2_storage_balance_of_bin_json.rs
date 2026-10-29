// rhea_dclv2_storage_balance_of (json)
//
// Reads the NEP-145 storage balance an account holds on the DCL v2
// contract via raw JSON args. No signer required.
//
// run:
//   cargo run --bin rhea_dclv2_storage_balance_of_bin_json -- <account_id> [dcl_id]
//
// example:
//   cargo run --bin rhea_dclv2_storage_balance_of_bin_json -- sleet.near
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_storage_balance_of_fun_json::storage_balance_of;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_id = args
        .get(1)
        .expect("usage: rhea_dclv2_storage_balance_of_bin_json <account_id> [dcl_id]");
    let dcl_id = args
        .get(2)
        .map(String::as_str)
        .unwrap_or("dclv2.ref-labs.near");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Fetching storage balance of `{account_id}` via `{dcl_id}`...");
    let balance = storage_balance_of(&near, dcl_id, account_id).await?;
    println!("{balance:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
