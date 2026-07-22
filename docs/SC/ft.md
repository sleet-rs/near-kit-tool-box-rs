# NEP-141 / NEP-145 Fungible Token (FT)

Generic helpers for any NEP-141 / NEP-145 fungible-token contract
(wNEAR, USDC, USDT, custom tokens, etc.).

---

#### DETAILS

The methods constants mirror a typical FT contract's `near contract
inspect` output:

```sh
near contract inspect wrap.testnet network-config testnet now
near contract inspect wrap.near    network-config mainnet now
```

Only the six most common methods have dedicated helpers:

| helper | type | description |
|---|---|---|
| `ft_balance_of` | view | read an account's FT balance |
| `ft_total_supply` | view | read the contract's total supply |
| `ft_metadata` | view | read the NEP-145 metadata (`FT_METADATA_GET_RESPONSE_TYPE`) |
| `ft_transfer` | call | transfer FT to a receiver |
| `ft_transfer_call` | call | transfer FT and call `ft_transfer_call` on the receiver |
| `ft_storage_deposit` | call | register storage for the signer |

---

#### RESPONSE TYPES

- `FT_METADATA_GET_RESPONSE_TYPE` — NEP-145 metadata returned by `ft_metadata` (fields: `spec`, `name`, `symbol`, `icon?`, `reference?`, `reference_hash?`, `decimals`).

---

#### FILES

- `src/lib/methods/methods_ft.rs`
- `src/lib/types/ft_metadata_get_response_type.rs`
- `src/fun/ft/ft_balance_of_fun_json.rs`
- `src/fun/ft/ft_total_supply_fun_json.rs`
- `src/fun/ft/ft_metadata_fun_json.rs`
- `src/fun/ft/ft_transfer_fun_json.rs`
- `src/fun/ft/ft_transfer_call_fun_json.rs`
- `src/fun/ft/ft_storage_deposit_fun_json.rs`


==========================
<br/>
copyright 2026 by sleet.near