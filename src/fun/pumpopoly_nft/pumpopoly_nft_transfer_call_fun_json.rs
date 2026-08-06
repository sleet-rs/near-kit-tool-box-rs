// use near_kit::*;
use crate::lib::methods::methods_pumpopoly_nft::PUMPOPOLY_NFT_METHODS_CONST;
use near_kit::{AccountId, Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// Transfer a Pumpopoly NFT to `receiver_id` and call `nft_on_transfer`
/// on it with `msg`, using raw JSON args.
///
/// Use this to hand a token to a contract (a marketplace listing, for
/// example) — the receiver can reject it, in which case the NFT is
/// returned to the signer by `nft_resolve_transfer`. Requires exactly
/// 1 yoctoNEAR deposit. Gas is set high because the cross-contract call
/// and the resolve callback both run inside this transaction.
///
/// Arg shape verified against mainnet tx
/// `8fwpXybHJd8Vfo3RAdcaejq6gybRPconB73djA3MwY9o`, which listed a bond on
/// `intents.near` with 100 TGas.
pub async fn nft_transfer_call(
    near: &Near,
    nft_contract_id: &str,
    receiver_id: &str,
    token_id: &str,
    msg: &str,
    approval_id: Option<u64>,
    memo: Option<&str>,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let receiver: AccountId = receiver_id.parse()?;
    let result = near
        .call(
            nft_contract_id,
            PUMPOPOLY_NFT_METHODS_CONST.nft_transfer_call,
        )
        .args(json!({
            "receiver_id": receiver,
            "token_id": token_id,
            "approval_id": approval_id,
            "memo": memo,
            "msg": msg,
        }))
        .gas(Gas::from_tgas(100))
        .deposit(NearToken::from_yoctonear(1))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
