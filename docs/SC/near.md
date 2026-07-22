# near / testnet (TLD registrar + linkdrop)

the `near` (mainnet) and `testnet` (testnet) top-level-domain
registrar / linkdrop contracts — same `near-linkdrop` source compiled
for each network.

---

#### DETAILS

**CONTRACT ID PER NETWORK**
- MAINNET: near
- TESTNET: testnet

```sh
# near cli rs - get a list of methods
near contract inspect near network-config mainnet now
near contract inspect testnet network-config testnet now
```

---

#### FILES

- `src/lib/methods/methods_near.rs`
- `src/fun/near/near_create_account_fun_json.rs`


==========================
<br/>
copyright 2026 by sleet.near