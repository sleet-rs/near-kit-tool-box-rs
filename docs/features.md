# FEATURES

Top-level features layered on top of `near-kit`. Each one either wraps an
existing `near-kit` API in a more ergonomic shape, or adds a new knob
(load balancing) that the underlying client doesn't expose.

---

# NEAR_KIT_CLIENT

`NEAR_KIT_CLIENT` (in `src/lib/client_kit.rs`) is the builder for the
near-kit `Near` client. It doesn't reimplement anything — it just
short-circuits the common ways to configure one.

```rust
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;

// Hardcoded preset
let near = NEAR_KIT_CLIENT::testnet().build();
let near = NEAR_KIT_CLIENT::mainnet().build();

// Env-driven (NEAR_NETWORK, NEAR_CHAIN_ID, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY)
let near = NEAR_KIT_CLIENT::from_env()?.build();

// Custom RPC URL
let near = Near::custom("https://my-rpc.example.com", "mainnet").build();
```

For signing, call `.credentials(private_key, account_id)?` on the builder
before `.build()`. Signer is only attached when the user explicitly opts in;
read-only bins don't need it.

The build also prints a one-shot summary of the configured network, RPC
URL, signer account, and signing mode so it's obvious what you're hitting
from a log.

See `src/lib/helper/print_client_details.rs` for the exact format.

---

# LOAD BALANCING RPC CLIENT

`LOAD_BALANCING_RPC_CLIENT` (in `src/lib/load_balancer/`) is a thin
view-client wrapper around multiple `near_kit::RpcClient`s. It picks one
endpoint per call via a pluggable strategy (round-robin by default),
fails over to the next on any error, and exposes the same view-method
surface as `near_kit::RpcClient`.

```rust
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;

let view = NEAR_KIT_CLIENT::view_balancer_testnet();
let view = NEAR_KIT_CLIENT::view_balancer_mainnet();
let view = NEAR_KIT_CLIENT::view_balancer_with_urls(&[
    "https://rpc.testnet.near.org",
    "https://my-private-rpc.example.com",
]);
```

**Why view-only?** Spreading state-changing writes across many nodes
breaks nonces and replay protection. For transactions, use the regular
`NEAR_KIT_CLIENT::testnet() / mainnet() / from_env()` builders.

**Pick + failover:**
- Default strategy is `RoundRobin` (cycles through endpoints in order
  per call). `Random` is also provided. Implement the
  `LoadBalancingStrategy` trait for custom picks (weighted, latency-aware,
  sticky-by-method, etc.).
- Failover is on by default and walks the chain on **any** error, not just
  `RpcError::is_retryable()`. A 403 from a misconfigured node is
  classified as non-retryable by near-kit, but for a load balancer the
  right move is to try the next endpoint, not to give up.
- Per-endpoint retries are still handled by the underlying `RpcClient`
  first, so transient hiccups are absorbed before failover is even
  attempted.

**Inspect:**
- `last_used_url()` returns the URL that served the last successful call
  (`None` if no call has completed yet). Best-effort under concurrency —
  useful for logging, not for coordination.

See [`docs/load_balancer.md`](./load_balancer.md) for the full API,
endpoint lists, and examples.

---

# ALREADY IN `near_kit::RpcClient`

The load balancer is a near-drop-in replacement for `near_kit::RpcClient`'s
view surface. Every method below is available on both — the load balancer
just distributes them across endpoints. For a deeper look at any of these,
see the `near-kit` docs.

## account / access keys

| Method | What it returns |
|---|---|
| `view_account(account_id, block)` | `AccountView` — balance, locked, storage, code hash |
| `view_access_key(account_id, pub_key, block)` | `AccessKeyView` for one key |
| `view_access_key_list(account_id, block)` | `AccessKeyListView` — every key with permission |
| `view_gas_key_nonces(account_id, pub_key, block)` | `GasKeyNoncesView` — parallel nonces for a gas key |

## contract views

| Method | What it returns |
|---|---|
| `view_function(account_id, method, args, block)` | `ViewFunctionResult` — result bytes + logs |
| `view_state(account_id, prefix, after_key, limit, block)` | `ViewStateResult` — one page of raw key/value |
| `view_state_all(account_id, prefix, page_size, block)` | `Vec<StateItem>` — full state, transparent pagination |

## block / chain

| Method | What it returns |
|---|---|
| `block(block)` | `BlockView` — header + chunks |
| `block_effects(block)` | `BlockEffects` — which accounts had state touched in the block |
| `genesis_config()` | `serde_json::Value` — large network-specific config blob |
| `maintenance_windows(account_id)` | `Vec<MaintenanceWindow>` — upcoming validator down-time |

## network / status

| Method | What it returns |
|---|---|
| `status()` | `StatusResponse` — chain id, sync info, validator account |
| `gas_price(block_hash)` | `GasPrice` — current price at a block |
| `validators(block)` | `EpochValidatorInfo` — current + next epoch validators |

## transactions (writes — not load-balanced, see warning above)

| Method | What it returns |
|---|---|
| `send_tx(signed_tx, wait_until)` | `RawTransactionResponse` |
| `tx_status(tx_hash, sender_id, wait_until)` | `RawTransactionResponse` with full receipts |
| `receipt_to_tx(receipt_id)` | `ReceiptToTxResponse` — look up the tx that produced a receipt |

## low-level

| Method | What it does |
|---|---|
| `call<P, R>(method, params)` | Raw JSON-RPC dispatch; the load balancer exposes this too for any non-wrapped method (e.g. `query`, `EXPERIMENTAL_protocol_config`, `EXPERIMENTAL_changes`) |

Bins in `src/bin/` that exercise the above:
- `view_account_bin_json -- <account_id>`
- `view_access_key_list_bin_json -- <account_id>`
- `greeting_get_bin_json -- <contract_id>` (uses `view_function`)
- `greeting_get_balanced_bin_json -- <contract_id> [times]` (uses `view_function` via the load balancer)
- `load_balancer_status_bin_json` (uses `status` via the load balancer)


==========================
<br/>
copyright 2026 by sleet.near
