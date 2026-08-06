// use near_kit::*;
use crate::lib::methods::methods_pumpopoly_nft::PUMPOPOLY_NFT_METHODS_CONST;
use near_kit::{AccountId, Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// Transfer a Pumpopoly NFT to `receiver_id` using raw JSON args.
///
/// Unlike FT transfers there is no amount — a token id is unique and moves
/// whole. The signer must own `token_id` (or hold an NEP-178 approval, in
/// which case pass the matching `approval_id`). The contract requires
/// exactly 1 yoctoNEAR attached as deposit for security.
///
/// `memo` is an optional free-form note recorded in the transfer event.
/// `approval_id` is a raw `u64` — the contract rejects a stringified
/// `"1"` with `invalid type: string, expected u64`. Both may be `None`,
/// which serialises to `null` and deserialises the same as omitting the
/// key.
///
/// Arg shape verified against mainnet tx
/// `jRDQPvKvdHMd9nMjv2ugqGXAvjF2wN2uocdEUFJqysL`.
pub async fn nft_transfer(
    near: &Near,
    nft_contract_id: &str,
    receiver_id: &str,
    token_id: &str,
    approval_id: Option<u64>,
    memo: Option<&str>,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let receiver: AccountId = receiver_id.parse()?;
    let result = near
        .call(nft_contract_id, PUMPOPOLY_NFT_METHODS_CONST.nft_transfer)
        .args(json!({
            "receiver_id": receiver,
            "token_id": token_id,
            "approval_id": approval_id,
            "memo": memo,
        }))
        .gas(Gas::from_tgas(30))
        .deposit(NearToken::from_yoctonear(1))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
