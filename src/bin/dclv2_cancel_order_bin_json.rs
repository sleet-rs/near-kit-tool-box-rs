// dclv2_cancel_order (json)
//
// Cancels a DCL v2 limit order via raw JSON args. Omit amount for full
// cancel; pass "0" to claim fills without cancelling.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin dclv2_cancel_order_bin_json -- <order_id> [amount] [dcl_id]
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::dclv2::dclv2_cancel_order_fun_json::cancel_order;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let order_id = args.get(1).expect("usage: dclv2_cancel_order_bin_json <order_id> [amount] [dcl_id]");
    let amount = args.get(2).map(String::as_str);
    let dcl_id = args
        .get(3)
        .map(String::as_str)
        .unwrap_or("dclv2.ref-dev.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Cancelling order `{order_id}` via `{dcl_id}`...");
    let result = cancel_order(&near, dcl_id, order_id, amount).await?;
    println!("✅ cancel_order complete. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
