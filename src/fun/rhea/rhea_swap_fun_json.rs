// use near_kit::*;
use crate::lib::methods::methods_rhea::RHEA_METHODS_CONST;
use crate::lib::types::ref_swap_action_type::REF_SWAP_ACTION_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// Change helper: perform a swap (or routed multi-hop swap) on the
/// rhea / ref-finance DEX using raw JSON args.
///
/// `actions` is an ordered list of one or more
/// [`REF_SWAP_ACTION_TYPE`] steps — for a single-pool swap, pass a
/// one-element slice; for a routed hop, chain actions in order.
///
/// `referral_id` is the optional NEAR account credited as the
/// referrer. Pass `""` for none.
///
/// No deposit is attached — the swap routes user tokens already held
/// by the contract under the signer's storage registration.
pub async fn swap(
    near: &Near,
    rhea_contract_id: &str,
    actions: &[REF_SWAP_ACTION_TYPE],
    referral_id: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let result = near
        .call(rhea_contract_id, RHEA_METHODS_CONST.swap)
        .args(json!({
            "actions": actions,
            "referral_id": referral_id,
        }))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
