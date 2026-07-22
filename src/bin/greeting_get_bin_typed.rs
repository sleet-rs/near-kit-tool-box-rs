// get_greeting
//
// Reads `get_greeting` via the typed contract wrapper. No signer required —
// only `NEAR_NETWORK` is needed (defaults to testnet).
//
// run:
//   cargo run --bin greeting_get_bin_with_client -- <contract_id>
//
// example:
//   cargo run --bin greeting_get_bin_with_client -- hello.sleet.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::greeting::greeting_get_fun::get_greeting;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let contract_id = args
        .get(1)
        .expect("usage: greeting_get_bin_with_client <contract_id>");

    let near = NEAR_KIT_CLIENT::from_env()?.build();

    println!("Fetching greeting from `{}`...", contract_id);

    let greeting = get_greeting(&near, contract_id).await?;

    println!("Current Greeting: {greeting}");

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
