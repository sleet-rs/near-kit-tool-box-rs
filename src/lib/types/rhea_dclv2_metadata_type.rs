use serde::Deserialize;
// =================================================
/// Shape returned by the DCL v2 `get_metadata` view call.
///
/// Verified against live `dclv2.ref-labs.near` responses on 2026-10-28.
/// The live response has 12 fields. `pool_count` and `user_count` are
/// JSON strings, while `protocol_fee_rate` is a JSON number.
///
/// Unknown contract fields are preserved in `extra`.
#[derive(Debug, Clone, Deserialize)]
pub struct RHEA_DCLV2_METADATA_TYPE {
    /// The deployed contract version, for example `"2.4.0"`.
    pub version: String,
    /// The account that currently owns the contract.
    pub owner_id: String,
    /// The farming contract the DEX points at.
    pub farming_contract_id: String,
    /// The account nominated to take ownership next, when there is one.
    #[serde(default)]
    pub next_owner_id: Option<String>,
    /// The deadline for the pending owner to accept, when there is one.
    #[serde(default)]
    pub next_owner_accept_deadline: Option<String>,
    /// The wNEAR contract id.
    pub wnear_id: String,
    /// The contract state, for example `"Running"`.
    pub state: String,
    /// The accounts allowed to operate the contract.
    #[serde(default)]
    pub operators: Vec<String>,
    /// The protocol fee rate, in hundredths of a percent.
    pub protocol_fee_rate: u32,
    /// The number of registered pools, as a decimal string.
    pub pool_count: String,
    /// The number of registered users, as a decimal string.
    pub user_count: String,
    #[serde(flatten, default)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
