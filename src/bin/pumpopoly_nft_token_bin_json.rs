// nft_token (json)
//
// Reads a single Pumpopoly NFT by token id from the mainnet NFT contract.
// No signer required.
//
// Token ids contain spaces, so quote them.
//
// run:
//   cargo run --bin pumpopoly_nft_token_bin_json -- <token_id>
//
// example:
//   cargo run --bin pumpopoly_nft_token_bin_json -- "Bond 10-849-1"
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::pumpopoly_nft::pumpopoly_nft_token_fun_json::nft_token;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use near_kit_tool_box::lib::const_id::pumpopoly_nft_contract_id_const::PUMPOPOLY_NFT_CONTRACT_ID_CONST;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let token_id = args
        .get(1)
        .expect("usage: pumpopoly_nft_token_bin_json <token_id>");

    let near = NEAR_KIT_CLIENT::mainnet().build();

    println!("Fetching Pumpopoly NFT `{}`...", token_id);
    match nft_token(&near, PUMPOPOLY_NFT_CONTRACT_ID_CONST, token_id).await? {
        Some(token) => println!("{:#?}", token),
        None => println!("No such token: `{}`", token_id),
    }
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
