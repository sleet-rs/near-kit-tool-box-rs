// rhea_dclv2_predict_add_liquidity (json)
//
// Simulates adding liquidity to a DCL v2 pool range via raw JSON args.
// No signer required — only `NEAR_NETWORK` is needed.
//
// run:
//   cargo run --bin rhea_dclv2_predict_add_liquidity_bin_json -- <pool_id> <left_point> <right_point> <amount_x> <amount_y> [dcl_id]
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_predict_add_liquidity_fun_json::predict_add_liquidity;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let pool_id = args.get(1).expect(
        "usage: rhea_dclv2_predict_add_liquidity_bin_json <pool_id> <left_point> <right_point> <amount_x> <amount_y> [dcl_id]",
    );
    let left_point: i32 = args.get(2).expect("missing <left_point>").parse().expect("left_point must be an i32");
    let right_point: i32 = args.get(3).expect("missing <right_point>").parse().expect("right_point must be an i32");
    let amount_x = args.get(4).expect("missing <amount_x>");
    let amount_y = args.get(5).expect("missing <amount_y>");
    let dcl_id = args.get(6).map(String::as_str).unwrap_or("dclv2.ref-labs.near");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Predicting add-liquidity on pool `{pool_id}` via `{dcl_id}`...");
    let out = predict_add_liquidity(&near, dcl_id, pool_id, left_point, right_point, amount_x, amount_y).await?;
    println!("{out:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
