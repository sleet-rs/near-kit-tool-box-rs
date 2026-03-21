// set_greeting
// 
// NEAR_NETWORK=testnet \
// NEAR_ACCOUNT_ID=alice.testnet \
// NEAR_PRIVATE_KEY=ed25519:YOUR_PRIVATE_KEY_HERE \
// GREETING_CONTRACT_ID=hello.sleet.testnet \
// GREETING_NEW_TEXT="hello from near kit rust" \
// cargo run --bin greeting_set_bin
// 
// =================================================
use std::env;
use near_kit::*;
use near_kit_tool_box::fun::greeting_set_fun::set_greeting;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Read config from env vars
    let network = env::var("NEAR_NETWORK").unwrap_or_else(|_| "testnet".to_string());
    let account_id =
        env::var("NEAR_ACCOUNT_ID").expect("NEAR_ACCOUNT_ID env var is required for signing");
    let private_key =
        env::var("NEAR_PRIVATE_KEY").expect("NEAR_PRIVATE_KEY env var is required for signing");
    let contract_id =
        env::var("GREETING_CONTRACT_ID").expect("GREETING_CONTRACT_ID env var is required");
    let new_greeting =
        env::var("GREETING_NEW_TEXT").expect("GREETING_NEW_TEXT env var is required");

    // Build Near client using the chosen network and credentials
    let near = match network.as_str() {
        "mainnet" => Near::mainnet()
            .credentials(&private_key, &account_id)?
            .build(),
        "testnet" => Near::testnet()
            .credentials(&private_key, &account_id)?
            .build(),
        other => {
            panic!("Unsupported NEAR_NETWORK value `{other}` (use `mainnet` or `testnet`)");
        }
    };

    println!(
        "Setting greeting on `{}` as `{}` to: {}",
        contract_id, account_id, new_greeting
    );

    set_greeting(&near, &contract_id, new_greeting).await?;

    println!("✅ Greeting updated successfully");

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near