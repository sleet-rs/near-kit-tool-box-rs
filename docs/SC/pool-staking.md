# standard v2 staking pool

the standard v2 validator staking-pool child contract
(`code_hash`: `AjD4YJaXgpiRdiArqnzyDi7Bkr1gJms9Z2w7Ev5esTKB`)

---

#### DETAILS

This contract is **deployed per-validator** by the `pool.near` factory — there is
no global id. Each pool lives at its own account id (e.g. `<validator>.pool.near`).

```sh
# near cli rs - get a list of methods
# replace <pool_account> with the specific pool's account id
near contract inspect <pool_account> network-config mainnet now
```



#### FILES

- `src/lib/methods/methods_pool_staking.rs`


==========================
<br/>
copyright 2026 by sleet.near