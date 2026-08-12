use serde::Deserialize;
// =================================================
/// NEP-145 storage balance returned by the rhea / ref-finance
/// `storage_balance_of` view call (and any other storage-standard
/// contract).
///
/// All amounts are yoctoNEAR decimal strings.
#[derive(Debug, Clone, Deserialize)]
pub struct STORAGE_BALANCE_OF_RESPONSE_TYPE {
    pub total: String,
    pub available: String,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
