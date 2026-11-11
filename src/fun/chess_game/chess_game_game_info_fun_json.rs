// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use crate::lib::types::chess_game_id_type::CHESS_GAME_ID_TYPE;
use crate::lib::types::chess_game_info_type::CHESS_GAME_INFO_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the players, the turn and the bet flag of a chess
/// game using raw JSON args.
///
/// The response holds `white` and `black` as a human account id or an ai
/// difficulty, the `turn_color` of `"White"` or `"Black"`, the
/// `last_block_height`, and `has_bets`.
pub async fn game_info(
    near: &Near,
    chess_contract_id: &str,
    game_id: &CHESS_GAME_ID_TYPE,
) -> Result<CHESS_GAME_INFO_TYPE, Error> {
    let info: CHESS_GAME_INFO_TYPE = near
        .view::<CHESS_GAME_INFO_TYPE>(chess_contract_id, CHESS_GAME_METHODS_CONST.game_info)
        .args(json!({ "game_id": game_id }))
        .await?;
    Ok(info)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
