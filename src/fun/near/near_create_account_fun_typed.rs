// use near_kit::*;
use crate::lib::const_id::near_contract_id_const::near_contractid;
use crate::lib::contract::contract_near::{
    NEAR_CREATE_ACCOUNT_ARGS, NEAR_TLD_CONTRACT_TRAIT,
};
use near_kit::{Error, Gas, Near, NearToken};
use near_kit::types::AccountId;
// =================================================
/// Create a new sub-account via the TLD registrar contract using the
/// typed contract interface.
///
/// The TLD contract id is resolved from the client's `chain_id`:
/// `near` on mainnet, `testnet` on testnet. The registrar creates
/// the sub-account and assigns the supplied public key as its
/// initial full-access key.
///
/// Requires a `Near` client configured with credentials for the
/// signer that pays for the transaction.
pub async fn near_create_account(
    near: &Near,
    new_account_id: &str,
    new_public_key: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let tld_contract_id = match near.chain_id().is_mainnet() {
        true => near_contractid("mainnet"),
        false => near_contractid("testnet"),
    };

    let new_account_id: AccountId = new_account_id
        .parse()
        .map_err(|e| Error::Config(format!("invalid account id `{new_account_id}`: {e}")))?;

    let tld = near.contract::<NEAR_TLD_CONTRACT_TRAIT>(tld_contract_id);

    let result = tld
        .create_account(NEAR_CREATE_ACCOUNT_ARGS {
            new_account_id,
            new_public_key: new_public_key.to_string(),
        })
        .gas(Gas::from_tgas(30))
        .deposit(NearToken::from_yoctonear(0))
        .await?;

    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near