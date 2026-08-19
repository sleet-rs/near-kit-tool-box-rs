use std::collections::HashMap;
// =========================================
/// Shape returned by the rhea / ref-finance `get_deposits` view call.
///
/// The contract returns a `HashMap<token_id, balance>` where
/// `token_id` is a NEP-141 contract id (e.g. `wrap.near`,
/// `usdt.tether-token.near`, `shit-1170.meme-cooking.near`) and
/// `balance` is the user's **deposited balance of that NEP-141
/// token** held by the rhea contract on the user's behalf, as a
/// decimal string in the token's smallest unit.
pub type REF_GET_DEPOSITS_TYPE = HashMap<String, String>;
// =========================================
// =========================================
// copyright 2026 by sleet.near
