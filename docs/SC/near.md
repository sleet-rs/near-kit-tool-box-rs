# near / testnet (TLD registrar)

the `near` (mainnet) and `testnet` (testnet) top-level-domain
registrar contracts — and the NEAR-level account / key actions that
are built into the protocol.

---

#### DETAILS

**CONTRACT ID PER NETWORK**
- MAINNET: near
- TESTNET: testnet

```sh
# near cli rs - get a list of methods
near contract inspect near       network-config mainnet now
near contract inspect testnet    network-config testnet now
```

---

#### NEAR-LEVEL ACTIONS (built-in, no contract call)

| action | description |
|---|---|
| `add_key` | add a full-access or function-call access key to the signer's account |
| `delete_key` | remove an access key from the signer's account |
| `delete_account` | delete the signer's account and refund the beneficiary |

---

#### FILES

- `src/lib/methods/methods_near.rs`
- `src/fun/near/near_create_account_fun_json.rs`
- `src/fun/near/add_key_fun_json.rs`
- `src/fun/near/delete_key_fun_json.rs`
- `src/fun/near/delete_account_fun_json.rs`


==========================
<br/>
copyright 2026 by sleet.near