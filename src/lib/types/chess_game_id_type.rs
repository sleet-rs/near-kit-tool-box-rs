// =================================================
/// The `GameId` of a chess game: a tuple of three values.
///
/// - the first value is the block height the game was created at
/// - the second is the white player, always an account id
/// - the third is the black player, or `null` for a game against the ai
///
/// The contract takes the game id as a JSON array, so this type mirrors
/// that shape exactly.
pub type CHESS_GAME_ID_TYPE = (u64, String, Option<String>);
// =================================================
// =================================================
// copyright 2026 by sleet.near
