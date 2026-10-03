// rhea_dclv2_list_active_orders (json)
//
// Lists the still-open DCL v2 limit orders of an account via raw JSON
// args. No signer required — only `NEAR_NETWORK` is needed.
//
// run:
//   cargo run --bin rhea_dclv2_list_active_orders_bin_json -- <account_id> [dcl_id]
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_list_active_orders_fun_json::list_active_orders;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_id = args
        .get(1)
        .expect("usage: rhea_dclv2_list_active_orders_bin_json <account_id> [dcl_id]");
    let dcl_id = args.get(2).map(String::as_str).unwrap_or("dclv2.ref-labs.near");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Fetching active orders of `{account_id}` via `{dcl_id}`...");
    let out = list_active_orders(&near, dcl_id, account_id).await?;
    println!("{out:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
