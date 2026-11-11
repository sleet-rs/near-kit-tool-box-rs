// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use crate::lib::types::chess_game_id_type::CHESS_GAME_ID_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the board of a chess game using raw JSON args.
///
/// Returns eight strings, one per rank. Row 0 is rank 8, row 7 is
/// rank 1. The pieces are `KQRBNP` for white and `kqrbnp` for black,
/// a space for an empty square.
///
/// The `game_id` is the `(block height, white, black)` tuple; the black
/// player is `None` for a game against the ai.
pub async fn get_board(
    near: &Near,
    chess_contract_id: &str,
    game_id: &CHESS_GAME_ID_TYPE,
) -> Result<Vec<String>, Error> {
    let board: Vec<String> = near
        .view::<Vec<String>>(chess_contract_id, CHESS_GAME_METHODS_CONST.get_board)
        .args(json!({ "game_id": game_id }))
        .await?;
    Ok(board)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
