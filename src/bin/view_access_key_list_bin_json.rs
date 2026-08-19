// view_access_key_list (json)
//
// Reads `view_access_key_list` from a NEAR client built from env vars.
// No signer required — only `NEAR_NETWORK` is needed.
//
// usage:
//   cargo run --bin view_access_key_list_bin_json -- <account_id>
//
// examples:
//   cargo run --bin view_access_key_list_bin_json -- sleet.testnet
//
// =================================================
use near_kit::Error;
use near_kit::types::AccessKeyPermissionView;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_id = args
        .get(1)
        .expect("usage: view_access_key_list_bin_json <account_id>");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Fetching access keys for `{}`...", account_id);

    let list = near.access_keys(account_id).await?;

    println!(
        "Access keys ({}):  block_height={} block_hash={}",
        list.keys.len(),
        list.block_height,
        list.block_hash,
    );

    for (i, k) in list.keys.iter().enumerate() {
        println!("\n[{i}] public_key: {}", k.public_key);
        println!("    nonce:      {}", k.access_key.nonce);
        match &k.access_key.permission {
            AccessKeyPermissionView::FullAccess => {
                println!("    permission: FullAccess");
            }
            AccessKeyPermissionView::FunctionCall {
                allowance,
                receiver_id,
                method_names,
            } => {
                println!("    permission: FunctionCall");
                println!(
                    "    allowance:  {}",
                    allowance
                        .map(|a| a.to_string())
                        .unwrap_or_else(|| "none".to_string())
                );
                println!("    receiver:   {receiver_id}");
                println!(
                    "    methods:    {}",
                    if method_names.is_empty() {
                        "(all)".to_string()
                    } else {
                        method_names.join(", ")
                    }
                );
            }
            AccessKeyPermissionView::GasKeyFunctionCall {
                balance,
                num_nonces,
                allowance,
                receiver_id,
                method_names,
            } => {
                println!("    permission: GasKeyFunctionCall");
                println!("    balance:    {balance}");
                println!("    num_nonces: {num_nonces}");
                println!(
                    "    allowance:  {}",
                    allowance
                        .map(|a| a.to_string())
                        .unwrap_or_else(|| "none".to_string())
                );
                println!("    receiver:   {receiver_id}");
                println!(
                    "    methods:    {}",
                    if method_names.is_empty() {
                        "(all)".to_string()
                    } else {
                        method_names.join(", ")
                    }
                );
            }
            AccessKeyPermissionView::GasKeyFullAccess {
                balance,
                num_nonces,
            } => {
                println!("    permission: GasKeyFullAccess");
                println!("    balance:    {balance}");
                println!("    num_nonces: {num_nonces}");
            }
        }
    }

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
