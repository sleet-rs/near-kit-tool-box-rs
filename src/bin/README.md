# Bins

All bins sign / read via `NEAR_KIT_CLIENT` (see `src/lib/client_kit.rs`).
Auth comes from env vars — copy `.env.example` to `.env` and fill in:

```env
NEAR_NETWORK=testnet        # or mainnet
NEAR_ACCOUNT_ID=alice.testnet
NEAR_PRIVATE_KEY=ed25519:YOUR_PRIVATE_KEY_HERE
```

## src/fun/greeting

Read bins only need `NEAR_NETWORK` (defaults to `testnet`). Set bins also need `NEAR_ACCOUNT_ID` and `NEAR_PRIVATE_KEY`.

```bash
## get greeting (typed)
# Uses the typed contract wrapper (`get_greeting_fun`).
cargo run --bin greeting_get_bin_typed -- <contract_id>
cargo run --bin greeting_get_bin_typed -- hello.sleet.testnet

## get greeting (json)
# Same read, but via raw JSON (`get_greeting_json`). Useful when you don't want the typed wrapper.
cargo run --bin greeting_get_bin_json -- <contract_id>
cargo run --bin greeting_get_bin_json -- hello.sleet.testnet

## set greeting (typed)
# Uses the typed contract wrapper (`set_greeting_typed`).
cargo run --bin greeting_set_bin_typed -- <contract_id> "<new_text>"
cargo run --bin greeting_set_bin_typed -- hello.sleet.testnet "hello from near kit toolbox rust"

## set greeting (json)
# Same call, but via raw JSON (`set_greeting`). Useful when you don't want the typed wrapper.
cargo run --bin greeting_set_bin_json -- <contract_id> "<new_text>"
cargo run --bin greeting_set_bin_json -- hello.sleet.testnet "hello from near kit toolbox rust"
```


## src/fun/rhea



==========================
<br/>
copyright 2026 by sleet.near