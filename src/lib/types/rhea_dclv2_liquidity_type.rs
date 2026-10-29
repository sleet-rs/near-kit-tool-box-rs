use serde::Deserialize;
// =================================================
/// Shape of a DCL v2 liquidity position (LPT).
///
/// Verified against live `dclv2.ref-labs.near` responses on 2026-10-28.
/// `list_liquidities` answers with `lpt_id`, `owner_id`, `pool_id`,
/// `left_point`, `right_point`, `amount`, `mft_id`, `v_liquidity`,
/// `unclaimed_fee_x`, and `unclaimed_fee_y`.
///
/// `lpt_id` is the pool id plus the owner's liquidity index, for example
/// `nearvember-337.meme-cooking.near|wrap.near|10000#17677`; `pool_id` is
/// the same string without the `#index` suffix. `left_point` and
/// `right_point` are negative for positions below the current point.
///
/// `get_liquidity` and `get_liquidity_view` answer with slightly
/// different fields: the former carries `unclaimed_fee_x` and
/// `unclaimed_fee_y`, while the latter carries `token_x`, `token_y`,
/// `fee_x`, and `fee_y`. Both sets are named here. Unknown fields are
/// preserved in `extra`.
#[derive(Debug, Clone, Deserialize)]
pub struct RHEA_DCLV2_LIQUIDITY_TYPE {
    #[serde(default)]
    pub lpt_id: Option<String>,
    #[serde(default)]
    pub owner_id: Option<String>,
    pub pool_id: String,
    pub left_point: i32,
    pub right_point: i32,
    #[serde(default)]
    pub amount: Option<String>,
    /// The MFT id of the position, or `""` when it is a plain FT.
    #[serde(default)]
    pub mft_id: Option<String>,
    /// The virtual liquidity carried by the position.
    #[serde(default)]
    pub v_liquidity: Option<String>,
    /// Fees the position has not claimed yet.
    #[serde(default)]
    pub unclaimed_fee_x: Option<String>,
    #[serde(default)]
    pub unclaimed_fee_y: Option<String>,
    /// Current token amount on the x side. Only the liquidity views
    /// (`get_liquidity_view`, `list_liquidity_views`) answer with it.
    #[serde(default)]
    pub token_x: Option<String>,
    /// Current token amount on the y side. Only the liquidity views
    /// (`get_liquidity_view`, `list_liquidity_views`) answer with it.
    #[serde(default)]
    pub token_y: Option<String>,
    /// Fees accrued on the x side. Only the liquidity views
    /// (`get_liquidity_view`, `list_liquidity_views`) answer with it.
    #[serde(default)]
    pub fee_x: Option<String>,
    /// Fees accrued on the y side. Only the liquidity views
    /// (`get_liquidity_view`, `list_liquidity_views`) answer with it.
    #[serde(default)]
    pub fee_y: Option<String>,
    #[serde(flatten, default)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
