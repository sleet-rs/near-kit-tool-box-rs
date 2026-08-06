// nft_supply_for_owner (json)
//
// Counts how many Pumpopoly NFTs an account holds on the mainnet NFT
// contract. No signer required.
//
// run:
//   cargo run --bin pumpopoly_nft_supply_for_owner_bin_json -- <account_id>
//
// example:
//   cargo run --bin pumpopoly_nft_supply_for_owner_bin_json -- sleet.near
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::pumpopoly_nft::pumpopoly_nft_supply_for_owner_fun_json::nft_supply_for_owner;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use near_kit_tool_box::lib::const_id::pumpopoly_nft_contract_id_const::PUMPOPOLY_NFT_CONTRACT_ID_CONST;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_id = args
        .get(1)
        .expect("usage: pumpopoly_nft_supply_for_owner_bin_json <account_id>");

    let near = NEAR_KIT_CLIENT::mainnet().build();

    println!("Counting Pumpopoly NFTs held by `{}`...", account_id);
    let supply = nft_supply_for_owner(&near, PUMPOPOLY_NFT_CONTRACT_ID_CONST, account_id).await?;
    println!("`{}` holds {} Pumpopoly NFT(s)", account_id, supply);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
