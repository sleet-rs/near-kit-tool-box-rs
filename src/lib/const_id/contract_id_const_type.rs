// =================================================
/// Shared `testnet` / `mainnet` contract-id container used by every
/// dual-network `*_CONTRACT_ID_CONST_TYPE` in this module.
///
/// Each per-contract const file aliases this type so there is one
/// canonical shape for `(testnet, mainnet)` contract id pairs.
pub struct CONTRACT_ID_CONST_TYPE {
    pub testnet: &'static str,
    pub mainnet: &'static str,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
