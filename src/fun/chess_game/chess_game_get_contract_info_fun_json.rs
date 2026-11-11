// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the owner account id of the chess contract using
/// raw JSON args.
pub async fn get_owner(near: &Near, chess_contract_id: &str) -> Result<String, Error> {
    let owner: String = near
        .view::<String>(chess_contract_id, CHESS_GAME_METHODS_CONST.get_owner)
        .args(json!({}))
        .await?;
    Ok(owner)
}
// =================================================
/// View helper: fetch the treasury fee of the chess contract using raw
/// JSON args.
///
/// The fee is in basis points and is taken from the losing side of a
/// settled bet.
pub async fn get_fees(near: &Near, chess_contract_id: &str) -> Result<u32, Error> {
    let fees: u32 = near
        .view::<u32>(chess_contract_id, CHESS_GAME_METHODS_CONST.get_fees)
        .args(json!({}))
        .await?;
    Ok(fees)
}
// =================================================
/// View helper: fetch the token whitelist of the chess contract using
/// raw JSON args.
///
/// Returns the token contracts the contract accepts for wagers and bets.
pub async fn get_token_whitelist(
    near: &Near,
    chess_contract_id: &str,
) -> Result<Vec<String>, Error> {
    let whitelist: Vec<String> = near
        .view::<Vec<String>>(
            chess_contract_id,
            CHESS_GAME_METHODS_CONST.get_token_whitelist,
        )
        .args(json!({}))
        .await?;
    Ok(whitelist)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
