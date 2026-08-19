use std::collections::HashMap;
// =================================================
/// Shape returned by the rhea / ref-finance `get_deposits` view call.
///
/// The contract returns a map keyed by `token_id` (the NEP-141 contract
/// id of one side of a pool, e.g. `wrap.near`) whose value is the user's
/// **total** LP share balance across every pool that contains that
/// token. Balances of `0` are included for tokens the user does not
/// actually hold LP for, so callers should filter them out.
pub type REF_GET_DEPOSITS_TYPE = HashMap<String, String>;
// =================================================
// =================================================
// copyright 2026 by sleet.near
