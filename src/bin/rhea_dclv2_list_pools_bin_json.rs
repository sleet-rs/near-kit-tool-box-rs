// rhea_dclv2_list_pools (json)
//
// Reads a paginated slice of DCL v2 pools via raw JSON args. No signer
// required — only `NEAR_NETWORK` is needed (defaults to testnet).
//
// run:
//   cargo run --bin rhea_dclv2_list_pools_bin_json -- <from_index> <limit> [dcl_id]
//
// example:
//   cargo run --bin rhea_dclv2_list_pools_bin_json -- 0 5
//   cargo run --bin rhea_dclv2_list_pools_bin_json -- 0 5 dclv2.ref-labs.near
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_list_pools_fun_json::list_pools;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let from_index: u64 = args
        .get(1)
        .expect("usage: rhea_dclv2_list_pools_bin_json <from_index> <limit> [dcl_id]")
        .parse()
        .expect("from_index must be a u64");
    let limit: u64 = args
        .get(2)
        .expect("usage: rhea_dclv2_list_pools_bin_json <from_index> <limit> [dcl_id]")
        .parse()
        .expect("limit must be a u64");
    let dcl_id = args.get(3).map(String::as_str).unwrap_or("dclv2.ref-labs.near");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Fetching pools [{from_index}, {}) from DCL v2 contract `{dcl_id}`...", from_index + limit);
    let pools = list_pools(&near, dcl_id, from_index, limit).await?;
    println!("{pools:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
