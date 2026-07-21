# poolv1.near

the `poolv1.near` v1 staking-pool factory contract — legacy factory that deploys v1 staking-pool child contracts.

---

#### DETAILS

**CONTRACT ID PER NETWORK**
- MAINNET: poolv1.near
- TESTNET: poolv1.testnet

```sh
# near cli rs - get a list of methods
# ⚠️ verify the two networks share the same code hash before assuming parity
near contract inspect poolv1.testnet network-config testnet now
near contract inspect poolv1.near network-config mainnet now
```



#### FILES

- `src/lib/methods/methods_pool_factoryv1.rs`
- `src/lib/const_id/pool_factoryv1_contract_id_const.rs`


==========================
<br/>
copyright 2026 by sleet.near