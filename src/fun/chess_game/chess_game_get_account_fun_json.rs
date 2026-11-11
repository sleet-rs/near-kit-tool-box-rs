// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use crate::lib::types::chess_game_account_type::CHESS_GAME_ACCOUNT_TYPE;
use near_kit::{AccountId, Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the registration, rating, points and stats of one
/// chess account using raw JSON args.
///
/// The response holds `near_amount`, `is_agent`, `points`,
/// `pending_points`, `elo`, `wins`, `win_streak`, `max_win_streak`,
/// `bets_placed`, `bets_won`, `wagers_played`, `wager_wins` and
/// `challenges_sent`.
pub async fn get_account(
    near: &Near,
    chess_contract_id: &str,
    account_id: &str,
) -> Result<CHESS_GAME_ACCOUNT_TYPE, Error> {
    let account: AccountId = account_id.parse()?;
    let account_info: CHESS_GAME_ACCOUNT_TYPE = near
        .view::<CHESS_GAME_ACCOUNT_TYPE>(chess_contract_id, CHESS_GAME_METHODS_CONST.get_account)
        .args(json!({ "account_id": account }))
        .await?;
    Ok(account_info)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
