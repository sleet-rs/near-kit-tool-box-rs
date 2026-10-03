use serde::Deserialize;
// =================================================
/// Shape returned by the DCL v2 `predict_add_liquidity` view call.
#[derive(Debug, Clone, Deserialize)]
pub struct RHEA_DCLV2_PREDICT_ADD_LIQUIDITY_TYPE {
    pub liquidity_amount: String,
    pub need_x: String,
    pub need_y: String,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
