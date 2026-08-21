// use near_kit::*;
use near_kit::{AccessKey, AccessKeyPermission, Action, DelegateResult, Error, Near, PublicKey};
// =================================================
/// Build and sign a meta-transaction (NEP-366) that adds `public_key`
/// to `target_account_id`. The returned [`DelegateResult`] can be
/// transported to a relayer (off-chain) and submitted via
/// [`submit_add_key_meta`] — the relayer pays gas and storage on
/// behalf of `target_account_id`.
///
/// `near` MUST have credentials set for `target_account_id` using one
/// of that account's full-access private keys (the on-chain access key
/// the delegation is bound to). The relayer is then free to be any
/// funded account and does not need to be related to the targets.
///
/// `permission` controls whether the new key is full-access or
/// restricted to a single receiver / method set (function-call key).
pub async fn sign_add_key_meta(
    near: &Near,
    target_account_id: &str,
    public_key: &str,
    permission: AccessKeyPermission,
) -> Result<DelegateResult, Error> {
    let pk: PublicKey = public_key.parse()?;
    let action = Action::AddKey(near_kit::AddKeyAction {
        public_key: pk,
        access_key: AccessKey {
            nonce: 0,
            permission,
        },
    });
    let result = near
        .transaction(target_account_id)
        .add_action(action)
        .delegate(Default::default())
        .await?;
    Ok(result)
}
// =================================================
/// Wrap a previously signed [`DelegateResult`] (built by
/// [`sign_add_key_meta`]) in an outer transaction signed by
/// `relayer_near` and submit it to the chain. The relayer pays for gas
/// and storage on behalf of the inner signer.
pub async fn submit_add_key_meta(
    relayer_near: &Near,
    delegate: DelegateResult,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let signed = delegate.signed_delegate_action;
    let result = relayer_near
        .transaction(signed.sender_id())
        .signed_delegate_action(signed)
        .send()
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
