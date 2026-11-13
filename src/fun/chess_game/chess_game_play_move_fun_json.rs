// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use crate::lib::types::chess_game_id_type::CHESS_GAME_ID_TYPE;
use near_kit::{Error, Gas, Near};
use serde_json::json;
// =================================================
/// Change helper: play a move in a chess game.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
///
/// `mv` is **coordinate notation**, not san. `"Nf3"` and `"Qxe4"` fail.
/// The accepted forms are `"e2e4"`, `"e2 e4"`, `"e2 to e4"`, `"O-O"`,
/// `"O-O-O"` and a promotion like `"e7 to e8 queen"` — a promotion
/// spells the piece out in full. The columns are a–h and the rows are
/// 1–8, counted from white's side.
///
/// The gas depends on the opponent: 50 Tgas against the `Easy` ai, 150
/// against the `Medium` ai or a human game, 250 against `Hard` and 400
/// against `VeryHard`. Use `play_move_with_gas` when the opponent is not
/// known up front.
///
/// Against the ai, the ai replies inside the same transaction.
pub async fn play_move(
    near: &Near,
    chess_contract_id: &str,
    game_id: &CHESS_GAME_ID_TYPE,
    mv: &str,
    gas_tgas: u64,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(chess_contract_id, CHESS_GAME_METHODS_CONST.play_move)
        .args(json!({ "game_id": game_id, "mv": mv }))
        .gas(Gas::from_tgas(gas_tgas))
        .await?;
    Ok(result)
}
// =================================================
/// Change helper: play a move in a chess game, with the gas chosen for
/// the given opponent.
///
/// `difficulty` is `None` for a game against a human, which needs 150
/// Tgas. Against the ai it is the ai difficulty.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
pub async fn play_move_with_gas(
    near: &Near,
    chess_contract_id: &str,
    game_id: &CHESS_GAME_ID_TYPE,
    mv: &str,
    difficulty: Option<
        crate::fun::chess_game::chess_game_create_ai_game_fun_json::CHESS_GAME_DIFFICULTY,
    >,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let gas_tgas = match difficulty {
        Some(d) => d.gas_tgas(),
        None => 150,
    };
    play_move(near, chess_contract_id, game_id, mv, gas_tgas).await
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
