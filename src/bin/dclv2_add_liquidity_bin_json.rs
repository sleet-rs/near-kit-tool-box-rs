// dclv2_add_liquidity (json)
//
// Adds concentrated liquidity to a DCL v2 pool range via raw JSON args.
// Requires internal balances (fund via dclv2_deposit_bin_json first) and
// DCL storage registered.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin dclv2_add_liquidity_bin_json -- <pool_id> <left_point> <right_point> <amount_x> <amount_y> [dcl_id]
//
// example (single-sided above price, only X consumed):
//   cargo run --bin dclv2_add_liquidity_bin_json -- "usdc.fakes.testnet|wrap.testnet|2000" 288700 289100 1000000 0 dclv2.ref-dev.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::dclv2::dclv2_add_liquidity_fun_json::add_liquidity;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let pool_id = args.get(1).expect(
        "usage: dclv2_add_liquidity_bin_json <pool_id> <left_point> <right_point> <amount_x> <amount_y> [dcl_id]",
    );
    let left_point: i32 = args
        .get(2)
        .expect("left_point required")
        .parse()
        .expect("left_point must be i32");
    let right_point: i32 = args
        .get(3)
        .expect("right_point required")
        .parse()
        .expect("right_point must be i32");
    let amount_x = args.get(4).expect("amount_x required");
    let amount_y = args.get(5).expect("amount_y required");
    let dcl_id = args.get(6).map(String::as_str).unwrap_or("dclv2.ref-dev.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Adding liquidity to `{pool_id}` [{left_point},{right_point}] x={amount_x} y={amount_y} via `{dcl_id}`...");
    let result = add_liquidity(&near, dcl_id, pool_id, left_point, right_point, amount_x, amount_y, None, None).await?;
    println!("✅ add_liquidity complete. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
