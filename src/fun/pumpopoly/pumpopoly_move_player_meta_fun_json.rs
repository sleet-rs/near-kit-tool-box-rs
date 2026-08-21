// use near_kit::*;
use crate::lib::methods::methods_pumpopoly_world::PUMPOPOLY_WORLD_METHODS_CONST;
use near_kit::{DelegateOptions, DelegateResult, Error, Gas, Near};
use serde_json::json;
// =================================================
/// Build and sign a meta-transaction (NEP-366) that calls
/// `move_player` on `pumpopoly_contract_id`. The returned
/// [`DelegateResult`] can be transported to a relayer (off-chain) and
/// submitted via [`submit_move_player_meta`] — the relayer pays gas on
/// behalf of the inner signer.
///
/// `user_near` MUST have credentials set for the player account whose
/// on-chain access key the delegation is bound to (in practice the
/// shared `PUMPOPOLY_LIMITED_KEY` function-call key, one per account).
/// The relayer is then free to be any funded account.
pub async fn move_player_meta(
    user_near: &Near,
    pumpopoly_contract_id: &str,
) -> Result<DelegateResult, Error> {
    let result = user_near
        .transaction(pumpopoly_contract_id)
        .call(PUMPOPOLY_WORLD_METHODS_CONST.move_player)
        .args(json!({}))
        .gas(Gas::from_tgas(60))
        .delegate(DelegateOptions::default())
        .await?;
    Ok(result)
}
// =================================================
/// Wrap a previously signed [`DelegateResult`] (built by
/// [`move_player_meta`]) in an outer transaction signed by
/// `relayer_near` and submit it to the chain. The relayer pays for gas
/// on behalf of the inner signer.
pub async fn submit_move_player_meta(
    relayer_near: &Near,
    delegate: DelegateResult,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let signed = delegate.signed_delegate_action;
    let result = relayer_near
        .transaction(signed.sender_id())
        .signed_delegate_action(signed)
        .send()
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
