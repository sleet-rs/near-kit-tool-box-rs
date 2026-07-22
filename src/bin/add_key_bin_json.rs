// add_key (json)
//
// Adds an access key to the signer's account using the NEAR
// transaction-builder API. Defaults to full-access; pass "fc" to add
// a function-call key instead, with receiver/method/allowance.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin add_key_bin_json -- <public_key>                              # full access
//   cargo run --bin add_key_bin_json -- <public_key> fc <receiver> <methods> <allowance>
//
// example:
//   cargo run --bin add_key_bin_json -- ed25519:HDaBzemF6CYnQ2A3CgAh8vK7stWp5aXvKMKzKDe6s3QZ
//   cargo run --bin add_key_bin_json -- ed25519:9vnVSMT1hv2Q1vuZzbjStFieqGbTrwqe4KcvfqYAkb5T fc wrap.testnet "near_deposit,near_withdraw" "1 NEAR"
//
// =================================================
use near_kit::{AccessKeyPermission, AccountId, Error, IntoNearToken};
use near_kit_tool_box::fun::near::add_key_fun_json::add_key;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let public_key = args
        .get(1)
        .expect("usage: add_key_bin_json <public_key> [fc <receiver> <methods> <allowance>]");

    let near = NEAR_KIT_CLIENT::from_env()?.build();
    let signer_id = env::var("NEAR_ACCOUNT_ID").expect("NEAR_ACCOUNT_ID env var is required");

    let permission = match args.get(2).map(String::as_str) {
        Some("fc") => {
            let receiver = args
                .get(3)
                .expect("missing <receiver> for function-call key");
            let methods = args
                .get(4)
                .expect("missing <methods> (comma-separated) for function-call key");
            let allowance = args
                .get(5)
                .map(|s| s.as_str().into_near_token())
                .transpose()?;
            AccessKeyPermission::function_call(
                receiver.parse::<AccountId>()?,
                methods.split(',').map(|s| s.trim().to_string()).collect(),
                allowance,
            )
        }
        _ => AccessKeyPermission::full_access(),
    };

    println!("Adding access key `{}` to `{}`...", public_key, signer_id);
    let result = add_key(&near, &signer_id, public_key, permission).await?;
    println!("✅ Access key added. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
