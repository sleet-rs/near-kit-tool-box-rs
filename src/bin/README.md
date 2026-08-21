# Bins

All bins sign / read via `NEAR_KIT_CLIENT` (see `src/lib/client_kit.rs`).
Auth comes from env vars — copy `.env.example` to `.env` and fill in:

```env
NEAR_NETWORK=testnet        # or mainnet
NEAR_ACCOUNT_ID=alice.testnet
NEAR_PRIVATE_KEY=ed25519:YOUR_PRIVATE_KEY_HERE
```

```sh
source .env
```

## src/fun/greeting

Read bins only need `NEAR_NETWORK` (defaults to `testnet`). Set bins also need `NEAR_ACCOUNT_ID` and `NEAR_PRIVATE_KEY`.

```bash
## get greeting (typed)
# Uses the typed contract wrapper (`get_greeting`).
cargo run --bin greeting_get_bin_typed -- <contract_id>
cargo run --bin greeting_get_bin_typed -- hello.sleet.testnet

## get greeting (json)
# Same read, but via raw JSON (`get_greeting`). Useful when you don't want the typed wrapper.
cargo run --bin greeting_get_bin_json -- <contract_id>
cargo run --bin greeting_get_bin_json -- hello.sleet.testnet

## get greeting (balanced)
# Round-robins `get_greeting` across the default testnet endpoints and
# logs which RPC served each call. Default 5 calls; pass a count to override.
cargo run --bin greeting_get_balanced_bin_json -- <contract_id> [times]
cargo run --bin greeting_get_balanced_bin_json -- hello.sleet.testnet 5

## set greeting (typed)
# Uses the typed contract wrapper (`set_greeting`).
cargo run --bin greeting_set_bin_typed -- <contract_id> "<new_text>"
cargo run --bin greeting_set_bin_typed -- hello.sleet.testnet "hello from near kit toolbox rust"

## set greeting (json)
# Same call, but via raw JSON (`set_greeting`). Useful when you don't want the typed wrapper.
cargo run --bin greeting_set_bin_json -- <contract_id> "<new_text>"
cargo run --bin greeting_set_bin_json -- hello.sleet.testnet "hello from near kit toolbox rust"
```


## generic NEAR protocol views

Read-only — only `NEAR_NETWORK` is needed.

```bash
## view_account — account balance, storage, deployed contract info
cargo run --bin view_account_bin_json -- <account_id>
cargo run --bin view_account_bin_json -- hello.sleet.testnet
cargo run --bin view_account_bin_json -- hello.sleet.near

## view_access_key_list — all access keys on an account with permissions
cargo run --bin view_access_key_list_bin_json -- <account_id>
cargo run --bin view_access_key_list_bin_json -- sleet.testnet
```


## src/fun/near

Near-level account / key actions and `wrap.near` helpers. All bins
need `NEAR_ACCOUNT_ID` and `NEAR_PRIVATE_KEY` except view-style ones.

```bash
## create_account via the TLD registrar (contract call)
cargo run --bin near_create_account_bin_json -- <tld> <new_account_id> <new_public_key>
cargo run --bin near_create_account_bin_json -- testnet newacct.testnet ed25519:56CJx1YrTJ6nBmJb8DoXwBm5HLHZcoyAt8FQDmcvkjFE

## add_key (full access)
cargo run --bin add_key_bin_json -- <public_key>
cargo run --bin add_key_bin_json -- ed25519:HDaBzemF6CYnQ2A3CgAh8vK7stWp5aXvKMKzKDe6s3QZ

## add_key (function call, restricted)
cargo run --bin add_key_bin_json -- <public_key> fc <receiver> <methods> <allowance>
cargo run --bin add_key_bin_json -- ed25519:9vnVSMT1hv2Q1vuZzbjStFieqGbTrwqe4KcvfqYAkb5T fc wrap.testnet "near_deposit,near_withdraw" "1 NEAR"

## delete_key
cargo run --bin delete_key_bin_json -- <public_key>

## add_key_delegate — meta-tx (NEP-366): relayer pays all fees
# Adds a full-access key to one or more accounts, even ones with zero
# own balance. The shared user key signs a DelegateAction per target;
# a funded relayer account submits each one on the target's behalf.
# Requires `RELAYER_ACCOUNT_ID` + `RELAYER_PRIVATE_KEY` in env in
# addition to `NEAR_PRIVATE_KEY`.
cargo run --bin add_key_delegate_bin_json -- <new_public_key> <account_1> [account_2 ...]
cargo run --bin add_key_delegate_bin_json -- ed25519:2eDMWnKcDt7UQ1xVximcWbd1YKwJbfE7HGPmNZSegjcV walcwarchest.near walcpool.near

## delete_account (refund goes to beneficiary)
cargo run --bin delete_account_bin_json -- <beneficiary>

## wrap NEAR -> wNEAR
cargo run --bin wrap_near_deposit_bin_json -- <wrap_contract_id> <amount>
cargo run --bin wrap_near_deposit_bin_json -- wrap.testnet "1 NEAR"

## unwrap wNEAR -> NEAR (amount is yoctoNEAR)
cargo run --bin wrap_near_withdraw_bin_json -- <wrap_contract_id> <amount_yocto>
cargo run --bin wrap_near_withdraw_bin_json -- wrap.testnet 1000000000000000000000000
```


## src/fun/ft

Generic NEP-141 / NEP-145 fungible-token helpers. View bins only need
`NEAR_NETWORK`; call bins also need `NEAR_ACCOUNT_ID` and
`NEAR_PRIVATE_KEY`.

```bash
## ft_balance_of (view)
cargo run --bin ft_balance_of_bin_json -- <ft_contract_id> <account_id>
cargo run --bin ft_balance_of_bin_json -- wrap.testnet sleet.testnet

## ft_total_supply (view)
cargo run --bin ft_total_supply_bin_json -- <ft_contract_id>
cargo run --bin ft_total_supply_bin_json -- wrap.testnet

## ft_metadata (view)
cargo run --bin ft_metadata_bin_json -- <ft_contract_id>
cargo run --bin ft_metadata_bin_json -- wrap.testnet

## ft_transfer (call)
cargo run --bin ft_transfer_bin_json -- <ft_contract_id> <receiver_id> <amount>
cargo run --bin ft_transfer_bin_json -- wrap.testnet alice.testnet 1000000000000000000000000

## ft_transfer_call (call)
cargo run --bin ft_transfer_call_bin_json -- <ft_contract_id> <receiver_id> <amount> <msg>
cargo run --bin ft_transfer_call_bin_json -- wrap.testnet alice.testnet 1000000000000000000000000 "register"

## ft_storage_deposit (call, attaches 0.00125 NEAR)
cargo run --bin ft_storage_deposit_bin_json -- <ft_contract_id>
cargo run --bin ft_storage_deposit_bin_json -- wrap.testnet

## ft_storage_balance_of (view) — NEP-145 storage registration on an FT contract
cargo run --bin ft_storage_balance_of_bin_json -- <ft_contract_id> <account_id>
cargo run --bin ft_storage_balance_of_bin_json -- wrap.testnet sleet.testnet
cargo run --bin ft_storage_balance_of_bin_json -- usdt.tether-token.near sleet.near
```


## src/fun/rhea

rhea / ref-finance DEX view helpers. All bins are read-only — only
`NEAR_NETWORK` is needed. Defaults to `ref-finance-101.testnet` when
no contract id is passed; pass `v2.ref-finance.near` for mainnet.

```bash
## get_number_of_pools — total pool count on the DEX (json)
cargo run --bin rhea_get_number_of_pools_bin_json
cargo run --bin rhea_get_number_of_pools_bin_json -- v2.ref-finance.near

## get_pool — fetch a single pool by id (json)
cargo run --bin rhea_get_pool_bin_json -- 0
cargo run --bin rhea_get_pool_bin_json -- 0 v2.ref-finance.near
cargo run --bin rhea_get_pool_bin_json -- 0 ref-finance-101.testnet

## get_deposits — every LP position an account holds (json)
cargo run --bin rhea_get_deposits_bin_json -- sleet.testnet
cargo run --bin rhea_get_deposits_bin_json -- sleet.near v2.ref-finance.near

## get_pools — paginated list of pools (json)
cargo run --bin rhea_get_pools_bin_json -- 0 3
cargo run --bin rhea_get_pools_bin_json -- 0 3 v2.ref-finance.near

## get_return — swap quote: amount_out for amount_in on a pool (json)
# `amount_in` is in yocto units of `token_in` (10^decimals).
cargo run --bin rhea_get_return_bin_json -- \
    2794 wrap.near 10000000000000000000000 \
    a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.near v2.ref-finance.near
cargo run --bin rhea_get_return_bin_json -- 0 wrap.testnet 10000000000000000000000 wrap.testnet

## storage_balance_of — NEP-145 storage registration (json, returns null if unregistered)
cargo run --bin rhea_storage_balance_of_bin_json -- sleet.testnet
cargo run --bin rhea_storage_balance_of_bin_json -- sleet.near v2.ref-finance.near
```



## src/fun/pumpopoly

Read-only — only `NEAR_NETWORK` is needed. `world.pumpopoly.near` is mainnet only.

```bash
## view_player (json) — fetch a single player's state
cargo run --bin pumpopoly_view_player_bin_json -- <account_id>
cargo run --bin pumpopoly_view_player_bin_json -- sleet.near

## view_players (json) — fetch multiple players' state in a single view call
cargo run --bin pumpopoly_view_players_bin_json -- <account_id> [<account_id> ...]
cargo run --bin pumpopoly_view_players_bin_json -- sleet.near narkmeta.near
```



## src/lib/load_balancer

`NEAR_KIT_CLIENT` exposes a special **view client** that round-robins calls
across multiple public endpoints with automatic failover. Read-only — do not
use for signing.

```bash
## load_balancer_status — round-robin `status` across default testnet endpoints
cargo run --bin load_balancer_status_bin_json

## greeting_get_balanced — round-robin `get_greeting` with RPC-of-origin logging
cargo run --bin greeting_get_balanced_bin_json -- hello.sleet.testnet 5
```


==========================
<br/>
copyright 2026 by sleet.near