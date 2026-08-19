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
///
/// Notes:
/// - The map is NOT a per-pool view; it collapses every pool the
///   user has touched into the underlying token's balance. So
///   `wrap.near` in the response is the total wrap.near the user
///   has deposited across every pool, not the user's LP share
///   balance for any one pool. (For per-pool LP shares use
///   `mft_balance_of` on the contract with the inner token id, which
///   in rhea is the numeric pool id as a string.)
/// - Entries with `"0"` are included for tokens the user does not
///   actually have a balance for, so callers should filter them out
///   before rendering.
pub type REF_GET_DEPOSITS_TYPE = HashMap<String, String>;
// =========================================
// =========================================
// copyright 2026 by sleet.near
