// dclv2_get_pool (json)
//
// Reads a single DCL v2 pool by its pool id via raw JSON args. No signer
// required — only `NEAR_NETWORK` is needed (defaults to testnet).
//
// run:
//   cargo run --bin dclv2_get_pool_bin_json -- <pool_id> [dcl_id]
//
// example:
//   cargo run --bin dclv2_get_pool_bin_json -- "wrap.near|usdc.near|2000"
//   cargo run --bin dclv2_get_pool_bin_json -- "wrap.near|usdc.near|2000" dclv2.ref-labs.near
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::dclv2::dclv2_get_pool_fun_json::get_pool;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let pool_id = args
        .get(1)
        .expect("usage: dclv2_get_pool_bin_json <pool_id> [dcl_id]");
    let dcl_id = args.get(2).map(String::as_str).unwrap_or("dclv2.ref-labs.near");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Fetching pool `{pool_id}` from DCL v2 contract `{dcl_id}`...");
    let pool = get_pool(&near, dcl_id, pool_id).await?;
    println!("{pool:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
