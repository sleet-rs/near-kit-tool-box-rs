// use near_kit::*;
use crate::lib::methods::methods_chess_game::CHESS_GAME_METHODS_CONST;
use near_kit::{Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// Change helper: mark the caller as an automated agent.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
///
/// The flag is public and visible to anyone through `get_account`. It does
/// not change how the contract behaves.
pub async fn set_is_agent(
    near: &Near,
    chess_contract_id: &str,
    is_agent: bool,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(chess_contract_id, CHESS_GAME_METHODS_CONST.set_is_agent)
        .args(json!({ "is_agent": is_agent }))
        .gas(Gas::from_tgas(10))
        .deposit(NearToken::from_yoctonear(1))
        .await?;
    Ok(result)
}
// =================================================
/// Change helper: withdraw the storage balance of the caller.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
///
/// **NOT IMPLEMENTED on the contract** — `internal_storage_withdraw`
/// returns `ContractError::OperationNotSupported` for every call, so no
/// storage balance can ever be withdrawn this way. The helper is kept
/// because the method exists on the contract and the call type, gas and
/// deposit are correct — but it will not succeed.
///
/// `amount` is optional and is a yoctoNEAR decimal string.
pub async fn storage_withdraw(
    near: &Near,
    chess_contract_id: &str,
    amount: Option<&str>,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(chess_contract_id, CHESS_GAME_METHODS_CONST.storage_withdraw)
        .args(json!({ "amount": amount }))
        .gas(Gas::from_tgas(30))
        .deposit(NearToken::from_yoctonear(1))
        .await?;
    Ok(result)
}
// =================================================
/// Change helper: withdraw the tokens the chess contract holds for the
/// caller.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
///
/// The balance is a cancelled bet, a refund or a payout. Read it with the
/// `get_token_amount` view.
pub async fn withdraw_token(
    near: &Near,
    chess_contract_id: &str,
    token_id: &str,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(chess_contract_id, CHESS_GAME_METHODS_CONST.withdraw_token)
        .args(json!({ "token_id": token_id }))
        .gas(Gas::from_tgas(30))
        .deposit(NearToken::from_yoctonear(1))
        .await?;
    Ok(result)
}
// =================================================
/// Change helper: claim the pending ppp points of the caller.
///
/// Requires a `Near` client configured with signer credentials and returns
/// the final execution outcome.
///
/// Quest points earned in a wager or a bet flow are held as pending
/// points and land in the balance after this call.
pub async fn claim_points(
    near: &Near,
    chess_contract_id: &str,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(chess_contract_id, CHESS_GAME_METHODS_CONST.claim_points)
        .args(json!({}))
        .gas(Gas::from_tgas(30))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
