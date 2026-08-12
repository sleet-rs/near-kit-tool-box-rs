// use near_kit::*;
use crate::lib::methods::methods_rhea::RHEA_METHODS_CONST;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: simulate a swap on the rhea / ref-finance DEX using
/// raw JSON args, returning the expected amount of `token_out` you
/// would receive for `amount_in` of `token_in` on `pool_id`.
///
/// Both `token_in` / `token_out` are account ids (typically the
/// inner token contracts for mft pools, e.g. `wrap.near`,
/// `17208628f84f715d082c5c492b05d3e2cae5bd9d.near`, ...). `amount_in`
/// is in yocto units of `token_in` (10^decimals). Returns the quoted
/// output amount as a yoctoNEAR decimal string of `token_out`.
pub async fn get_return(
    near: &Near,
    rhea_contract_id: &str,
    pool_id: u32,
    token_in: &str,
    amount_in: &str,
    token_out: &str,
) -> Result<String, Error> {
    let amount_out: String = near
        .view::<String>(rhea_contract_id, RHEA_METHODS_CONST.get_return)
        .args(json!({
            "pool_id": pool_id,
            "token_in": token_in,
            "amount_in": amount_in,
            "token_out": token_out,
        }))
        .await?;
    Ok(amount_out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
