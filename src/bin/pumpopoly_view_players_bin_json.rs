// view_players (json)
//
// Reads multiple Pumpopoly players' state from the mainnet world contract in
// a single view call. No signer required.
//
// run:
//   cargo run --bin pumpopoly_view_players_bin_json -- <account_id> [<account_id> ...]
//
// example:
//   cargo run --bin pumpopoly_view_players_bin_json -- sleet.near narkmeta.near
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::pumpopoly::pumpopoly_view_players_fun_json::view_players;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use near_kit_tool_box::lib::const_id::pumpopoly_world_contract_id_const::PUMPOPOLY_WORLD_CONTRACT_ID_CONST;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_ids: Vec<String> = args.iter().skip(1).cloned().collect();
    if account_ids.is_empty() {
        panic!("usage: pumpopoly_view_players_bin_json <account_id> [<account_id> ...]");
    }

    let near = NEAR_KIT_CLIENT::mainnet().build();

    let account_refs: Vec<&str> = account_ids.iter().map(String::as_str).collect();
    println!(
        "Fetching Pumpopoly players `{}`...",
        account_refs.join(", ")
    );
    let players = view_players(&near, PUMPOPOLY_WORLD_CONTRACT_ID_CONST, &account_refs).await?;
    println!("{:#?}", players);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
