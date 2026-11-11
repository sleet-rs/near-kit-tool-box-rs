// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use near_kit::{AccountId, Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// The minimum deposit the chess contract requires to register an
/// account, in yoctoNEAR (0.05 NEAR).
///
/// The exact minimum is readable with the `storage_balance_bounds` view.
pub const CHESS_GAME_STORAGE_DEPOSIT_MIN_YOCTONEAR: u128 = 50_000_000_000_000_000_000_000;
// =================================================
/// Change helper: register an account on the chess contract, or top up
/// the storage balance of one that is already registered.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
///
/// `account_id` is optional and registers **another** account. Omit it to
/// register the caller. `registration_only` is optional and defaults to
/// `false`.
///
/// The deposit is the storage cost, at least 0.05 NEAR. A new account
/// starts with an elo rating of 1000, no points, and every stat at zero.
pub async fn storage_deposit(
    near: &Near,
    chess_contract_id: &str,
    account_id: Option<&str>,
    registration_only: Option<bool>,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let account = match account_id {
        Some(id) => Some(id.parse::<AccountId>()?),
        None => None,
    };
    let result = near
        .call(chess_contract_id, CHESS_GAME_METHODS_CONST.storage_deposit)
        .args(json!({
            "account_id": account,
            "registration_only": registration_only,
        }))
        .gas(Gas::from_tgas(30))
        .deposit(NearToken::from_yoctonear(
            CHESS_GAME_STORAGE_DEPOSIT_MIN_YOCTONEAR,
        ))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
