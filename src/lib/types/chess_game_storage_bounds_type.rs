use serde::Deserialize;
// =================================================
/// The NEP-145 storage balance bounds of the app.chess-game.near contract.
///
/// Returned by the `storage_balance_bounds` view. The `min` is the exact
/// deposit a caller must attach to `storage_deposit` to register, and
/// `max` is `null` when the contract sets no upper bound.
///
/// Verified against live `app.chess-game.near` responses on 2026-11-11:
/// `min` is `"50000000000000000000000"` (0.05 NEAR).
#[derive(Debug, Clone, Deserialize)]
pub struct CHESS_GAME_STORAGE_BOUNDS_TYPE {
    /// The maximum storage balance, in yoctoNEAR, or `null` for no bound.
    #[serde(default)]
    pub max: Option<String>,
    /// The minimum storage balance, in yoctoNEAR.
    pub min: String,
}
// =================================================
/// The ppp token metadata of the app.chess-game.near contract.
///
/// Returned by the `ft_metadata` view. The symbol is `PPP`, the decimals
/// are `6` and the name is `Protocol Pawns Points`.
///
/// Verified against live `app.chess-game.near` responses on 2026-11-11.
/// The `icon` is a data URL and is left out of this shape on purpose —
/// it is large and rarely needed.
#[derive(Debug, Clone, Deserialize)]
pub struct CHESS_GAME_FT_METADATA_TYPE {
    /// The token spec, `ft-1.0.0`.
    pub spec: String,
    /// The token name, `Protocol Pawns Points`.
    pub name: String,
    /// The token symbol, `PPP`.
    pub symbol: String,
    /// The token decimals, `6`.
    pub decimals: u64,
    #[serde(default)]
    pub reference: Option<String>,
    #[serde(default)]
    pub reference_hash: Option<String>,
    #[serde(flatten, default)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
