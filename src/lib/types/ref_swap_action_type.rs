use serde::{Deserialize, Serialize};
// =================================================
/// One step of a rhea / ref-finance `swap` action list.
///
/// `amount_in` is optional — when `None`, the contract interprets the
/// action as "swap every available balance of `token_in` I hold on
/// the DEX up to `min_amount_out`".
///
/// `min_amount_out` is the slippage guard, a decimal string in
/// `token_out`'s smallest unit. Use `"0"` to accept any positive
/// amount.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct REF_SWAP_ACTION_TYPE {
    pub pool_id: u32,
    pub token_in: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_in: Option<String>,
    pub token_out: String,
    pub min_amount_out: String,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
