// set_greeting
//
// Demonstrates using typed contracts with a call that requires an argument.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin greeting_set_bin_typed -- <contract_id> "<new_text>"
//
// example:
//   cargo run --bin greeting_set_bin_typed -- hello.sleet.testnet "hello from near kit rust"
//
// =================================================
use near_kit::*;
use near_kit_tool_box::fun::greeting::greeting_set_fun::set_greeting;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let contract_id = args
        .get(1)
        .expect("usage: greeting_set_bin_typed <contract_id> \"<new_text>\"");
    let new_greeting = args
        .get(2)
        .expect("usage: greeting_set_bin_typed <contract_id> \"<new_text>\"");

    let near = NEAR_KIT_CLIENT::from_env()?.build();
    let account_id = env::var("NEAR_ACCOUNT_ID").expect("NEAR_ACCOUNT_ID env var is required");

    println!(
        "Setting greeting on `{}` as `{}` to: {}",
        contract_id, account_id, new_greeting
    );

    // Use the reusable typed function
    set_greeting(&near, contract_id, new_greeting.clone()).await?;

    println!("✅ Greeting updated successfully");

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
