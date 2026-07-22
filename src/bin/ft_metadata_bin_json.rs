// ft_metadata (json)
//
// Reads an FT contract's metadata via raw JSON args. No signer
// required.
//
// run:
//   cargo run --bin ft_metadata_bin_json -- <ft_contract_id>
//
// example:
//   cargo run --bin ft_metadata_bin_json -- wrap.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::ft::ft_metadata_fun_json::ft_metadata;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let ft_contract_id = args
        .get(1)
        .expect("usage: ft_metadata_bin_json <ft_contract_id>");

    let near = NEAR_KIT_CLIENT::from_env()?.build();

    println!("Fetching FT metadata for `{}`...", ft_contract_id);
    let meta = ft_metadata(&near, ft_contract_id).await?;
    println!(
        "{} ({}) — spec {} decimals {}",
        meta.name, meta.symbol, meta.spec, meta.decimals
    );
    println!("{:?}", meta);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
