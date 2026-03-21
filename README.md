# near-kit-tool-box-rs
🦀 near kit rs tool box, functions for specific near interactions

ℹ️ primarly for personal use
<br/>
ℹ️ the lib contains reusable functions the bins are examples. 

---

**CARGO COMMANDS**

```sh
cargo run
cargo check
# cargo check --target wasm32-unknown-unknown
cargo test
cargo clean
cargo fmt

# docs
cargo doc --no-deps
cargo doc --open
```

**dependencies**
- near-kit-rs (```cargo add near-kit```)


BINS
```sh
cargo run --bin greeting_get_bin

# cargo run --bin greeting_set_bin
NEAR_NETWORK=testnet \
NEAR_ACCOUNT_ID=alice.testnet \
NEAR_PRIVATE_KEY=ed25519:YOUR_PRIVATE_KEY_HERE \
GREETING_CONTRACT_ID=hello.sleet.testnet \
GREETING_NEW_TEXT="hello from near kit rust" \
cargo run --bin greeting_set_bin
```


---



copyright 2026 by sleet.near