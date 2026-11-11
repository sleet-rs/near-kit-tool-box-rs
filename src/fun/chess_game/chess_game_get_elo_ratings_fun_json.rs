// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use crate::lib::types::chess_game_elo_rating_type::CHESS_GAME_ELO_RATING_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the elo leaderboard of the chess contract using
/// raw JSON args.
///
/// Returns account id and rating pairs. Both `skip` and `limit` are
/// optional on the contract.
pub async fn get_elo_ratings(
    near: &Near,
    chess_contract_id: &str,
    skip: Option<u64>,
    limit: Option<u64>,
) -> Result<Vec<CHESS_GAME_ELO_RATING_TYPE>, Error> {
    let ratings: Vec<CHESS_GAME_ELO_RATING_TYPE> = near
        .view::<Vec<CHESS_GAME_ELO_RATING_TYPE>>(
            chess_contract_id,
            CHESS_GAME_METHODS_CONST.get_elo_ratings,
        )
        .args(json!({ "skip": skip, "limit": limit }))
        .await?;
    Ok(ratings)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
