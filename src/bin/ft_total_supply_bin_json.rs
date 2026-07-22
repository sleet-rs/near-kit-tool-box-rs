// ft_total_supply (json)
//
// Reads an FT total supply via raw JSON args. No signer required.
//
// run:
//   cargo run --bin ft_total_supply_bin_json -- <ft_contract_id>
//
// example:
//   cargo run --bin ft_total_supply_bin_json -- wrap.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::ft::ft_total_supply_fun_json::ft_total_supply;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let ft_contract_id = args
        .get(1)
        .expect("usage: ft_total_supply_bin_json <ft_contract_id>");

    let near = NEAR_KIT_CLIENT::from_env()?.build();

    println!("Fetching FT total supply on `{}`...", ft_contract_id);
    let supply = ft_total_supply(&near, ft_contract_id).await?;
    println!("Total supply: {}", supply);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
