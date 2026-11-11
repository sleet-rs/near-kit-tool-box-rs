use serde::Deserialize;
// =================================================
/// One side of a chess game — either a human account or the on-chain ai.
///
/// The contract returns the two sides as a tagged enum:
///
/// ```json
/// { "type": "Human", "value": "player1.near" }
/// ```
///
/// ```json
/// { "type": "Ai", "value": "Hard" }
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct CHESS_GAME_PLAYER_TYPE {
    /// `"Human"` or `"Ai"`.
    #[serde(rename = "type")]
    pub player_type: String,
    /// The account id for a human, or the ai difficulty for the ai.
    pub value: String,
}
// =================================================
/// The players, whose turn it is, and whether the game carries bets.
///
/// Returned by the app.chess-game.near `game_info` view.
///
/// Verified against live `app.chess-game.near` responses on 2026-11-11.
#[derive(Debug, Clone, Deserialize)]
pub struct CHESS_GAME_INFO_TYPE {
    /// The white side — always a human account.
    pub white: CHESS_GAME_PLAYER_TYPE,
    /// The black side — a human account, or the ai.
    pub black: CHESS_GAME_PLAYER_TYPE,
    /// Whose turn it is, `"White"` or `"Black"`.
    pub turn_color: String,
    /// The block height of the last move in the game.
    pub last_block_height: u64,
    /// Whether the game carries bets.
    pub has_bets: bool,
    #[serde(flatten, default)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
