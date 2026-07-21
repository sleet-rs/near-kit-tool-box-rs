# Bins

All bins sign / read via `NEAR_KIT_CLIENT` (see `src/lib/client_kit.rs`).
Auth comes from env vars — copy `.env.example` to `.env` and fill in:

```env
NEAR_NETWORK=testnet        # or mainnet
NEAR_ACCOUNT_ID=alice.testnet
NEAR_PRIVATE_KEY=ed25519:YOUR_PRIVATE_KEY_HERE
```

## greet — read

```bash
cargo run --bin greeting_get_bin_with_client
```

Hardcoded testnet contract `hello.sleet.testnet`. No signer needed.

## set greeting (typed)

```bash
cargo run --bin greeting_set_bin_typed -- <contract_id> "<new_text>"

# example
cargo run --bin greeting_set_bin_typed -- hello.sleet.testnet "hello from near kit rust"
```

Uses the typed contract wrapper (`set_greeting_typed`).

## set greeting (json)

```bash
cargo run --bin greeting_set_bin_with_env -- <contract_id> "<new_text>"

# example
cargo run --bin greeting_set_bin_with_env -- hello.sleet.testnet "hello from near kit rust"
```

Same call, but via raw JSON (`set_greeting`). Useful when you don't want the typed wrapper.