use serde::Deserialize;
// =================================================
/// Shape returned by the DCL v2 `get_pool` and `get_pools` view calls.
#[derive(Debug, Clone, Deserialize)]
pub struct RHEA_DCLV2_POOL_TYPE {
    pub pool_id: String,
    pub token_x: String,
    pub token_y: String,
    pub fee: u32,
    pub point_delta: u16,
    pub current_point: i32,
    pub liquidity: String,
    pub liquidity_x: String,
    pub max_liquidity_per_point: String,
    pub total_x: String,
    pub total_y: String,
    pub total_liquidity: String,
    pub total_order_x: String,
    pub total_order_y: String,
    pub volume_x_in: String,
    pub volume_x_out: String,
    pub volume_y_in: String,
    pub volume_y_out: String,
    pub total_fee_x_charged: String,
    pub total_fee_y_charged: String,
    pub state: String,
    pub whitelist: Option<serde_json::Value>,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
