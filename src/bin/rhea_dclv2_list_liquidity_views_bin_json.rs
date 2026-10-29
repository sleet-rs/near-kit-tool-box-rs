// rhea_dclv2_list_liquidity_views (json)
//
// Lists the enriched liquidity views of an account via raw JSON args. No
// signer required — only `NEAR_KIT_NETWORK` is needed.
//
// The answers carry `token_x` / `token_y` (the current token amount on
// each side) and `fee_x` / `fee_y` (the fees accrued on each side), which
// the plain `list_liquidities` view does not answer with.
//
// run:
//   cargo run --bin rhea_dclv2_list_liquidity_views_bin_json -- <account_id> <from_index> <limit> [dcl_id]
//
// example:
//   cargo run --bin rhea_dclv2_list_liquidity_views_bin_json -- sleet.testnet 0 10 dclv2.ref-dev.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_list_liquidity_views_fun_json::list_liquidity_views;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_id = args.get(1).expect(
        "usage: rhea_dclv2_list_liquidity_views_bin_json <account_id> <from_index> <limit> [dcl_id]",
    );
    let from_index: u64 = args
        .get(2)
        .expect("from_index required")
        .parse()
        .expect("from_index must be a u64");
    let limit: u64 = args
        .get(3)
        .expect("limit required")
        .parse()
        .expect("limit must be a u64");
    let dcl_id = args
        .get(4)
        .map(String::as_str)
        .unwrap_or("dclv2.ref-dev.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Fetching liquidity views of `{account_id}` via `{dcl_id}`...");
    let out = list_liquidity_views(&near, dcl_id, account_id, from_index, limit).await?;
    println!("{out:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
