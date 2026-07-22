// delete_account (json)
//
// Deletes the signer's account and transfers remaining balance to
// `beneficiary` using the NEAR transaction-builder API.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin delete_account_bin_json -- <beneficiary>
//
// example:
//   cargo run --bin delete_account_bin_json -- sleet.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::near::delete_account_fun_json::delete_account;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let beneficiary = args
        .get(1)
        .expect("usage: delete_account_bin_json <beneficiary>");

    let near = NEAR_KIT_CLIENT::from_env()?.build();
    let signer_id = env::var("NEAR_ACCOUNT_ID").expect("NEAR_ACCOUNT_ID env var is required");

    println!(
        "Deleting account `{}`, beneficiary `{}`...",
        signer_id, beneficiary
    );
    let result = delete_account(&near, &signer_id, beneficiary).await?;
    println!("✅ Account deleted. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
