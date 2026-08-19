// use near_kit::*;
use crate::lib::methods::methods_pumpopoly_nft::PUMPOPOLY_NFT_METHODS_CONST;
use near_kit::{AccountId, Error, Near};
use serde_json::json;
// =================================================
/// View helper: count how many Pumpopoly NFTs `account_id` holds, using
/// raw JSON args.
///
/// The contract returns the count as a `U64` JSON string (e.g. `"104"`),
/// so it is parsed into a `u64` here.
pub async fn nft_supply_for_owner(
    near: &Near,
    nft_contract_id: &str,
    account_id: &str,
) -> Result<u64, Error> {
    let account: AccountId = account_id.parse()?;
    let supply: String = near
        .view::<String>(
            nft_contract_id,
            PUMPOPOLY_NFT_METHODS_CONST.nft_supply_for_owner,
        )
        .args(json!({ "account_id": account }))
        .await?;
    let supply: u64 = supply.parse().map_err(|_| {
        Error::Config(format!(
            "nft_supply_for_owner returned non-numeric `{supply}`"
        ))
    })?;
    Ok(supply)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
