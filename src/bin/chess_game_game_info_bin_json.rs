// game_info (json)
//
// Reads the players, the turn and the bet flag of a chess game from the
// mainnet chess contract. No signer required.
//
// run:
//   cargo run --bin chess_game_game_info_bin_json -- <game_id>
//
// The game id is the json tuple of the block height, the white player and the
// black player. Use `null` for the black player on a game against the ai.
//
// example:
//   cargo run --bin chess_game_game_info_bin_json -- '[113689871,"within4d45.near","marior.near"]'
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::chess_game::chess_game_game_info_fun_json::game_info;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use near_kit_tool_box::lib::const_id::chess_game_contract_id_const::CHESS_GAME_CONTRACT_ID_CONST;
use near_kit_tool_box::lib::types::chess_game_id_type::CHESS_GAME_ID_TYPE;
use std::env;
// =================================================
fn parse_game_id(raw: &str) -> Result<CHESS_GAME_ID_TYPE, Error> {
    let value: serde_json::Value =
        serde_json::from_str(raw).map_err(|e| near_kit::Error::Config(e.to_string()))?;
    let arr = value
        .as_array()
        .ok_or_else(|| near_kit::Error::Config("game id must be a json array".to_string()))?;
    if arr.len() != 3 {
        return Err(near_kit::Error::Config(
            "game id must have three values".to_string(),
        ));
    }
    let height = arr[0]
        .as_u64()
        .ok_or_else(|| near_kit::Error::Config("first value must be a block height".to_string()))?;
    let white = arr[1]
        .as_str()
        .ok_or_else(|| near_kit::Error::Config("second value must be an account id".to_string()))?
        .to_string();
    let black = arr[2].as_str().map(|s| s.to_string());
    Ok((height, white, black))
}
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let raw_game_id = args
        .get(1)
        .expect("usage: chess_game_game_info_bin_json <game_id>");

    let game_id = parse_game_id(raw_game_id)?;
    let near = NEAR_KIT_CLIENT::mainnet().build();

    println!("Fetching chess game info for `{}`...", raw_game_id);
    let info = game_info(&near, CHESS_GAME_CONTRACT_ID_CONST, &game_id).await?;
    println!("{:#?}", info);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
