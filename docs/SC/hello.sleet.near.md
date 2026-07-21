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


#### FUN

- `src/lib/contract/contract_greeting.rs`
- `src/fun/greeting/greeting_get_fun.rs`
- `src/fun/greeting/greeting_set_fun.rs`
- `src/fun/greeting/greeting_set_fun_json.rs`


==========================
<br/>
copyright 2026 by sleet.near