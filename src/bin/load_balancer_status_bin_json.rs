// load_balancer_status (json)
//
// Reads `status` from the load-balanced view client across the default
// public testnet endpoints. Each call picks the next endpoint in
// round-robin order, with failover on retryable errors.
//
// run:
//   cargo run --bin load_balancer_status_bin_json
//
// example:
//   cargo run --bin load_balancer_status_bin_json
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let view = NEAR_KIT_CLIENT::view_testnet();

    println!("==========================");
    println!("Load-balanced view client (testnet):");
    println!("  Endpoints ({}):", view.len());
    for url in view.urls() {
        println!("    - {url}");
    }
    println!("==========================");

    println!("\nFetching node status 3 times (round-robin)...");

    for i in 1..=3 {
        let status = view.status().await?;
        println!(
            "[{i}] chain_id={} latest_block_height={} served_by={}",
            status.chain_id,
            status.sync_info.latest_block_height,
            view.last_used_url().unwrap_or("?"),
        );
    }

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
