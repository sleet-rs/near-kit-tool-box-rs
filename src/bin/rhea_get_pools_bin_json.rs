// rhea_get_pools (json)
//
// Reads a paginated slice of pools from the rhea / ref-finance DEX via
// raw JSON args. No signer required — only `NEAR_NETWORK` is needed
// (defaults to testnet).
//
// run:
//   cargo run --bin rhea_get_pools_bin_json -- <from_index> <limit> [rhea_contract_id]
//
// example:
//   cargo run --bin rhea_get_pools_bin_json -- 0 3
//   cargo run --bin rhea_get_pools_bin_json -- 0 3 v2.ref-finance.near
//   cargo run --bin rhea_get_pools_bin_json -- 0 3 ref-finance-101.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea::rhea_get_pools_fun_json::get_pools;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let from_index: u64 = args
        .get(1)
        .expect("usage: rhea_get_pools_bin_json <from_index> <limit> [rhea_contract_id]")
        .parse()
        .expect("from_index must be a u64");
    let limit: u64 = args
        .get(2)
        .expect("usage: rhea_get_pools_bin_json <from_index> <limit> [rhea_contract_id]")
        .parse()
        .expect("limit must be a u64");
    let rhea_contract_id = args
        .get(3)
        .map(String::as_str)
        .unwrap_or("ref-finance-101.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?.build();

    println!(
        "Fetching pools [{}, {}) from rhea contract `{}`...",
        from_index,
        from_index + limit,
        rhea_contract_id
    );
    let pools = get_pools(&near, rhea_contract_id, from_index, limit).await?;
    println!("{pools:#}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
