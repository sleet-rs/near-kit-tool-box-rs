# pool.near

the `pool.near` staking-pool factory contract — deploys v2 validator staking-pool child contracts.

---

#### DETAILS

**CONTRACT ID PER NETWORK**
- MAINNET: pool.near
- TESTNET: pool.testnet

```sh
# near cli rs - get a list of methods
# ⚠️ verify the two networks share the same code hash before assuming parity
near contract inspect pool.testnet network-config testnet now
near contract inspect pool.near network-config mainnet now
```



#### FILES

- `src/lib/methods/methods_pool_factory.rs`
- `src/lib/const_id/pool_factory_contract_id_const.rs`


==========================
<br/>
copyright 2026 by sleet.near