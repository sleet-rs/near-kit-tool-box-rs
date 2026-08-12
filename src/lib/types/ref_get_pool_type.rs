use serde::Deserialize;
// =================================================
/// Shape returned by the rhea / ref-finance `get_pool` and `get_pools`
/// view calls.
#[derive(Debug, Clone, Deserialize)]
pub struct REF_GET_POOL_TYPE {
    pub pool_kind: String,
    pub token_account_ids: Vec<String>,
    pub amounts: Vec<String>,
    pub total_fee: u32,
    pub shares_total_supply: String,
    pub amp: u64,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
