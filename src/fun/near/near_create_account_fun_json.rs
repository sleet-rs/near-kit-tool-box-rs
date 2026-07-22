// use near_kit::*;
use crate::lib::methods::methods_near::NEAR_METHODS_CONST;
use near_kit::{Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// Create a new sub-account via the `near` / `testnet` TLD registrar
/// contract using raw JSON args.
///
/// Calls the `create_account` method on the TLD contract (e.g. `near`
/// on mainnet, `testnet` on testnet). The registrar creates the
/// sub-account and assigns the supplied public key as its initial
/// full-access key.
///
/// Requires a `Near` client configured with credentials for the
/// signer that pays for the transaction.
pub async fn near_create_account(
    near: &Near,
    near_tld_contract_id: &str,
    new_account_id: &str,
    new_public_key: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let result = near
        .call(near_tld_contract_id, NEAR_METHODS_CONST.create_account)
        .args(json!({
            "new_account_id": new_account_id,
            "new_public_key": new_public_key,
        }))
        .gas(Gas::from_tgas(30))
        .deposit(NearToken::from_yoctonear(0))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
