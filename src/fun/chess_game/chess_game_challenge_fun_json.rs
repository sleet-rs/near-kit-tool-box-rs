// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use near_kit::{AccountId, Error, Gas, Near};
use serde_json::json;
// =================================================
/// Build the challenge id the contract uses for a pair of accounts.
///
/// The challenge id is the two accounts joined with `-vs-`.
pub fn challenge_id(challenger_id: &str, challenged_id: &str) -> String {
    format!("{}-vs-{}", challenger_id, challenged_id)
}
// =================================================
/// Change helper: challenge another account to a chess game.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
///
/// The caller must be registered and cannot challenge themselves. An
/// account may hold **25 open challenges**.
///
/// To challenge with a **wager**, use an `ft_transfer_call` on the token
/// contract instead.
pub async fn challenge(
    near: &Near,
    chess_contract_id: &str,
    challenged_id: &str,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let challenged: AccountId = challenged_id.parse()?;
    let result = near
        .call(chess_contract_id, CHESS_GAME_METHODS_CONST.challenge)
        .args(json!({ "challenged_id": challenged }))
        .gas(Gas::from_tgas(30))
        .await?;
    Ok(result)
}
// =================================================
/// Change helper: accept a chess challenge, which starts the game.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
///
/// The challenge id is the two accounts joined with `-vs-`. Use
/// `challenge_id` to build it.
///
/// To accept a **wagered** challenge, use an `ft_transfer_call` on the
/// token contract instead.
pub async fn accept_challenge(
    near: &Near,
    chess_contract_id: &str,
    challenge_id: &str,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(chess_contract_id, CHESS_GAME_METHODS_CONST.accept_challenge)
        .args(json!({ "challenge_id": challenge_id }))
        .gas(Gas::from_tgas(30))
        .await?;
    Ok(result)
}
// =================================================
/// Change helper: decline a chess challenge.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
///
/// `is_challenger` is `true` for the sender of the challenge and `false`
/// for the receiver. Either party may reject. A wagered challenge refunds
/// the tokens to the challenger.
pub async fn reject_challenge(
    near: &Near,
    chess_contract_id: &str,
    challenge_id: &str,
    is_challenger: bool,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(chess_contract_id, CHESS_GAME_METHODS_CONST.reject_challenge)
        .args(json!({
            "challenge_id": challenge_id,
            "is_challenger": is_challenger,
        }))
        .gas(Gas::from_tgas(30))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
