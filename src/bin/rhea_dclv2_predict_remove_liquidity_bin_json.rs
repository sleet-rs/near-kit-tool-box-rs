// rhea_dclv2_predict_remove_liquidity (json)
//
// Simulates removing liquidity from a DCL v2 position via raw JSON args.
// No signer required — only `NEAR_KIT_NETWORK` is needed.
//
// set NEAR_KIT_NETWORK, NEAR_KIT_ACCOUNT_ID, NEAR_KIT_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin rhea_dclv2_predict_remove_liquidity_bin_json -- <lpt_id> <amount> [dcl_id]
//
// example:
//   cargo run --bin rhea_dclv2_predict_remove_liquidity_bin_json -- "usdc.fakes.testnet|wrap.testnet|2000#2544" 2698486080695 dclv2.ref-dev.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_predict_remove_liquidity_fun_json::predict_remove_liquidity;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let lpt_id = args
        .get(1)
        .expect("usage: rhea_dclv2_predict_remove_liquidity_bin_json <lpt_id> <amount> [dcl_id]");
    let amount = args.get(2).expect("amount required");
    let dcl_id = args
        .get(3)
        .map(String::as_str)
        .unwrap_or("dclv2.ref-dev.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Predicting removal of `{amount}` from `{lpt_id}` via `{dcl_id}`...");
    let out = predict_remove_liquidity(&near, dcl_id, lpt_id, amount).await?;
    println!("{out:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
