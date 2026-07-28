// greeting_get_balanced (json)
//
// Calls `get_greeting` a few times via the load-balanced view client and
// logs the greeting + the RPC endpoint that actually served the call. Useful
// for eyeballing that round-robin picks are actually spreading across nodes
// and to spot which endpoint answered when failover kicks in.
//
// run:
//   cargo run --bin greeting_get_balanced_bin_json -- <contract_id> [times]
//
// example:
//   cargo run --bin greeting_get_balanced_bin_json -- hello.sleet.testnet 5
//
// =================================================
use near_kit::{Error, Finality, types::TryIntoAccountId};
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let contract_id = args
        .get(1)
        .expect("usage: greeting_get_balanced_bin_json <contract_id> [times]");
    let times: usize = args
        .get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);

    let view = NEAR_KIT_CLIENT::view_testnet();
    let account_id = contract_id
        .try_into_account_id()
        .expect("invalid contract_id");

    println!("==========================");
    println!("Load-balanced view client (testnet):");
    println!("  Contract: {contract_id}");
    println!("  Calls:    {times}");
    println!("  Endpoints ({}):", view.len());
    for url in view.urls() {
        println!("    - {url}");
    }
    println!("==========================");

    for i in 1..=times {
        let result = view
            .view_function(
                &account_id,
                "get_greeting",
                b"{}",
                Finality::Final.into(),
            )
            .await?;
        let greeting: String = serde_json::from_slice(&result.result)?;
        println!(
            "[{i:>2}/{times}] greeting={greeting:?} block_height={} served_by={}",
            result.block_height,
            view.last_used_url().unwrap_or("?"),
        );
    }

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
