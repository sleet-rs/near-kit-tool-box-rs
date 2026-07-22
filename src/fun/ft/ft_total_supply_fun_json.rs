// use near_kit::*;
use crate::lib::methods::methods_ft::FT_METHODS_CONST;
use near_kit::{Error, Near};
// =================================================
/// View helper: fetch the FT total supply using raw JSON args.
///
/// Returns the supply as a decimal string in the token's smallest
/// unit.
pub async fn ft_total_supply(near: &Near, ft_contract_id: &str) -> Result<String, Error> {
    let supply: String = near
        .view::<String>(ft_contract_id, FT_METHODS_CONST.ft_total_supply)
        .args(serde_json::json!({}))
        .await?;
    Ok(supply)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
