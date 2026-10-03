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
- limit order + swap — `msg = {"LimitOrderWithSwap":{"pool_id":...,"buy_token":...,"point":...}}`
- swap to a price point — `msg = {"SwapByStopPoint":{"pool_id":...,"stop_point":...}}`

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

**Verified error codes**
- `E101` insufficient internal balance (deposit first)
- `E102` storage deposit too small
- `E200` invalid endpoint (unaligned points)
- `E202` illegal point (out-of-range bounds)
- `E207` liquidity not found
- `E600` invalid msg / unknown variant


#### FILES

- `src/lib/methods/methods_rhea_dclv2.rs`
- `src/lib/const_id/rhea_dclv2_contract_id_const.rs`
- `src/lib/types/rhea_dclv2_pool_type.rs`, `rhea_dclv2_quote_type.rs`, `rhea_dclv2_predict_add_liquidity_type.rs`, `rhea_dclv2_liquidity_type.rs`, `rhea_dclv2_order_type.rs`, `rhea_dclv2_user_asset_type.rs`
- `src/fun/rhea_dclv2/` — `dclv2_get_pool_fun_json.rs`, `dclv2_get_pools_fun_json.rs`, `dclv2_list_pools_fun_json.rs`, `dclv2_quote_fun_json.rs`, `dclv2_quote_by_output_fun_json.rs`, `dclv2_get_liquidity_fun_json.rs`, `dclv2_list_liquidities_fun_json.rs`, `dclv2_get_liquidity_view_fun_json.rs`, `dclv2_list_liquidity_views_fun_json.rs`, `dclv2_get_liquidity_range_fun_json.rs`, `dclv2_get_pointorder_range_fun_json.rs`, `dclv2_get_marketdepth_fun_json.rs`, `dclv2_get_market_depth_list_fun_json.rs`, `dclv2_predict_add_liquidity_fun_json.rs`, `dclv2_predict_remove_liquidity_fun_json.rs`, `dclv2_get_order_fun_json.rs`, `dclv2_find_order_fun_json.rs`, `dclv2_list_active_orders_fun_json.rs`, `dclv2_list_history_orders_fun_json.rs`, `dclv2_get_user_asset_fun_json.rs`, `dclv2_list_user_assets_fun_json.rs`, `dclv2_get_user_storage_detail_fun_json.rs`, `dclv2_storage_balance_of_fun_json.rs`, `dclv2_add_liquidity_fun_json.rs`, `dclv2_remove_liquidity_fun_json.rs`, `dclv2_withdraw_asset_fun_json.rs`, `dclv2_cancel_order_fun_json.rs`, `dclv2_deposit_fun_json.rs`, `dclv2_swap_ft_transfer_call_fun_json.rs`, `dclv2_storage_deposit_fun_json.rs`, `dclv2_storage_withdraw_fun_json.rs`, `dclv2_storage_unregister_fun_json.rs`
- `src/bin/rhea_dclv2_*_bin_json.rs`

---

#### EXAMPLE — single-sided LP above price, then swap, then unwind

```sh
# 1) storage (0.5 NEAR)
cargo run --bin rhea_dclv2_storage_deposit_bin_json -- dclv2.ref-dev.testnet

# 2) fund the internal balance
cargo run --bin rhea_dclv2_deposit_bin_json -- usdc.fakes.testnet 1000000 dclv2.ref-dev.testnet

# 3) see what a range above current_point actually needs
cargo run --bin rhea_dclv2_predict_add_liquidity_bin_json -- \
  "usdc.fakes.testnet|wrap.testnet|2000" 288680 289080 1000000 0 dclv2.ref-dev.testnet
# -> { liquidity_amount: "...", need_x: "1000000", need_y: "0" }

# 4) add it (points must be multiples of point_delta = 40)
cargo run --bin rhea_dclv2_add_liquidity_bin_json -- \
  "usdc.fakes.testnet|wrap.testnet|2000" 288680 289080 1000000 0 dclv2.ref-dev.testnet

# 5) quote + swap (no swap method exists — it routes through ft_transfer_call)
cargo run --bin rhea_dclv2_quote_bin_json -- \
  "usdc.fakes.testnet|wrap.testnet|2000" usdc.fakes.testnet wrap.testnet 1000000 Auto dclv2.ref-dev.testnet
cargo run --bin rhea_dclv2_swap_bin_json -- usdc.fakes.testnet 1000000 \
  "usdc.fakes.testnet|wrap.testnet|2000" wrap.testnet 3357951012646182400 dclv2.ref-dev.testnet

# 6) unwind
cargo run --bin rhea_dclv2_remove_liquidity_bin_json -- "usdc.fakes.testnet|wrap.testnet|2000#1901" 2341681305 dclv2.ref-dev.testnet
cargo run --bin rhea_dclv2_withdraw_asset_bin_json -- usdc.fakes.testnet dclv2.ref-dev.testnet
cargo run --bin rhea_dclv2_storage_unregister_bin_json -- dclv2.ref-dev.testnet
```


==========================
<br/>
copyright 2026 by sleet.near