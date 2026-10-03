// rhea_dclv2_get_order (json)
//
// Reads a single DCL v2 limit order by its order id via raw JSON args.
// No signer required — only `NEAR_NETWORK` is needed.
//
// run:
//   cargo run --bin rhea_dclv2_get_order_bin_json -- <order_id> [dcl_id]
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_get_order_fun_json::get_order;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let order_id = args
        .get(1)
        .expect("usage: rhea_dclv2_get_order_bin_json <order_id> [dcl_id]");
    let dcl_id = args.get(2).map(String::as_str).unwrap_or("dclv2.ref-labs.near");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Fetching order `{order_id}` via `{dcl_id}`...");
    let out = get_order(&near, dcl_id, order_id).await?;
    println!("{out:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
