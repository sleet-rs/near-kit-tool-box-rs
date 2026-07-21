// set_greeting_typed
//
// Demonstrates using typed contracts with a call that requires an argument.
//
// NEAR_NETWORK=testnet \
// NEAR_ACCOUNT_ID=alice.testnet \
// NEAR_PRIVATE_KEY=ed25519:YOUR_PRIVATE_KEY_HERE \
// GREETING_CONTRACT_ID=hello.sleet.testnet \
// GREETING_NEW_TEXT="hello from near kit rust" \
// cargo run --bin greeting_set_bin_typed
//
// =================================================
use near_kit::*;
use near_kit_tool_box::fun::greeting::greeting_set_fun::set_greeting_typed;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let near = NEAR_KIT_CLIENT::from_env()?.build();
    let account_id =
        env::var("NEAR_ACCOUNT_ID").expect("NEAR_ACCOUNT_ID env var is required");
    let contract_id =
        env::var("GREETING_CONTRACT_ID").expect("GREETING_CONTRACT_ID env var is required");
    let new_greeting =
        env::var("GREETING_NEW_TEXT").expect("GREETING_NEW_TEXT env var is required");

    println!(
        "Setting greeting on `{}` as `{}` to: {}",
        contract_id, account_id, new_greeting
    );

    // Use the reusable typed function
    set_greeting_typed(&near, &contract_id, new_greeting).await?;

    println!("✅ Greeting updated successfully");

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
