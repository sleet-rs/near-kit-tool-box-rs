// use near_kit::*;
use near_kit::{Error, Near, PublicKey};
// =================================================
/// Delete an access key from the signer's account using the NEAR
/// transaction-builder API.
pub async fn delete_key(
    near: &Near,
    signer_id: &str,
    public_key: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let pk: PublicKey = public_key.parse()?;
    let result = near.transaction(signer_id).delete_key(pk).send().await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
