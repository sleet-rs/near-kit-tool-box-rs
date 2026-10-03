// rhea_dclv2_list_liquidities (json)
//
// Lists the DCL v2 liquidity positions of an account via raw JSON args.
// No signer required — only `NEAR_NETWORK` is needed.
//
// run:
//   cargo run --bin rhea_dclv2_list_liquidities_bin_json -- <account_id> <from_index> <limit> [dcl_id]
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_list_liquidities_fun_json::list_liquidities;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_id = args
        .get(1)
        .expect("usage: rhea_dclv2_list_liquidities_bin_json <account_id> <from_index> <limit> [dcl_id]");
    let from_index: u64 = args.get(2).expect("missing <from_index>").parse().expect("from_index must be a u64");
    let limit: u64 = args.get(3).expect("missing <limit>").parse().expect("limit must be a u64");
    let dcl_id = args.get(4).map(String::as_str).unwrap_or("dclv2.ref-labs.near");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Fetching liquidities of `{account_id}` via `{dcl_id}`...");
    let out = list_liquidities(&near, dcl_id, account_id, from_index, limit).await?;
    println!("{out:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
