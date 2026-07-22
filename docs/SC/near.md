# near / testnet (TLD registrar + linkdrop)

the `near` (mainnet) and `testnet` (testnet) top-level-domain
registrar / linkdrop contracts — same `near-linkdrop` source compiled
for each network.

---

#### DETAILS

**CONTRACT ID PER NETWORK**
- MAINNET: near
- TESTNET: testnet

**SUPPORTED STANDARDS** (mainnet)
- nep330 (1.3.0)
- NEAR ABI 0.4.0

```sh
# near cli rs - get a list of methods
near contract inspect near network-config mainnet now
near contract inspect testnet network-config testnet now
```

| method | kind | description |
|---|---|---|
| `claim` | change · private | claim tokens for an account attached to the signing public key |
| `contract_source_metadata` | view | NEP-330 contract source metadata (mainnet only) |
| `create_account` | change · payable | create a sub-account and deposit the attached funds |
| `create_account_advanced` | change · payable | create a sub-account with extra options and deposit the attached funds |
| `create_account_and_claim` | change · private | create a sub-account and claim tokens to it |
| `get_key_balance` | view | balance associated with a given public key |
| `get_key_information` | view | linkdrop key info for a given public key |
| `new` | init | contract initializer |
| `on_account_created` | change · private | callback after `create_account` / `create_account_advanced` |
| `on_account_created_and_claimed` | change · private | callback after `create_account_and_claim` |
| `send` | change · payable | allow the given public key to claim an attached balance |

---

#### FILES

- `src/lib/methods/methods_near.rs`
- `src/fun/near/near_create_account_fun_json.rs`


==========================
<br/>
copyright 2026 by sleet.near