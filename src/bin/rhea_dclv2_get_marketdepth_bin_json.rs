// rhea_dclv2_get_marketdepth (json)
//
// Reads the market-depth snapshot around the current price of a DCL v2
// pool via raw JSON args. No signer required.
//
// The response holds the pool's `liquidities` map (keyed by point) and its
// `orders` map (keyed by point) - the live limit order points of the pool.
//
// set NEAR_KIT_NETWORK, NEAR_KIT_ACCOUNT_ID, NEAR_KIT_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin rhea_dclv2_get_marketdepth_bin_json -- <pool_id> <depth> [dcl_id]
//
// example:
//   cargo run --bin rhea_dclv2_get_marketdepth_bin_json -- "usdc.fakes.testnet|wrap.testnet|2000" 20 dclv2.ref-dev.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_get_marketdepth_fun_json::get_marketdepth;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let pool_id = args
        .get(1)
        .expect("usage: rhea_dclv2_get_marketdepth_bin_json <pool_id> <depth> [dcl_id]");
    let depth: u64 = args
        .get(2)
        .expect("depth required")
        .parse()
        .expect("depth must be a u64");
    let dcl_id = args
        .get(3)
        .map(String::as_str)
        .unwrap_or("dclv2.ref-dev.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Fetching market depth of `{pool_id}` (depth {depth}) via `{dcl_id}`...");
    let out = get_marketdepth(&near, dcl_id, pool_id, depth).await?;
    println!("{out:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
