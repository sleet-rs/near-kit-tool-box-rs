use serde::Deserialize;
// =================================================
/// Best-effort shape of a DCL v2 liquidity position (LPT).
#[derive(Debug, Clone, Deserialize)]
pub struct RHEA_DCLV2_LIQUIDITY_TYPE {
    pub lpt_id: String,
    pub owner_id: String,
    pub pool_id: String,
    pub left_point: i32,
    pub right_point: i32,
    pub amount_l: String,
    #[serde(flatten, default)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
