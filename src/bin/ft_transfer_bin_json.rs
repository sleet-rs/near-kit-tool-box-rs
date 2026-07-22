// ft_transfer (json)
//
// Transfers FT to `receiver_id` via raw JSON args.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin ft_transfer_bin_json -- <ft_contract_id> <receiver_id> <amount>
//
// example:
//   cargo run --bin ft_transfer_bin_json -- wrap.testnet alice.testnet 1000000000000000000000000
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::ft::ft_transfer_fun_json::ft_transfer;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let ft_contract_id = args
        .get(1)
        .expect("usage: ft_transfer_bin_json <ft_contract_id> <receiver_id> <amount>");
    let receiver_id = args
        .get(2)
        .expect("usage: ft_transfer_bin_json <ft_contract_id> <receiver_id> <amount>");
    let amount = args
        .get(3)
        .expect("usage: ft_transfer_bin_json <ft_contract_id> <receiver_id> <amount>");

    let near = NEAR_KIT_CLIENT::from_env()?.build();
    let account_id = env::var("NEAR_ACCOUNT_ID").expect("NEAR_ACCOUNT_ID env var is required");

    println!(
        "Transferring {} of `{}` from `{}` to `{}`...",
        amount, ft_contract_id, account_id, receiver_id
    );
    let result = ft_transfer(&near, ft_contract_id, receiver_id, amount).await?;
    println!("✅ Transfer complete. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
