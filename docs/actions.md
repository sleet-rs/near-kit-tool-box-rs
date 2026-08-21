# NEAR-level actions

Built-in protocol actions exposed via `near_kit::Near::transaction(...)`
— not contract method calls, so they don't go through any methods
constants.

---

#### ACTIONS

| action | description |
|---|---|
| `add_key` | add a full-access or function-call access key to the signer's account |
| `add_key_meta` | meta-tx (NEP-366) variant of `add_key` — a separate funded relayer signs + submits on the signer's behalf |
| `delete_key` | remove an access key from the signer's account |
| `delete_account` | delete the signer's account and refund the beneficiary |

---

#### FILES

- `src/fun/near/add_key_fun_json.rs`
- `src/fun/near/add_key_meta_fun_json.rs`
- `src/fun/near/delete_key_fun_json.rs`
- `src/fun/near/delete_account_fun_json.rs`

---

#### BINS

```bash
## add_key_meta — meta-tx (NEP-366): relayer pays all fees
# Adds a full-access key to one or more accounts, even ones with zero
# own balance. The shared user key signs a DelegateAction per target;
# a funded relayer account submits each one on the target's behalf.
# Requires `RELAYER_ACCOUNT_ID` + `RELAYER_PRIVATE_KEY` in env in
# addition to `NEAR_PRIVATE_KEY`.
cargo run --bin add_key_meta_bin_json -- <new_public_key> <account_1> [account_2 ...]
cargo run --bin add_key_meta_bin_json -- ed25519:2eDMWnKcDt7UQ1xVximcWbd1YKwJbfE7HGPmNZSegjcV walcwarchest.near walcpool.near
```


==========================
<br/>
copyright 2026 by sleet.near