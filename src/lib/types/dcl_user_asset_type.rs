use serde::Deserialize;
// =================================================
/// Best-effort shape of a DCL v2 user asset entry.
#[derive(Debug, Clone, Deserialize)]
pub struct DCL_USER_ASSET_TYPE {
    pub token_id: String,
    pub balance: String,
    #[serde(flatten, default)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
