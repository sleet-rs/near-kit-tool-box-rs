# RPC LOAD BALANCER

Distributes read-heavy calls across multiple NEAR RPC endpoints with a
pluggable strategy (round-robin by default) and automatic failover on
retryable errors.

Designed for **view** (read-only) operations. For state-changing writes, use
the regular `NEAR_KIT_CLIENT` — spreading transactions across many nodes makes
nonces and replay protection unreliable.

---

## AT A GLANCE

- Drop-in for the `RpcClient` view surface — same method signatures, same
  return types.
- Picks an endpoint per call via [`LoadBalancingStrategy`].
- Tries the next endpoint on retryable errors (`RpcError::is_retryable()`)
  when failover is enabled (default).
- Per-endpoint retries are still handled by the underlying `RpcClient`, so
  transient hiccups are absorbed before failover is even attempted.
- [`last_used_url()`](#which-rpc-was-used) records which endpoint served the
  last successful call.

---

## DEFAULT ENDPOINTS

```rust
use near_kit_tool_box::lib::load_balancer::endpoints::{
    NEAR_RPC_TESTNET_ENDPOINTS, NEAR_RPC_MAINNET_ENDPOINTS,
};

// testnet
//   https://rpc.testnet.near.org
//   https://test.rpc.fastnear.com
//   https://testnet-rpc.intea.rs

// mainnet
//   https://rpc.mainnet.fastnear.com
//   https://near.drpc.org
//   https://rpc.intea.rs
//   https://rpc.shitzuapes.xyz
```

---

## VIA `NEAR_KIT_CLIENT` (recommended)

The `NEAR_KIT_CLIENT` builder exposes a special **view client** that uses
the load balancer. No signer is needed — these endpoints are read-only.

```rust
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;

// Default testnet endpoints, round-robin, failover on
let view = NEAR_KIT_CLIENT::view_balancer_testnet();

// Default mainnet endpoints
let view = NEAR_KIT_CLIENT::view_balancer_mainnet();

// Custom URL list
let view = NEAR_KIT_CLIENT::view_balancer_with_urls(&[
    "https://rpc.testnet.near.org",
    "https://my-private-rpc.example.com",
]);
```

All three return a `LOAD_BALANCING_RPC_CLIENT` whose method surface mirrors
`near_kit::RpcClient` view calls.

---

## DIRECT CONSTRUCTION

```rust
use std::sync::Arc;
use near_kit_tool_box::lib::load_balancer::{
    Random, RoundRobin, LOAD_BALANCING_RPC_CLIENT,
};

let view = LOAD_BALANCING_RPC_CLIENT::new(&[
    "https://rpc.testnet.near.org",
    "https://near-testnet.lava.build",
])
.with_strategy(Arc::new(Random))
.with_failover(true);
```

Available constructors:
- `LOAD_BALANCING_RPC_CLIENT::new(urls)` — default round-robin, failover on.
- `LOAD_BALANCING_RPC_CLIENT::with_retry_config(urls, retry_config)` — custom
  per-endpoint retry config.
- `LOAD_BALANCING_RPC_CLIENT::testnet()` — public testnet endpoints.
- `LOAD_BALANCING_RPC_CLIENT::mainnet()` — public mainnet endpoints.

---

## STRATEGIES

```rust
pub trait LoadBalancingStrategy: Send + Sync {
    fn next(&self, len: usize) -> usize;
}
```

- `RoundRobin` (default) — cycles through endpoints in order.
- `Random` — pseudo-random pick from the system clock.

Implement the trait for any custom strategy (e.g. weighted, latency-aware,
sticky-by-method).

---

## EXAMPLES

### View account balance

```rust
# async fn demo() -> Result<(), near_kit::Error> {
use near_kit::{Finality, serde_json};
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;

let view = NEAR_KIT_CLIENT::view_balancer_testnet();
let account = view.view_account("alice.testnet", Finality::Final.into()).await?;
println!("balance: {}", account.amount);
# Ok(())
# }
```

### Call a view function

```rust
# async fn demo() -> Result<(), near_kit::Error> {
use near_kit::{Finality, serde_json};
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;

let view = NEAR_KIT_CLIENT::view_mainnet();
let result = view
    .view_function(
        "hello.sleet.near",
        "get_greeting",
        b"{}",
        Finality::Final.into(),
    )
    .await?;
let greeting: String = serde_json::from_slice(&result.result)?;
println!("greeting: {greeting}");
# Ok(())
# }
```

### Low-level `call`

For methods not exposed directly, use `call`:

```rust
# async fn demo() -> Result<(), near_kit::RpcError> {
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;

let view = NEAR_KIT_CLIENT::view_balancer_testnet();
let status: serde_json::Value = view.call("status", serde_json::json!([])).await?;
println!("{}", status);
# Ok(())
# }
```

### Which RPC was used?

Every successful call records the index of the client that served it. Call
`last_used_url()` to read it back:

```rust
# async fn demo() -> Result<(), near_kit::Error> {
use near_kit::{Finality, serde_json};
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;

let view = NEAR_KIT_CLIENT::view_balancer_testnet();
let result = view
    .view_function("hello.sleet.near", "get_greeting", b"{}", Finality::Final.into())
    .await?;
let greeting: String = serde_json::from_slice(&result.result)?;
println!("greeting={greeting:?} served_by={}", view.last_used_url().unwrap_or("?"));
# Ok(())
# }
```

This is especially useful for verifying that round-robin picks are actually
spreading across endpoints, and for spotting which endpoint answered when
failover kicked in. The value is best-effort under concurrent calls — use
it for logging, not coordination.

---

## FILES

- `src/lib/load_balancer/load_balancing_rpc_client.rs` — `LOAD_BALANCING_RPC_CLIENT`
- `src/lib/load_balancer/strategies.rs` — `RoundRobin`, `Random`,
  `LoadBalancingStrategy`
- `src/lib/load_balancer/endpoints.rs` — `NEAR_RPC_TESTNET_ENDPOINTS`,
  `NEAR_RPC_MAINNET_ENDPOINTS`
- `src/lib/load_balancer/mod.rs` — module declarations
- `src/lib/client_kit.rs` — `NEAR_KIT_CLIENT::view_balancer_testnet` etc.
- `src/bin/load_balancer_status_bin_json.rs` — example: round-robin `status`
- `src/bin/greeting_get_balanced_bin_json.rs` — example: round-robin
  `get_greeting` with RPC-of-origin logging


==========================
<br/>
copyright 2026 by sleet.near
