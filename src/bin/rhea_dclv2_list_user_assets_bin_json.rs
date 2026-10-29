// rhea_dclv2_list_user_assets (json)
//
// Lists every inner token balance an account holds inside the DCL v2
// contract via raw JSON args. No signer required.
//
// Returns a map of token id to balance, each balance a decimal string in
// the token's smallest unit.
//
// set NEAR_KIT_NETWORK, NEAR_KIT_ACCOUNT_ID, NEAR_KIT_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin rhea_dclv2_list_user_assets_bin_json -- <account_id> [dcl_id]
//
// example:
//   cargo run --bin rhea_dclv2_list_user_assets_bin_json -- sleet.testnet dclv2.ref-dev.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_list_user_assets_fun_json::list_user_assets;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_id = args
        .get(1)
        .expect("usage: rhea_dclv2_list_user_assets_bin_json <account_id> [dcl_id]");
    let dcl_id = args
        .get(2)
        .map(String::as_str)
        .unwrap_or("dclv2.ref-dev.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Fetching inner balances of `{account_id}` via `{dcl_id}`...");
    let out = list_user_assets(&near, dcl_id, account_id).await?;
    println!("{out:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
