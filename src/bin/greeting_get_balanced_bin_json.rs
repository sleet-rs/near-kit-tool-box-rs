// greeting_get_balanced (json)
//
// Calls `get_greeting` a few times via the load-balanced view client and
// logs the greeting + the RPC endpoint that actually served the call. The
// greeting is fetched through `greeting_get_fun_json::get_greeting` — each
// iteration picks a fresh `Near` from the balancer with `next_near()` and
// hands it to the reusable view helper.
//
// run:
//   cargo run --bin greeting_get_balanced_bin_json -- <contract_id> [times]
//
// example:
//   cargo run --bin greeting_get_balanced_bin_json -- hello.sleet.testnet 5
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::greeting::greeting_get_fun_json::get_greeting;
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

    let view = NEAR_KIT_CLIENT::view_balancer_testnet();

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
        let near = view.next_near();
        let greeting = get_greeting(&near, contract_id).await?;
        println!(
            "[{i:>2}/{times}] greeting={greeting:?} served_by={}",
            view.last_used_url().unwrap_or("?"),
        );
    }

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near