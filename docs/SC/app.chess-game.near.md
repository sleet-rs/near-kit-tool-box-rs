# app.chess-game.near

the app.chess-game.near on-chain chess smart contract — play, wager, matchmaking, AI, quests, achievements, and ELO

---

#### DETAILS

**CONTRACT ID PER NETWORK**
- MAINNET: app.chess-game.near
- TESTNET: (none — mainnet only)

```sh
# near cli rs - get a list of methods
near contract inspect app.chess-game.near network-config mainnet now
```

**THE GAME ID**

a game is identified by a `GameId`, a tuple of three values:

```json
[128903456, "player1.near", "player2.near"]
```

- the first value is the block height the game was created at
- the second is the white player, always an account id
- the third is the black player, or `null` for a game against the ai

in Rust this is `CHESS_GAME_ID_TYPE` — a `(u64, String, Option<String>)`
tuple, which serializes to exactly that json array.

**MOVES ARE COORDINATE NOTATION**

`play_move` takes **coordinate notation**, not san. `"Nf3"` and `"Qxe4"`
fail. The accepted forms are `"e2e4"`, `"e2 e4"`, `"e2 to e4"`, `"O-O"`,
`"O-O-O"` and a promotion like `"e7 to e8 Q"`. The columns are a–h and
the rows are 1–8, counted from white's side.

**GAS AND DEPOSITS**

- every **view** is free and takes no deposit
- a **contract method** takes no deposit, except the storage methods
  and the token methods:
  - `storage_deposit` attaches at least **0.05 NEAR**, the storage cost
  - `storage_withdraw`, `set_is_agent` and `withdraw_token` attach exactly
    **1 yoctoNEAR**
  - `join_matchmaking` attaches **0.01 NEAR**, the storage for the queue
    entry
- an **`ft_transfer_call`** attaches exactly 1 yoctoNEAR

the gas follows the work the call does:

| flow                    | gas    |
| ----------------------- | ------ |
| `play_move` against Easy | 50    |
| `play_move` against Medium or a human game | 150 |
| `play_move` against Hard | 250   |
| `play_move` against VeryHard | 400 |
| `create_ai_game` against Easy | 50 |
| `create_ai_game` against Medium | 150 |
| `create_ai_game` against Hard | 250 |
| `create_ai_game` against VeryHard | 400 |
| `resign`, `cancel` | 300 |
| the other calls | 30 |

in Rust the ai difficulty carries its own gas — `CHESS_GAME_DIFFICULTY`
has a `gas_tgas()` method, and `play_move_with_gas` picks the right value
for you (`None` for a human game, 150 Tgas).

**REGISTRATION IS THE FIRST STEP**

an account must be registered on the contract before it can play, challenge,
join the matchmaking queue, or bet. `storage_deposit` with at least 0.05
NEAR does it, and it is done once. A new account starts with an elo rating
of 1000, no points, and every stat at zero.

**THE LIMITS**

| limit                            | value |
| -------------------------------- | ----- |
| active games per account         | 5     |
| open challenges per account      | 25    |
| open bets per bettor             | 10    |
| bets per game                    | 250   |
| matchmaking queue entries        | 200   |
| blocks before a player may cancel their own game | ~3 days |
| blocks before anyone may cancel a game | ~14 days |

a cancel is only allowed when it is not the caller's turn.

**THE PPP POINTS**

the contract is also a non-transferable ppp token. The symbol is `PPP`, the
decimals are `6`, and the name is `Protocol Pawns Points`. `ft_transfer`
and `ft_transfer_call` on the ppp side panic — the points cannot move.
Points are earned from quests and achievements, and are claimed with
`claim_points`.

**THE `#[private]` SETTLEMENT CALLBACKS**

the contract carries a set of settlement callbacks — `wager_victory_callback`,
`wager_stalemate_callback`, `cancel_wager_refund_callback`,
`reject_challenge_wager_callback`, `withdraw_token_callback` and
`withdraw_treasury_callback`. they are `#[private]`, they are called by the
contract itself, and they are not for a caller to invoke. they are
**not** in `CHESS_GAME_METHODS_CONST`.

**Typed view responses**
- `get_board` → `Vec<String>`, eight rows, row 0 is rank 8
- `render_board` → `String`, the same board rendered with unicode pieces
- `game_info` → `CHESS_GAME_INFO_TYPE`
- `get_game_ids` → `Vec<CHESS_GAME_ID_TYPE>`
- `get_account` → `CHESS_GAME_ACCOUNT_TYPE`
- `get_accounts` → `Vec<String>`
- `get_elo_ratings` / `get_elo_ratings_by_ids` → `Vec<CHESS_GAME_ELO_RATING_TYPE>`
- `storage_balance_bounds` → `CHESS_GAME_STORAGE_BOUNDS_TYPE`
- `storage_balance_of` → `STORAGE_BALANCE_OF_RESPONSE_TYPE`
- `get_owner` → `String`
- `get_fees` → `u32`, in basis points
- `get_token_whitelist` → `Vec<String>`

`CHESS_GAME_PLAYER_TYPE` is the tagged side of a game — `{ "type": "Human",
"value": "player1.near" }` for a human, `{ "type": "Ai", "value": "Hard" }`
for the ai.

All of the above shapes were verified against live `app.chess-game.near`
responses on 2026-11-11, with the `extra` catch-all empty on every
response.

#### FILES

- `src/lib/methods/methods_chess_game.rs`
- `src/lib/const_id/chess_game_contract_id_const.rs`
- `src/lib/types/chess_game_id_type.rs`, `chess_game_account_type.rs`, `chess_game_info_type.rs`, `chess_game_elo_rating_type.rs`, `chess_game_storage_bounds_type.rs`
- `src/fun/chess_game/` — the contract methods:
  - views — `chess_game_get_board_fun_json.rs`, `chess_game_render_board_fun_json.rs`, `chess_game_game_info_fun_json.rs`, `chess_game_get_game_ids_fun_json.rs`, `chess_game_get_account_fun_json.rs`, `chess_game_get_accounts_fun_json.rs`, `chess_game_get_elo_ratings_fun_json.rs`, `chess_game_get_elo_ratings_by_ids_fun_json.rs`, `chess_game_get_contract_info_fun_json.rs`, `chess_game_storage_balance_fun_json.rs`
  - calls — `chess_game_storage_deposit_fun_json.rs`, `chess_game_create_ai_game_fun_json.rs`, `chess_game_play_move_fun_json.rs`, `chess_game_game_end_fun_json.rs`, `chess_game_challenge_fun_json.rs`, `chess_game_matchmaking_fun_json.rs`, `chess_game_account_calls_fun_json.rs`
- `src/bin/chess_game_get_board_bin_json.rs`, `chess_game_game_info_bin_json.rs`, `chess_game_get_account_bin_json.rs`

---

#### EXAMPLE — read a game

```sh
# the board, plain and rendered
cargo run --bin chess_game_get_board_bin_json -- '[113689871,"within4d45.near","marior.near"]'
# -> rank 8: RNBQKBNR
# -> rank 7: PPPPPPPP
# ...
# Rendered board:
# 8♜♞♝♛♚♝♞♜
# 7♟♟♟♟♟♟♟♟

# the players, the turn and the bet flag
cargo run --bin chess_game_game_info_bin_json -- '[113689871,"within4d45.near","marior.near"]'
# -> white: { player_type: "Human", value: "within4d45.near" }
# -> black: { player_type: "Human", value: "marior.near" }
# -> turn_color: "White"

# an account, with its active games
cargo run --bin chess_game_get_account_bin_json -- marior.near
# -> elo: 969.548032325438
# -> wins: 3
# Active games:
#   (113689871, "within4d45.near", Some("marior.near"))
```

the write helpers need a signer, so they are not in the bins yet. the
calling shape is:

```rust
use near_kit_tool_box::fun::chess_game::chess_game_create_ai_game_fun_json::{
    create_ai_game, CHESS_GAME_DIFFICULTY,
};
use near_kit_tool_box::fun::chess_game::chess_game_play_move_fun_json::play_move;

let game_id = (113689871u64, "within4d45.near".to_string(), None);
create_ai_game(&near, CHESS_GAME_CONTRACT_ID_CONST, CHESS_GAME_DIFFICULTY::Easy).await?;
play_move(&near, CHESS_GAME_CONTRACT_ID_CONST, &game_id, "e2e4", 50).await?;
```

==========================
<br/>
copyright 2026 by sleet.near
