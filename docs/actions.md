# NEAR-level actions

Built-in protocol actions exposed via `near_kit::Near::transaction(...)`
— not contract method calls, so they don't go through any methods
constants.

---

#### ACTIONS

| action | description |
|---|---|
| `add_key` | add a full-access or function-call access key to the signer's account |
| `delete_key` | remove an access key from the signer's account |
| `delete_account` | delete the signer's account and refund the beneficiary |

---

#### FILES

- `src/fun/near/add_key_fun_json.rs`
- `src/fun/near/delete_key_fun_json.rs`
- `src/fun/near/delete_account_fun_json.rs`


==========================
<br/>
copyright 2026 by sleet.near