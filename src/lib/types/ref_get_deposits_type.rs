use std::collections::HashMap;
// =================================================
/// Shape returned by the rhea / ref-finance `get_deposits` view call.
///
/// The contract returns a map keyed by `<pool_id>:<token_id>` (the
/// inner MFT token id) whose value is the user's LP share balance as
/// a decimal string.
pub type REF_GET_DEPOSITS_TYPE = HashMap<String, String>;
// =================================================
// =================================================
// copyright 2026 by sleet.near
