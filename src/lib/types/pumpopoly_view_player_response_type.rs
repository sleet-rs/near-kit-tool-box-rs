use serde::Deserialize;
// =================================================
/// Player state returned by the Pumpopoly world's `view_player` method.
#[derive(Debug, Clone, Deserialize)]
pub struct PUMPOPOLY_VIEW_PLAYER_RESPONSE_TYPE {
    pub location: u64,
    pub balance: String,
    pub account_id: String,
    pub insolvent: bool,
    pub invites: u64,
    pub invited_by: Option<String>,
    pub land_count: u64,
    pub last_move: String,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
