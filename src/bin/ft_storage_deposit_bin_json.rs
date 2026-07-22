// ft_storage_deposit (json)
//
// Registers storage on an FT contract for the signer via raw JSON
// args. Attaches 0.00125 NEAR.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin ft_storage_deposit_bin_json -- <ft_contract_id>
//
// example:
//   cargo run --bin ft_storage_deposit_bin_json -- wrap.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::ft::ft_storage_deposit_fun_json::ft_storage_deposit;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let ft_contract_id = args
        .get(1)
        .expect("usage: ft_storage_deposit_bin_json <ft_contract_id>");

    let near = NEAR_KIT_CLIENT::from_env()?.build();
    let account_id = env::var("NEAR_ACCOUNT_ID").expect("NEAR_ACCOUNT_ID env var is required");

    println!(
        "Registering storage on `{}` for `{}`...",
        ft_contract_id, account_id
    );
    let result = ft_storage_deposit(&near, ft_contract_id).await?;
    println!("✅ Storage registered. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
