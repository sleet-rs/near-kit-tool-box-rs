// view_account (json)
//
// View-call helper: fetch an `AccountView` for the given account id
// at the latest finalised block, using raw JSON args on the
// load-balanced near client. The result is the same `AccountView`
// type returned by `near.account(...)` / `LOAD_BALANCING_NEAR::view_account`.
//
// usage:
//   let view = view_account(&near, "sleet.near").await?;
//
// =================================================
use near_kit::types::{AccountId, AccountView};
use near_kit::{Error, Near};
// =================================================
/// View-call helper: fetch `AccountView` for `account_id` at the
/// latest finalised block.
///
/// Returns the full `AccountView` struct (amount, locked, storage_usage,
/// code_hash, block_height, etc.) so callers can pick the field they
/// need. Read-only; no signer required.
pub async fn view_account(near: &Near, account_id: &str) -> Result<AccountView, Error> {
    let parsed: AccountId = account_id.parse()?;
    near.account(parsed).await
}
// =========================================
// =========================================
// copyright 2026 by sleet.near
