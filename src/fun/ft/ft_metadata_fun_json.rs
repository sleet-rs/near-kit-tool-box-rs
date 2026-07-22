// use near_kit::*;
use crate::lib::methods::methods_ft::FT_METHODS_CONST;
use crate::lib::types::ft_metadata_get_response_type::FT_METADATA_GET_RESPONSE_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the FT contract metadata using raw JSON args.
pub async fn ft_metadata(
    near: &Near,
    ft_contract_id: &str,
) -> Result<FT_METADATA_GET_RESPONSE_TYPE, Error> {
    let metadata: FT_METADATA_GET_RESPONSE_TYPE = near
        .view::<FT_METADATA_GET_RESPONSE_TYPE>(ft_contract_id, FT_METHODS_CONST.ft_metadata)
        .args(json!({}))
        .await?;
    Ok(metadata)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
