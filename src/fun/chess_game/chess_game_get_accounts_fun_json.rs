// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the registered chess account ids, paginated, using
/// raw JSON args.
///
/// Both `skip` and `limit` are optional on the contract.
pub async fn get_accounts(
    near: &Near,
    chess_contract_id: &str,
    skip: Option<u64>,
    limit: Option<u64>,
) -> Result<Vec<String>, Error> {
    let accounts: Vec<String> = near
        .view::<Vec<String>>(chess_contract_id, CHESS_GAME_METHODS_CONST.get_accounts)
        .args(json!({ "skip": skip, "limit": limit }))
        .await?;
    Ok(accounts)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
