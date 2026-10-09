// use near_kit::*;
use crate::lib::methods::methods_rhea_ammv2::RHEA_AMMV2_METHODS_CONST;
use crate::lib::types::rhea_ammv2_swap_action_type::RHEA_AMMV2_SWAP_ACTION_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// Change helper: perform a swap (or routed multi-hop swap) on the
/// rhea_ammv2 / ref-finance DEX using raw JSON args.
///
/// `actions` is an ordered list of one or more
/// [`RHEA_AMMV2_SWAP_ACTION_TYPE`] steps — for a single-pool swap, pass a
/// one-element slice; for a routed hop, chain actions in order.
///
/// `referral_id` is the optional NEAR account credited as the
/// referrer. Pass `""` for none.
///
/// No deposit is attached — the swap routes user tokens already held
/// by the contract under the signer's storage registration.
pub async fn swap(
    near: &Near,
    rhea_ammv2_contract_id: &str,
    actions: &[RHEA_AMMV2_SWAP_ACTION_TYPE],
    referral_id: &str,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(rhea_ammv2_contract_id, RHEA_AMMV2_METHODS_CONST.swap)
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
