// ft_storage_balance_of (json)
//
// Reads the NEP-145 storage balance an account holds on an FT
// contract (e.g. `wrap.testnet`) via raw JSON args. Returns `{ total,
// available }` when registered, or an FT-specific shape otherwise.
//
// No signer required — only `NEAR_NETWORK` is needed (defaults to
// testnet).
//
// run:
//   cargo run --bin ft_storage_balance_of_bin_json -- <ft_contract_id> <account_id>
//
// example:
//   cargo run --bin ft_storage_balance_of_bin_json -- wrap.testnet sleet.testnet
//   cargo run --bin ft_storage_balance_of_bin_json -- usdt.tether-token.near sleet.near
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::ft::ft_storage_balance_of_fun_json::ft_storage_balance_of;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let ft_contract_id = args
        .get(1)
        .expect("usage: ft_storage_balance_of_bin_json <ft_contract_id> <account_id>");
    let account_id = args
        .get(2)
        .expect("usage: ft_storage_balance_of_bin_json <ft_contract_id> <account_id>");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!(
        "Fetching storage balance of `{}` on FT contract `{}`...",
        account_id, ft_contract_id
    );
    let balance = ft_storage_balance_of(&near, ft_contract_id, account_id).await?;
    println!("{:#?}", balance);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
