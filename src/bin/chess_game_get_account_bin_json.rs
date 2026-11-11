// get_account (json)
//
// Reads the registration, rating, points and stats of a chess account from
// the mainnet chess contract. No signer required.
//
// run:
//   cargo run --bin chess_game_get_account_bin_json -- <account_id>
//
// example:
//   cargo run --bin chess_game_get_account_bin_json -- marior.near
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::chess_game::chess_game_get_account_fun_json::get_account;
use near_kit_tool_box::fun::chess_game::chess_game_get_game_ids_fun_json::get_game_ids;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use near_kit_tool_box::lib::const_id::chess_game_contract_id_const::CHESS_GAME_CONTRACT_ID_CONST;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_id = args
        .get(1)
        .expect("usage: chess_game_get_account_bin_json <account_id>");

    let near = NEAR_KIT_CLIENT::mainnet().build();

    println!("Fetching chess account `{}`...", account_id);
    let account = get_account(&near, CHESS_GAME_CONTRACT_ID_CONST, account_id).await?;
    println!("{:#?}", account);

    println!("\nActive games:");
    let game_ids = get_game_ids(&near, CHESS_GAME_CONTRACT_ID_CONST, account_id).await?;
    for game_id in game_ids {
        println!("  {:?}", game_id);
    }
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
