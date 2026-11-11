use serde::Deserialize;
// =================================================
/// The registration, rating, points and stats of one chess account.
///
/// Returned by the app.chess-game.near `get_account` view.
///
/// Verified against live `app.chess-game.near` responses on 2026-11-11.
/// All amounts are decimal strings — `near_amount` is in yoctoNEAR and
/// `points` / `pending_points` are in ppp token units (6 decimals).
/// Unknown contract fields are preserved in `extra` so a contract update
/// does not make deserialization fail.
#[derive(Debug, Clone, Deserialize)]
pub struct CHESS_GAME_ACCOUNT_TYPE {
    /// The storage balance the account holds on the contract, in yoctoNEAR.
    pub near_amount: String,
    /// Whether the account is flagged as an automated agent.
    pub is_agent: bool,
    /// The ppp point balance of the account.
    pub points: String,
    /// The ppp points earned but not yet claimed.
    pub pending_points: String,
    /// The elo rating of the account.
    pub elo: f64,
    /// The number of games the account won.
    pub wins: u64,
    /// The current win streak.
    pub win_streak: u64,
    /// The longest win streak the account reached.
    pub max_win_streak: u64,
    /// The number of bets the account placed.
    pub bets_placed: u64,
    /// The number of bets the account won.
    pub bets_won: u64,
    /// The number of wagers the account played.
    pub wagers_played: u64,
    /// The number of wagers the account won.
    pub wager_wins: u64,
    /// The number of challenges the account sent.
    pub challenges_sent: u64,
    #[serde(flatten, default)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
