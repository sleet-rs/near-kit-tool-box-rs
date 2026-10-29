# dclv2.ref-labs.near

the DCL v2 (discretized concentrated liquidity) dex smart contract — ref finance's
concentrated-liquidity AMM with limit orders

---

#### DETAILS

**CONTRACT ID PER NETWORK**
- MAINNET: dclv2.ref-labs.near
- TESTNET: dclv2.ref-dev.testnet

```sh
# near cli rs - get a list of methods
near contract inspect dclv2.ref-dev.testnet network-config testnet now
near contract inspect dclv2.ref-labs.near network-config mainnet now
```

**IMPORTANT: there is no `swap` method.** Swaps (and every other token inflow)
enter the contract through `ft_transfer_call` on the *input* token contract and
land in `ft_on_transfer`. The accepted `msg` variants are exactly:
`Deposit`, `Swap`, `SwapByOutput`, `SwapByStopPoint`, `LimitOrder`,
`LimitOrderWithSwap`, `HotZap`. Anything else fails with
`E600: invalid msg: unknown variant`.

- deposit internal balance — `msg = "\"Deposit\""` (the JSON string `"Deposit"`,
  quotes included)
- swap — `msg = {"Swap":{"pool_ids":[...],"output_token":...,"min_output_amount":...}}`
- exact-output swap — `msg = {"SwapByOutput":{"pool_ids":[...],"output_token":...,"output_amount":...}}`
- limit order — `msg = {"LimitOrder":{"pool_id":...,"buy_token":...,"point":...,"client_id":...}}`
- limit order + swap — `msg = {"LimitOrderWithSwap":{"pool_id":...,"buy_token":...,"point":...,"client_id":...}}`
- swap to a price point — `msg = {"SwapByStopPoint":{"pool_id":...,"stop_point":...}}`

**The `msg` fields are strict — every one of them is required.** serde rejects a
missing field, so `{"LimitOrder":{...}}` without `client_id` fails with
`E600: invalid msg: Error("missing field `client_id`")`.

Do **not** pass `referral_id: ""`. The contract parses `referral_id` as an
account id, and the empty string fails with
`E600: invalid msg: Error("the account ID is invalid")`. Omit the field
entirely to leave it unset.

**Limit-order gas.** `LimitOrderWithSwap` runs a real swap before parking the
remainder, so it needs far more than the 50 TGas that the generic
`ft_transfer_call` helper attaches — it runs out of gas with
`Exceeded the prepaid gas.` after the swap already succeeded. Use the
`rhea_dclv2::ft_transfer_call::dclv2_limit_order_fun_json::limit_order` helper
(300 TGas) instead.

**LP flow** is *not* a `ft_transfer_call` message: deposit both tokens into the
internal balance first, then call `add_liquidity` directly on the contract.
`add_liquidity` / `remove_liquidity` require `min_amount_x` and `min_amount_y`
and reject any attached deposit.

**Point alignment matters.** `left_point` / `right_point` must be multiples of
the pool's `point_delta` (fee 100→1, 400→8, 2000→40, 10000→200). Unaligned
points fail with `E200: invalid endpoint`.

**Single-sided liquidity.** A range entirely above `current_point` only consumes
`amount_x` (`predict_add_liquidity` returns `need_y = "0"`); a range entirely
below only consumes `amount_y` (`need_x = "0"`). Pass `"0"` for the unused side.

**Storage.** `storage_deposit` needs 0.5 NEAR (the bare `storage_balance_bounds`
minimum fails with E102). `storage_withdraw` / `storage_unregister` attach exactly
1 yoctoNEAR. `add_liquidity` / `remove_liquidity` / `withdraw_asset` attach none.

**The DCL contract must be an FT holder of the token.** Every token inflow goes
`ft_transfer_call` on the token → `ft_on_transfer` on DCL, and the token
contract refuses to transfer to an unregistered receiver. If DCL has no storage
on that token the whole call panics with
`Smart contract panicked: The account <dcl_id> is not registered` and the
tokens are refunded. This is a per-token condition, not a per-account one:
`wrap.testnet` / `usdc.fakes.testnet` have DCL registered, a random meme token
may not. Check first with
`ft_storage_balance_of(<token>, <dcl_id>)` — a `null` means "not registered".
A token can still be deposited if DCL is registered on it, whether or not a
pool exists for it.

**`predict_add_liquidity` currently panics.** The view fails with
`panicked at contracts/dcl/src/dcl/pool.rs:1138:33: attempt to subtract with
overflow` for the pools that were checked. Use it for its intended purpose
only once it is fixed; `add_liquidity` itself works fine.

**Verified error codes**
- `E101` insufficient internal balance (deposit first)
- `E102` storage deposit too small
- `E200` invalid endpoint (unaligned points)
- `E202` illegal point (unaligned, or wrong side of `current_point` for a limit order)
- `E207` liquidity not found
- `E600` invalid msg / unknown variant / missing field / invalid account id

**Typed view responses**
- `get_pool` / `get_pools` / `list_pools` → `RHEA_DCLV2_POOL_TYPE`
- `get_liquidity` / `list_liquidities` / `get_liquidity_view` / `list_liquidity_views` → `RHEA_DCLV2_LIQUIDITY_TYPE`
- `get_user_asset` → `RHEA_DCLV2_USER_ASSET_TYPE` (a bare decimal string, not an object)
- `list_user_assets` → `RHEA_DCLV2_USER_ASSETS_TYPE` (a `{ token_id: balance }` map)
- `get_metadata` → `RHEA_DCLV2_METADATA_TYPE`
- `storage_balance_of` → `Option<STORAGE_BALANCE_OF_RESPONSE_TYPE>`

The pool id is a composite string in the form `<token_x>|<token_y>|<fee>`,
where both token slots are the tokens' plain NEAR account ids. An `lpt_id` is
the pool id followed by the owner's liquidity index, for example
`nearvember-337.meme-cooking.near|wrap.near|10000#17677`. `left_point` and
`right_point` are negative for positions below the current point.

**Limit-order points have a tighter rule than LP points.** A limit order's
`point` must be a multiple of the pool's `point_delta` *and* must sit on the
correct side of `current_point` for the token being sold, otherwise the order
fails with `E202: illegal point`. Selling `token_y` for `token_x` wants a point
**below** `current_point`; selling `token_x` wants one above. A point that is
aligned but on the wrong side still fails with `E202`. Read the live order
points of a pool with `get_marketdepth` (`orders` key) and pick one that
already exists there.

`get_liquidity` and `list_liquidities` answer with `unclaimed_fee_x` and
`unclaimed_fee_y`; `get_liquidity_view` and `list_liquidity_views` answer
with `token_x`, `token_y`, `fee_x`, and `fee_y` instead. Both sets are
named on `RHEA_DCLV2_LIQUIDITY_TYPE`.

`get_metadata` answers with 12 fields: `version`, `owner_id`,
`farming_contract_id`, `next_owner_id`, `next_owner_accept_deadline`,
`wnear_id`, `state`, `operators`, `protocol_fee_rate`, `pool_count`, and
`user_count`. `pool_count` and `user_count` are JSON strings, while
`protocol_fee_rate` is a JSON number.

All of the above shapes were verified against live `dclv2.ref-labs.near`
responses on 2026-10-29 — 3 pools, 7 liquidity positions, 1 metadata
object, and both user-asset views — with the `extra` catch-all empty on
every response.

The remaining DCL v2 views (quotes, predictions, order and depth ranges,
user storage detail) still return raw JSON until their shapes are
confirmed against a live response.


#### FILES

- `src/lib/methods/methods_rhea_dclv2.rs`
- `src/lib/const_id/rhea_dclv2_contract_id_const.rs`
- `src/lib/types/rhea_dclv2_pool_type.rs`, `rhea_dclv2_quote_type.rs`, `rhea_dclv2_predict_add_liquidity_type.rs`, `rhea_dclv2_liquidity_type.rs`, `rhea_dclv2_order_type.rs`, `rhea_dclv2_user_asset_type.rs`, `rhea_dclv2_metadata_type.rs`
- `src/fun/rhea_dclv2/` — the contract methods: `dclv2_get_pool_fun_json.rs`, `dclv2_get_pools_fun_json.rs`, `dclv2_list_pools_fun_json.rs`, `dclv2_get_metadata_fun_json.rs`, `dclv2_quote_fun_json.rs`, `dclv2_quote_by_output_fun_json.rs`, `dclv2_get_liquidity_fun_json.rs`, `dclv2_list_liquidities_fun_json.rs`, `dclv2_get_liquidity_view_fun_json.rs`, `dclv2_list_liquidity_views_fun_json.rs`, `dclv2_get_liquidity_range_fun_json.rs`, `dclv2_get_pointorder_range_fun_json.rs`, `dclv2_get_marketdepth_fun_json.rs`, `dclv2_get_market_depth_list_fun_json.rs`, `dclv2_predict_add_liquidity_fun_json.rs`, `dclv2_predict_remove_liquidity_fun_json.rs`, `dclv2_get_order_fun_json.rs`, `dclv2_find_order_fun_json.rs`, `dclv2_list_active_orders_fun_json.rs`, `dclv2_list_history_orders_fun_json.rs`, `dclv2_get_user_asset_fun_json.rs`, `dclv2_list_user_assets_fun_json.rs`, `dclv2_get_user_storage_detail_fun_json.rs`, `dclv2_storage_balance_of_fun_json.rs`, `dclv2_add_liquidity_fun_json.rs`, `dclv2_remove_liquidity_fun_json.rs`, `dclv2_withdraw_asset_fun_json.rs`, `dclv2_cancel_order_fun_json.rs`, `dclv2_storage_deposit_fun_json.rs`, `dclv2_storage_withdraw_fun_json.rs`, `dclv2_storage_unregister_fun_json.rs`
- `src/fun/rhea_dclv2/ft_transfer_call/` — the flows that are **not** DCL v2 methods, they call `ft_transfer_call` on the token contract instead: `dclv2_deposit_fun_json.rs`, `dclv2_swap_ft_transfer_call_fun_json.rs`, `dclv2_limit_order_fun_json.rs`
- `src/bin/rhea_dclv2_*_bin_json.rs`

---

#### EXAMPLE — single-sided LP below price, then swap, then unwind

Verified live against `dclv2.ref-dev.testnet` on 2026-10-29. Note the points:
`point_delta` for fee 2000 is **40**, and `current_point` is *not* a multiple
of 40 — 288785 — so the aligned points around it are 288760 / 288800.

```sh
# 1) storage (0.5 NEAR)
cargo run --bin rhea_dclv2_storage_deposit_bin_json -- dclv2.ref-dev.testnet

# 2) fund the internal balance (the token must have DCL registered on it)
cargo run --bin rhea_dclv2_deposit_bin_json -- wrap.testnet 100000000000000000000000 dclv2.ref-dev.testnet

# 3) add liquidity below current_point — only amount_y (wrap) is consumed.
#    points must be multiples of point_delta = 40; unaligned -> E200
cargo run --bin rhea_dclv2_add_liquidity_bin_json -- \
  "usdc.fakes.testnet|wrap.testnet|2000" 288560 288760 0 1000000000000000000000 dclv2.ref-dev.testnet
# -> liquidity_added: lpt_id "usdc.fakes.testnet|wrap.testnet|2000#2544"

# 4) quote + swap (no swap method exists — it routes through ft_transfer_call)
cargo run --bin rhea_dclv2_quote_bin_json -- \
  "usdc.fakes.testnet|wrap.testnet|2000" wrap.testnet usdc.fakes.testnet 100000000000000000000000 Auto dclv2.ref-dev.testnet
# -> { amount: "28721617190", tag: "Auto" }
cargo run --bin rhea_dclv2_swap_bin_json -- wrap.testnet 100000000000000000000000 \
  "usdc.fakes.testnet|wrap.testnet|2000" usdc.fakes.testnet 28000000000 dclv2.ref-dev.testnet

# 5) a limit order: the point must be on the correct side of current_point
#    for the token being sold. Selling wrap (token_y) wants a point BELOW.
cargo run --bin rhea_dclv2_limit_order_bin_json -- wrap.testnet 1000000000000000000000 \
  "usdc.fakes.testnet|wrap.testnet|2000" usdc.fakes.testnet 286880 false dclv2.ref-dev.testnet
# -> order "usdc.fakes.testnet|wrap.testnet|2000#409"
cargo run --bin rhea_dclv2_cancel_order_bin_json -- \
  "usdc.fakes.testnet|wrap.testnet|2000#409" 500000000000000000000 dclv2.ref-dev.testnet

# 6) unwind the LP (tokens land in the internal balance)
cargo run --bin rhea_dclv2_remove_liquidity_bin_json -- \
  "usdc.fakes.testnet|wrap.testnet|2000#2544" 2698486080695 dclv2.ref-dev.testnet
cargo run --bin rhea_dclv2_withdraw_asset_bin_json -- wrap.testnet 99000000000000000000000 dclv2.ref-dev.testnet

# 7) release the storage (withdraws 0.4 NEAR, keeps 0.1, then unregister)
cargo run --bin rhea_dclv2_storage_withdraw_bin_json -- dclv2.ref-dev.testnet
cargo run --bin rhea_dclv2_storage_unregister_bin_json -- dclv2.ref-dev.testnet
```

**Bins that print "✅ ... complete" even when the transaction failed.** Every
write bin prints success from `result.transaction.hash` without checking the
receipt outcome, so a failed on-chain call still prints a green check. Always
confirm the outcome — e.g. with `EXPERIMENTAL_tx_status` — before trusting the
result of a write bin.


==========================
<br/>
copyright 2026 by sleet.near