// rhea_get_number_of_pools (json)
//
// Reads the total number of pools registered on the rhea / ref-finance
// DEX via raw JSON args. No signer required — only `NEAR_NETWORK` is
// needed (defaults to testnet).
//
// run:
//   cargo run --bin rhea_get_number_of_pools_bin_json -- [rhea_contract_id]
//
// example:
//   cargo run --bin rhea_get_number_of_pools_bin_json
//   cargo run --bin rhea_get_number_of_pools_bin_json -- v2.ref-finance.near
//   cargo run --bin rhea_get_number_of_pools_bin_json -- ref-finance-101.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea::rhea_get_number_of_pools_fun_json::get_number_of_pools;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let rhea_contract_id = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("ref-finance-101.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!(
        "Fetching number of pools from rhea contract `{}`...",
        rhea_contract_id
    );
    let number_of_pools = get_number_of_pools(&near, rhea_contract_id).await?;
    println!("Number of pools: {number_of_pools}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
