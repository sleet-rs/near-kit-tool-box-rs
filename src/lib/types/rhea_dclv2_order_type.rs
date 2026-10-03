use serde::Deserialize;
// =================================================
/// Best-effort shape of a DCL v2 user order (`UserOrderInfo` in rhea-sdk).
#[derive(Debug, Clone, Deserialize)]
pub struct RHEA_DCLV2_ORDER_TYPE {
    pub order_id: String,
    pub owner_id: String,
    pub pool_id: String,
    pub point: i32,
    pub amount: String,
    pub is_bid: bool,
    #[serde(flatten, default)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
