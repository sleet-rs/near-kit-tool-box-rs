// rhea_ammv2_get_deposits (json)
//
// Reads every LP position (per pool) that an account holds on the
// rhea_ammv2 / ref-finance DEX via raw JSON args. No signer required —
// only `NEAR_NETWORK` is needed (defaults to testnet).
//
// run:
//   cargo run --bin rhea_ammv2_get_deposits_bin_json -- <account_id> [rhea_ammv2_contract_id]
//
// example:
//   cargo run --bin rhea_ammv2_get_deposits_bin_json -- sleet.testnet
//   cargo run --bin rhea_ammv2_get_deposits_bin_json -- sleet.near v2.ref-finance.near
//   cargo run --bin rhea_ammv2_get_deposits_bin_json -- sleet.testnet ref-finance-101.testnet
//
// =================================================
use near_kit::{AccountId, Error};
use near_kit_tool_box::fun::rhea_ammv2::rhea_ammv2_get_deposits_fun_json::get_deposits;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_id_raw = args
        .get(1)
        .expect("usage: rhea_ammv2_get_deposits_bin_json <account_id> [rhea_ammv2_contract_id]");
    let rhea_ammv2_contract_id = args
        .get(2)
        .map(String::as_str)
        .unwrap_or("ref-finance-101.testnet");

    let account_id: AccountId = account_id_raw.parse()?;
    let near = NEAR_KIT_CLIENT::from_env()?;

    println!(
        "Fetching deposits for `{}` on rhea_ammv2 contract `{}`...",
        account_id, rhea_ammv2_contract_id
    );
    let deposits = get_deposits(&near, rhea_ammv2_contract_id, &account_id).await?;
    println!("{deposits:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
