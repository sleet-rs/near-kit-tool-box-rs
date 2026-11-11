// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use crate::lib::types::chess_game_id_type::CHESS_GAME_ID_TYPE;
use near_kit::{AccountId, Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the active games of one chess account using raw
/// JSON args.
///
/// Returns the game ids of the account's active games.
pub async fn get_game_ids(
    near: &Near,
    chess_contract_id: &str,
    account_id: &str,
) -> Result<Vec<CHESS_GAME_ID_TYPE>, Error> {
    let account: AccountId = account_id.parse()?;
    let game_ids: Vec<CHESS_GAME_ID_TYPE> = near
        .view::<Vec<CHESS_GAME_ID_TYPE>>(chess_contract_id, CHESS_GAME_METHODS_CONST.get_game_ids)
        .args(json!({ "account_id": account }))
        .await?;
    Ok(game_ids)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
