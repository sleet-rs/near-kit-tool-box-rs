// use near_kit::*;
use crate::lib::methods::methods_ft::FT_METHODS_CONST;
use near_kit::{Error, Near};
use serde::Deserialize;
use serde_json::json;
// =================================================
/// NEP-145 FT metadata returned by the `ft_metadata` view call.
#[derive(Debug, Clone, Deserialize)]
pub struct FtMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
    pub decimals: u8,
}
// =================================================
/// View helper: fetch the FT contract metadata using raw JSON args.
pub async fn ft_metadata(near: &Near, ft_contract_id: &str) -> Result<FtMetadata, Error> {
    let metadata: FtMetadata = near
        .view::<FtMetadata>(ft_contract_id, FT_METHODS_CONST.ft_metadata)
        .args(json!({}))
        .await?;
    Ok(metadata)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
