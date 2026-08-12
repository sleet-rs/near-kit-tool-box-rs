// rhea_storage_balance_of (json)
//
// Reads the NEP-145 storage balance an account holds on the rhea /
// ref-finance DEX contract via raw JSON args. No signer required —
// only `NEAR_NETWORK` is needed (defaults to testnet).
//
// Returns `null` when the account has no storage registration; an
// object with `total`, `available`, `used` otherwise.
//
// run:
//   cargo run --bin rhea_storage_balance_of_bin_json -- <account_id> [rhea_contract_id]
//
// example:
//   cargo run --bin rhea_storage_balance_of_bin_json -- sleet.testnet
//   cargo run --bin rhea_storage_balance_of_bin_json -- sleet.near v2.ref-finance.near
//   cargo run --bin rhea_storage_balance_of_bin_json -- sleet.testnet ref-finance-101.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea::rhea_storage_balance_of_fun_json::storage_balance_of;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_id = args
        .get(1)
        .expect("usage: rhea_storage_balance_of_bin_json <account_id> [rhea_contract_id]");
    let rhea_contract_id = args
        .get(2)
        .map(String::as_str)
        .unwrap_or("ref-finance-101.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?.build();

    println!(
        "Fetching storage balance of `{}` on rhea contract `{}`...",
        account_id, rhea_contract_id
    );
    let balance = storage_balance_of(&near, rhea_contract_id, account_id).await?;
    println!("{balance:#}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
