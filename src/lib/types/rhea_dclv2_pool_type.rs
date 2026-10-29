use serde::Deserialize;
// =================================================
/// Shape returned by the DCL v2 `get_pool`, `get_pools`, and `list_pools`
/// view calls.
///
/// Verified against live `dclv2.ref-labs.near` responses on 2026-10-28.
/// A pool id is a composite string in the form
/// `<hash>|<token>|<fee>`, and `token_x` is the hash rather than an
/// account id. The live response has 23 fields; all are named here.
///
/// Fields that may be absent for a particular pool are optional. Unknown
/// contract fields are preserved in `extra` so a contract update does not
/// make deserialization fail.
#[derive(Debug, Clone, Deserialize)]
pub struct RHEA_DCLV2_POOL_TYPE {
    pub pool_id: String,
    pub token_x: String,
    pub token_y: String,
    pub fee: u32,
    #[serde(default)]
    pub point_delta: Option<u16>,
    #[serde(default)]
    pub current_point: Option<i32>,
    #[serde(default)]
    pub liquidity: Option<String>,
    #[serde(default)]
    pub liquidity_x: Option<String>,
    #[serde(default)]
    pub max_liquidity_per_point: Option<String>,
    pub total_x: String,
    pub total_y: String,
    #[serde(default)]
    pub total_liquidity: Option<String>,
    #[serde(default)]
    pub total_order_x: Option<String>,
    #[serde(default)]
    pub total_order_y: Option<String>,
    #[serde(default)]
    pub volume_x_in: Option<String>,
    #[serde(default)]
    pub volume_x_out: Option<String>,
    #[serde(default)]
    pub volume_y_in: Option<String>,
    #[serde(default)]
    pub volume_y_out: Option<String>,
    #[serde(default)]
    pub total_fee_x_charged: Option<String>,
    #[serde(default)]
    pub total_fee_y_charged: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    /// Tokens allowed in the pool, or `null` when the pool is open.
    #[serde(default)]
    pub whitelist: Option<Vec<String>>,
    /// The tax charged by the pool, or `null` when it charges none.
    #[serde(default)]
    pub tax: Option<String>,
    #[serde(flatten, default)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
