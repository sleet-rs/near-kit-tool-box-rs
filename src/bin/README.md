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

## set greeting (typed)
# Uses the typed contract wrapper (`set_greeting`).
cargo run --bin greeting_set_bin_typed -- <contract_id> "<new_text>"
cargo run --bin greeting_set_bin_typed -- hello.sleet.testnet "hello from near kit toolbox rust"

## set greeting (json)
# Same call, but via raw JSON (`set_greeting`). Useful when you don't want the typed wrapper.
cargo run --bin greeting_set_bin_json -- <contract_id> "<new_text>"
cargo run --bin greeting_set_bin_json -- hello.sleet.testnet "hello from near kit toolbox rust"
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
```


## src/fun/rhea



==========================
<br/>
copyright 2026 by sleet.near