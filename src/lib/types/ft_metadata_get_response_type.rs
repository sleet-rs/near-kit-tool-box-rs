use serde::Deserialize;
// =================================================
/// NEP-145 FT metadata returned by the `ft_metadata` view call.
#[derive(Debug, Clone, Deserialize)]
pub struct FT_METADATA_GET_RESPONSE_TYPE {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
    pub decimals: u8,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
