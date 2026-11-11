// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use crate::lib::types::chess_game_elo_rating_type::CHESS_GAME_ELO_RATING_TYPE;
use near_kit::{AccountId, Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the elo ratings of specific chess accounts using
/// raw JSON args.
///
/// Returns account id and rating pairs, in the order of `account_ids`.
pub async fn get_elo_ratings_by_ids(
    near: &Near,
    chess_contract_id: &str,
    account_ids: &[String],
) -> Result<Vec<CHESS_GAME_ELO_RATING_TYPE>, Error> {
    let accounts: Vec<AccountId> = account_ids
        .iter()
        .map(|id| id.parse::<AccountId>())
        .collect::<Result<Vec<AccountId>, _>>()?;
    let ratings: Vec<CHESS_GAME_ELO_RATING_TYPE> = near
        .view::<Vec<CHESS_GAME_ELO_RATING_TYPE>>(
            chess_contract_id,
            CHESS_GAME_METHODS_CONST.get_elo_ratings_by_ids,
        )
        .args(json!({ "account_ids": accounts }))
        .await?;
    Ok(ratings)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
