// ft_balance_of (json)
//
// Reads an FT balance via raw JSON args. No signer required — only
// `NEAR_NETWORK` is needed (defaults to testnet).
//
// run:
//   cargo run --bin ft_balance_of_bin_json -- <ft_contract_id> <account_id>
//
// example:
//   cargo run --bin ft_balance_of_bin_json -- wrap.testnet sleet.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::ft::ft_balance_of_fun_json::ft_balance_of;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let ft_contract_id = args
        .get(1)
        .expect("usage: ft_balance_of_bin_json <ft_contract_id> <account_id>");
    let account_id = args
        .get(2)
        .expect("usage: ft_balance_of_bin_json <ft_contract_id> <account_id>");

    let near = NEAR_KIT_CLIENT::from_env()?.build();

    println!(
        "Fetching FT balance of `{}` on `{}`...",
        account_id, ft_contract_id
    );
    let balance = ft_balance_of(&near, ft_contract_id, account_id).await?;
    println!("Balance: {}", balance);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
