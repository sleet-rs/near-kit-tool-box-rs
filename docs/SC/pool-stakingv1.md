# v1 standard staking pool

the v1 standard validator staking-pool child contract
(`code_hash`: `J1arLz48fgXcGyCPVckFwLnewNH6j1uw79thsvwqGYTY`)

---

#### DETAILS

This contract is **deployed per-validator** by the `poolv1.near` factory — there
is no global id. Each pool lives at its own account id.

```sh
# near cli rs - get a list of methods
# replace <pool_account> with the specific pool's account id
near contract inspect <pool_account> network-config mainnet now
```



#### FILES

- `src/lib/methods/methods_pool_stakingv1.rs`


==========================
<br/>
copyright 2026 by sleet.near