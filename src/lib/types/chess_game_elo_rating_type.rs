// =================================================
/// One account id and elo rating pair.
///
/// Returned by the app.chess-game.near `get_elo_ratings` and
/// `get_elo_ratings_by_ids` views as a tuple.
///
/// Verified against live `app.chess-game.near` responses on 2026-11-11.
/// A new account starts with a rating of 1000.
pub type CHESS_GAME_ELO_RATING_TYPE = (String, f64);
// =================================================
/// One account id and ppp balance pair.
///
/// Returned by the app.chess-game.near `get_ppp_balances_by_ids` view
/// as a tuple. The balance is a decimal string in ppp units.
///
/// Verified against live `app.chess-game.near` responses on 2026-11-11.
pub type CHESS_GAME_PPP_BALANCE_TYPE = (String, String);
// =================================================
// =================================================
// copyright 2026 by sleet.near
