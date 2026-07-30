// view_player (json)
//
// Reads a Pumpopoly player's state from the mainnet world contract. No signer
// required.
//
// run:
//   cargo run --bin pumpopoly_view_player_bin_json -- <account_id>
//
// example:
//   cargo run --bin pumpopoly_view_player_bin_json -- sleet.near
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::pumpopoly::pumpopoly_view_player_fun_json::view_player;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use near_kit_tool_box::lib::const_id::pumpopoly_world_contract_id_const::PUMPOPOLY_WORLD_CONTRACT_ID_CONST;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_id = args
        .get(1)
        .expect("usage: pumpopoly_view_player_bin_json <account_id>");

    let near = NEAR_KIT_CLIENT::mainnet().build();

    println!("Fetching Pumpopoly player `{}`...", account_id);
    let player = view_player(&near, PUMPOPOLY_WORLD_CONTRACT_ID_CONST, account_id).await?;
    println!("{:#?}", player);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
