// view_account (json)
//
// Reads `view_account` from a NEAR client built from env vars.
// No signer required — only `NEAR_NETWORK` is needed.
//
// usage:
//   cargo run --bin view_account_bin_json -- <account_id>
//
// examples:
//   cargo run --bin view_account_bin_json -- sleet.testnet
//   cargo run --bin view_account_bin_json -- hello.sleet.near
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_id = args
        .get(1)
        .expect("usage: view_account_bin_json <account_id>");

    let near = NEAR_KIT_CLIENT::from_env()?.build();

    println!("Fetching account info for `{}`...", account_id);

    let account = near.account(account_id).await?;

    println!("Account view:");
    println!("  amount:       {}", account.amount);
    println!("  locked:       {}", account.locked);
    println!("  available:    {}", account.available());
    println!("  storage_used: {} bytes", account.storage_usage);
    println!("  has_contract: {}", account.has_contract());
    println!("  code_hash:    {}", account.code_hash);
    println!("  block_height: {}", account.block_height);
    println!("  block_hash:   {}", account.block_hash);

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
