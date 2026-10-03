// dclv2_remove_liquidity (json)
//
// Removes concentrated liquidity from a DCL v2 position via raw JSON args.
// Funds land in internal balance; withdraw via dclv2_withdraw_asset_bin_json.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin dclv2_remove_liquidity_bin_json -- <lpt_id> <amount> [dcl_id]
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::dclv2::dclv2_remove_liquidity_fun_json::remove_liquidity;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let lpt_id = args.get(1).expect("usage: dclv2_remove_liquidity_bin_json <lpt_id> <amount> [dcl_id]");
    let amount = args.get(2).expect("usage: dclv2_remove_liquidity_bin_json <lpt_id> <amount> [dcl_id]");
    let dcl_id = args.get(3).map(String::as_str).unwrap_or("dclv2.ref-dev.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Removing liquidity `{amount}` from `{lpt_id}` via `{dcl_id}`...");
    let result = remove_liquidity(&near, dcl_id, lpt_id, amount, None, None).await?;
    println!("✅ remove_liquidity complete. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
