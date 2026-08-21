# world.pumpopoly.near

the pumpopoly world.pumpopoly near monopoly-style game smart contract

---

#### DETAILS

**CONTRACT ID**
- MAINNET: world.pumpopoly.near
- TESTNET: (none — mainnet only)

```sh
# near cli rs - get a list of methods
near contract inspect world.pumpopoly.near network-config mainnet now
```



#### FILES

- `src/lib/methods/methods_pumpopoly_world.rs`
- `src/lib/const_id/pumpopoly_world_contract_id_const.rs`
- `src/lib/types/pumpopoly_view_player_response_type.rs`
- `src/fun/pumpopoly/pumpopoly_move_player_fun_json.rs`
- `src/fun/pumpopoly/pumpopoly_move_player_meta_fun_json.rs`
- `src/fun/pumpopoly/pumpopoly_elite_move_fun_json.rs`
- `src/fun/pumpopoly/pumpopoly_view_player_fun_json.rs`
- `src/fun/pumpopoly/pumpopoly_view_players_fun_json.rs`
- `src/bin/pumpopoly_view_player_bin_json.rs`
- `src/bin/pumpopoly_view_players_bin_json.rs`
- `src/bin/pumpopoly_move_player_meta_bin_json.rs`


#### BINS

View bins only need `NEAR_NETWORK`; the meta-tx bin also needs
`NEAR_ACCOUNT_ID` + `NEAR_PRIVATE_KEY` (the signer / limited key) plus
`RELAYER_ACCOUNT_ID` + `RELAYER_PRIVATE_KEY` (the funded relayer).

```bash
## view_player (json) — single player's state from the mainnet world contract
cargo run --bin pumpopoly_view_player_bin_json -- <account_id>
cargo run --bin pumpopoly_view_player_bin_json -- sleet.near

## view_players (json) — multiple players in a single view call
cargo run --bin pumpopoly_view_players_bin_json -- <account_id> [<account_id> ...]
cargo run --bin pumpopoly_view_players_bin_json -- sleet.near narkmeta.near

## move_player_meta — meta-tx (NEP-366): relayer pays gas
# Calls `move_player` on the mainnet world contract. The signer key
# (typically the shared `PUMPOPOLY_LIMITED_KEY` function-call key on
# the player account) signs a DelegateAction; a funded relayer account
# submits it on the player's behalf.
cargo run --bin pumpopoly_move_player_meta_bin_json -- <pumpopoly_contract_id>
cargo run --bin pumpopoly_move_player_meta_bin_json -- world.pumpopoly.near
```


==========================
<br/>
copyright 2026 by sleet.near