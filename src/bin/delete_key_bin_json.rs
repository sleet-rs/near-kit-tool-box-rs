// delete_key (json)
//
// Deletes an access key from the signer's account using the NEAR
// transaction-builder API.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin delete_key_bin_json -- <public_key>
//
// example:
//   cargo run --bin delete_key_bin_json -- ed25519:HDaBzemF6CYnQ2A3CgAh8vK7stWp5aXvKMKzKDe6s3QZ
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::near::delete_key_fun_json::delete_key;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let public_key = args
        .get(1)
        .expect("usage: delete_key_bin_json <public_key>");

    let near = NEAR_KIT_CLIENT::from_env()?.build();
    let signer_id = env::var("NEAR_ACCOUNT_ID").expect("NEAR_ACCOUNT_ID env var is required");

    println!(
        "Deleting access key `{}` from `{}`...",
        public_key, signer_id
    );
    let result = delete_key(&near, &signer_id, public_key).await?;
    println!("✅ Access key deleted. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
