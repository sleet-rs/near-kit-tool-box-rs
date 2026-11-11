// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use crate::lib::types::chess_game_id_type::CHESS_GAME_ID_TYPE;
use near_kit::{Error, Gas, Near};
use serde_json::json;
// =================================================
/// Change helper: give up a chess game.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
///
/// The opponent wins. A resignation can happen on any turn.
pub async fn resign(
    near: &Near,
    chess_contract_id: &str,
    game_id: &CHESS_GAME_ID_TYPE,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(chess_contract_id, CHESS_GAME_METHODS_CONST.resign)
        .args(json!({ "game_id": game_id }))
        .gas(Gas::from_tgas(300))
        .await?;
    Ok(result)
}
// =================================================
/// Change helper: cancel a chess game that has gone stale.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
///
/// A player may cancel their own game after **3 days** of inactivity, and
/// only when it is not their turn. Anyone else may cancel a game after
/// **14 days** of inactivity. Wagers and bets are refunded on a
/// cancellation.
pub async fn cancel(
    near: &Near,
    chess_contract_id: &str,
    game_id: &CHESS_GAME_ID_TYPE,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(chess_contract_id, CHESS_GAME_METHODS_CONST.cancel)
        .args(json!({ "game_id": game_id }))
        .gas(Gas::from_tgas(300))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
