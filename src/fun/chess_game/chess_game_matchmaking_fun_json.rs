// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use near_kit::{Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// Change helper: join the chess matchmaking queue.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
///
/// `min_elo` and `max_elo` are the rating range of the opponent to match
/// and are required. The caller must be registered, and must not already
/// be in a challenge or in the queue. The queue holds **200 entries**.
///
/// The deposit is the storage for the queue entry. It is refunded when
/// the entry is matched or cancelled.
///
/// To join the queue **with a wager**, use an `ft_transfer_call` on the
/// token contract instead.
pub async fn join_matchmaking(
    near: &Near,
    chess_contract_id: &str,
    min_elo: u64,
    max_elo: u64,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(chess_contract_id, CHESS_GAME_METHODS_CONST.join_matchmaking)
        .args(json!({ "min_elo": min_elo, "max_elo": max_elo }))
        .gas(Gas::from_tgas(30))
        .deposit(NearToken::from_millinear(10))
        .await?;
    Ok(result)
}
// =================================================
/// Change helper: leave the chess matchmaking queue.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
///
/// The storage deposit is refunded, and a wager deposited with the queue
/// entry is refunded too.
pub async fn cancel_matchmaking(
    near: &Near,
    chess_contract_id: &str,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(
            chess_contract_id,
            CHESS_GAME_METHODS_CONST.cancel_matchmaking,
        )
        .args(json!({}))
        .gas(Gas::from_tgas(10))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
