// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use crate::lib::types::chess_game_id_type::CHESS_GAME_ID_TYPE;
use near_kit::{Error, Gas, Near};
use serde_json::json;
// =================================================
/// The difficulty of the on-chain chess ai.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CHESS_GAME_DIFFICULTY {
    Easy,
    Medium,
    Hard,
    VeryHard,
}
// =================================================
impl CHESS_GAME_DIFFICULTY {
    /// The difficulty string the contract expects.
    pub fn as_str(&self) -> &'static str {
        match self {
            CHESS_GAME_DIFFICULTY::Easy => "Easy",
            CHESS_GAME_DIFFICULTY::Medium => "Medium",
            CHESS_GAME_DIFFICULTY::Hard => "Hard",
            CHESS_GAME_DIFFICULTY::VeryHard => "VeryHard",
        }
    }

    /// The gas the contract needs for a call against this difficulty.
    ///
    /// `create_ai_game` and `play_move` both scale with the ai difficulty:
    /// 50 Tgas for `Easy`, 150 for `Medium`, 250 for `Hard` and 400 for
    /// `VeryHard`.
    pub fn gas_tgas(&self) -> u64 {
        match self {
            CHESS_GAME_DIFFICULTY::Easy => 50,
            CHESS_GAME_DIFFICULTY::Medium => 150,
            CHESS_GAME_DIFFICULTY::Hard => 250,
            CHESS_GAME_DIFFICULTY::VeryHard => 400,
        }
    }
}
// =================================================
/// Change helper: start a game against the on-chain chess ai, as white.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
///
/// The caller must be registered. The gas scales with the difficulty —
/// 50 Tgas for `Easy`, 150 for `Medium`, 250 for `Hard` and 400 for
/// `VeryHard`.
pub async fn create_ai_game(
    near: &Near,
    chess_contract_id: &str,
    difficulty: CHESS_GAME_DIFFICULTY,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(chess_contract_id, CHESS_GAME_METHODS_CONST.create_ai_game)
        .args(json!({ "difficulty": difficulty.as_str() }))
        .gas(Gas::from_tgas(difficulty.gas_tgas()))
        .await?;
    Ok(result)
}
// =================================================
/// The `GameId` a `create_ai_game` call returned.
///
/// The outcome of a chess transaction carries the return value of the
/// method in its logs — the game id for `create_ai_game`, and the
/// outcome and the board for `play_move`.
///
/// Verified against live `app.chess-game.near` responses on 2026-11-11.
pub type CHESS_GAME_CREATE_OUTCOME_TYPE = CHESS_GAME_ID_TYPE;
// =================================================
// =================================================
// copyright 2026 by sleet.near
