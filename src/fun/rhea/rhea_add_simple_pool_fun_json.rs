// use near_kit::*;
use crate::lib::methods::methods_rhea::RHEA_METHODS_CONST;
use near_kit::{AccountId, Error, Near};
use serde_json::json;
// =================================================
/// Change helper: register a new constant-product (x*y=k) pool on the
/// rhea / ref-finance DEX using raw JSON args.
///
/// `tokens` is the ordered list of token account ids that make up the
/// pool. `fee` is the pool's swap fee in basis points (e.g. `30` for
/// 0.30%). The signer pays the storage cost of the new pool.
pub async fn add_simple_pool(
    near: &Near,
    rhea_contract_id: &str,
    tokens: &[&str],
    fee: u32,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let tokens_parsed: Vec<AccountId> = tokens
        .iter()
        .map(|t| t.parse::<AccountId>())
        .collect::<Result<_, _>>()?;
    let result = near
        .call(rhea_contract_id, RHEA_METHODS_CONST.add_simple_pool)
        .args(json!({
            "fee": fee,
            "tokens": tokens_parsed,
        }))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
