// rhea_dclv2_limit_order (json)
//
// Places a DCL v2 limit order via `ft_transfer_call` on the SELL token
// contract, using raw JSON args.
//
// DCL has no `place_order` method — orders enter through the sell token's
// `ft_transfer_call` -> DCL `ft_on_transfer`.
//
// set NEAR_KIT_NETWORK, NEAR_KIT_ACCOUNT_ID, NEAR_KIT_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin rhea_dclv2_limit_order_bin_json -- <sell_token> <amount> <pool_id> <buy_token> <point> [with_swap] [dcl_id]
//
//   <with_swap> is "true" to use the `LimitOrderWithSwap` variant (fills
//   what it can right away, then rests the remainder) and "false" for the
//   plain `LimitOrder` variant (rests entirely).
//
// example:
//   cargo run --bin rhea_dclv2_limit_order_bin_json -- wrap.testnet 100000000000000000000000 "usdc.fakes.testnet|wrap.testnet|2000" usdc.fakes.testnet 288800 false dclv2.ref-dev.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_limit_order_fun_json::limit_order;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let sell_token = args.get(1).expect(
        "usage: rhea_dclv2_limit_order_bin_json <sell_token> <amount> <pool_id> <buy_token> <point> [with_swap] [dcl_id]",
    );
    let amount = args.get(2).expect("amount required");
    let pool_id = args.get(3).expect("pool_id required");
    let buy_token = args.get(4).expect("buy_token required");
    let point: i64 = args
        .get(5)
        .expect("point required")
        .parse()
        .expect("point must be an i64");
    let with_swap = args
        .get(6)
        .map(|s| s == "true" || s == "1")
        .unwrap_or(false);
    let dcl_id = args
        .get(7)
        .map(String::as_str)
        .unwrap_or("dclv2.ref-dev.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!(
        "Placing limit order: sell `{amount}` of `{sell_token}` for `{buy_token}` at point `{point}` on pool `{pool_id}` via `{dcl_id}` (with_swap={with_swap})..."
    );
    let result = limit_order(
        &near, sell_token, amount, dcl_id, pool_id, buy_token, point, with_swap,
    )
    .await?;
    println!(
        "✅ limit order complete. tx id: {}",
        result.transaction.hash
    );
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
