// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use crate::lib::types::chess_game_storage_bounds_type::CHESS_GAME_STORAGE_BOUNDS_TYPE;
use near_kit::{AccountId, Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the NEP-145 storage balance bounds of the chess
/// contract using raw JSON args.
///
/// The `min` is the exact deposit a caller must attach to
/// `storage_deposit` to register an account.
pub async fn storage_balance_bounds(
    near: &Near,
    chess_contract_id: &str,
) -> Result<CHESS_GAME_STORAGE_BOUNDS_TYPE, Error> {
    let bounds: CHESS_GAME_STORAGE_BOUNDS_TYPE = near
        .view::<CHESS_GAME_STORAGE_BOUNDS_TYPE>(
            chess_contract_id,
            CHESS_GAME_METHODS_CONST.storage_balance_bounds,
        )
        .args(json!({}))
        .await?;
    Ok(bounds)
}
// =================================================
/// View helper: fetch the NEP-145 storage balance of one account on the
/// chess contract using raw JSON args.
///
/// All amounts are yoctoNEAR decimal strings.
pub async fn storage_balance_of(
    near: &Near,
    chess_contract_id: &str,
    account_id: &str,
) -> Result<
    crate::lib::types::storage_balance_of_response_type::STORAGE_BALANCE_OF_RESPONSE_TYPE,
    Error,
> {
    let account: AccountId = account_id.parse()?;
    let balance: crate::lib::types::storage_balance_of_response_type::STORAGE_BALANCE_OF_RESPONSE_TYPE =
        near
            .view(chess_contract_id, CHESS_GAME_METHODS_CONST.storage_balance_of)
            .args(json!({ "account_id": account }))
            .await?;
    Ok(balance)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
