# hello.sleet.near

a simple near greeting smart contract

---

#### CONTRACT PER NETWORK

- MAINNET: hello.sleet.near
- TESTNET: hello.sleet.testnet

#### METHODS

```sh
# near cli rs - get a list of methods
near contract inspect hello.sleet.testnet network-config testnet now
near contract inspect hello.sleet.near network-config mainnet now
```

methods_greeting.rs
```rust
{{#include ../../src/lib/methods/methods_greeting.rs}}
```




==========================
<br/>
copyright 2026 by sleet.near