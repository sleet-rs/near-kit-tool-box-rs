// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use crate::lib::types::storage_balance_of_response_type::STORAGE_BALANCE_OF_RESPONSE_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the NEP-145 storage balance of an account on the
/// DCL v2 contract using raw JSON args.
///
/// Returns `None` when the account has no storage registered, otherwise
/// the standard `{ total, available }` object with both amounts as
/// yoctoNEAR decimal strings.
pub async fn storage_balance_of(
    near: &Near,
    dcl_id: &str,
    account_id: &str,
) -> Result<Option<STORAGE_BALANCE_OF_RESPONSE_TYPE>, Error> {
    let out: Option<STORAGE_BALANCE_OF_RESPONSE_TYPE> = near
        .view::<Option<STORAGE_BALANCE_OF_RESPONSE_TYPE>>(
            dcl_id,
            RHEA_DCLV2_METHODS_CONST.storage_balance_of,
        )
        .args(json!({ "account_id": account_id }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
