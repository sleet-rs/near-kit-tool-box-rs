// rhea_dclv2_get_liquidity (json)
//
// Reads a single DCL v2 liquidity position by its lpt id via raw JSON
// args. No signer required — only `NEAR_KIT_NETWORK` is needed.
//
// The answer carries the position's `amount` (the liquidity amount), its
// `left_point` / `right_point`, and its `unclaimed_fee_x` /
// `unclaimed_fee_y` — the fees that can be claimed.
//
// run:
//   cargo run --bin rhea_dclv2_get_liquidity_bin_json -- <lpt_id> [dcl_id]
//
// example:
//   cargo run --bin rhea_dclv2_get_liquidity_bin_json -- "usdc.fakes.testnet|wrap.testnet|2000#2544" dclv2.ref-dev.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_get_liquidity_fun_json::get_liquidity;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let lpt_id = args
        .get(1)
        .expect("usage: rhea_dclv2_get_liquidity_bin_json <lpt_id> [dcl_id]");
    let dcl_id = args
        .get(2)
        .map(String::as_str)
        .unwrap_or("dclv2.ref-dev.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Fetching liquidity `{lpt_id}` via `{dcl_id}`...");
    let out = get_liquidity(&near, dcl_id, lpt_id).await?;
    println!("{out:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
