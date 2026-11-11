// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use crate::lib::types::chess_game_id_type::CHESS_GAME_ID_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the board of a chess game as one formatted string.
///
/// Returns the same board as `get_board`, but rendered with unicode
/// pieces and chessboard squares.
pub async fn render_board(
    near: &Near,
    chess_contract_id: &str,
    game_id: &CHESS_GAME_ID_TYPE,
) -> Result<String, Error> {
    let board: String = near
        .view::<String>(chess_contract_id, CHESS_GAME_METHODS_CONST.render_board)
        .args(json!({ "game_id": game_id }))
        .await?;
    Ok(board)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
