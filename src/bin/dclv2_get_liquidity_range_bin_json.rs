// dclv2_get_liquidity_range (json)
//
// Reads the per-point liquidity distribution of a DCL v2 pool range via
// raw JSON args. No signer required — only `NEAR_NETWORK` is needed.
//
// run:
//   cargo run --bin dclv2_get_liquidity_range_bin_json -- <pool_id> <left_point> <right_point> [dcl_id]
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::dclv2::dclv2_get_liquidity_range_fun_json::get_liquidity_range;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let pool_id = args
        .get(1)
        .expect("usage: dclv2_get_liquidity_range_bin_json <pool_id> <left_point> <right_point> [dcl_id]");
    let left_point: i32 = args.get(2).expect("missing <left_point>").parse().expect("left_point must be an i32");
    let right_point: i32 = args.get(3).expect("missing <right_point>").parse().expect("right_point must be an i32");
    let dcl_id = args.get(4).map(String::as_str).unwrap_or("dclv2.ref-labs.near");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Fetching liquidity range [{left_point}, {right_point}] of `{pool_id}` via `{dcl_id}`...");
    let out = get_liquidity_range(&near, dcl_id, pool_id, left_point, right_point).await?;
    println!("{out:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
