use serde::Deserialize;
// =================================================
/// Shape returned by the DCL v2 `quote` and `quote_by_output` view calls.
#[derive(Debug, Clone, Deserialize)]
pub struct RHEA_DCLV2_QUOTE_TYPE {
    pub amount: String,
    pub tag: Option<String>,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
