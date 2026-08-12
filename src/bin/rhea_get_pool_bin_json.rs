// rhea_get_pool (json)
//
// Reads a single pool by its pool id from the rhea / ref-finance DEX
// via raw JSON args. No signer required — only `NEAR_NETWORK` is
// needed (defaults to testnet).
//
// run:
//   cargo run --bin rhea_get_pool_bin_json -- <pool_id> [rhea_contract_id]
//
// example:
//   cargo run --bin rhea_get_pool_bin_json -- 0
//   cargo run --bin rhea_get_pool_bin_json -- 0 v2.ref-finance.near
//   cargo run --bin rhea_get_pool_bin_json -- 0 ref-finance-101.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea::rhea_get_pool_fun_json::get_pool;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let pool_id: u32 = args
        .get(1)
        .expect("usage: rhea_get_pool_bin_json <pool_id> [rhea_contract_id]")
        .parse()
        .expect("pool_id must be a u32");
    let rhea_contract_id = args
        .get(2)
        .map(String::as_str)
        .unwrap_or("ref-finance-101.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!(
        "Fetching pool `{}` from rhea contract `{}`...",
        pool_id, rhea_contract_id
    );
    let pool = get_pool(&near, rhea_contract_id, pool_id).await?;
    println!("{pool:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
