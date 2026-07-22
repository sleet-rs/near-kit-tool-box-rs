// use near_kit::*;
use near_kit::{AccessKey, AccessKeyPermission, Action, Error, Near, PublicKey};
// =================================================
/// Add an access key to the signer's account using the NEAR
/// transaction-builder API.
///
/// `permission` controls whether the key is full access or a
/// function-call key restricted to a single receiver/method set.
pub async fn add_key(
    near: &Near,
    signer_id: &str,
    public_key: &str,
    permission: AccessKeyPermission,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let pk: PublicKey = public_key.parse()?;
    let action = Action::AddKey(near_kit::AddKeyAction {
        public_key: pk,
        access_key: AccessKey {
            nonce: 0,
            permission,
        },
    });
    let result = near
        .transaction(signer_id)
        .add_action(action)
        .send()
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
