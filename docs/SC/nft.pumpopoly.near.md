# nft.pumpopoly.near

the pumpopoly NFT contract (NEP-171 / NEP-178 compatible land & bond NFTs)

---

#### DETAILS

**CONTRACT ID**
- MAINNET: nft.pumpopoly.near
- TESTNET: (none — mainnet only)

```sh
# near cli rs - get a list of methods
near contract inspect nft.pumpopoly.near network-config mainnet now
```



#### FILES

- `src/lib/methods/methods_pumpopoly_nft.rs`
- `src/lib/const_id/pumpopoly_nft_contract_id_const.rs`
- `src/lib/types/nft_token_response_type.rs`

**view**
- `src/fun/pumpopoly_nft/pumpopoly_nft_supply_for_owner_fun_json.rs` — how many NFTs an account holds
- `src/fun/pumpopoly_nft/pumpopoly_nft_token_ids_for_owners_fun_json.rs` — token ids per account, no metadata, unpaginated
- `src/fun/pumpopoly_nft/pumpopoly_nft_tokens_for_owner_fun_json.rs` — full tokens for an account, paginated
- `src/fun/pumpopoly_nft/pumpopoly_nft_token_fun_json.rs` — one token by id

**change**
- `src/fun/pumpopoly_nft/pumpopoly_nft_transfer_fun_json.rs`
- `src/fun/pumpopoly_nft/pumpopoly_nft_transfer_call_fun_json.rs`

Each has a matching `src/bin/<name>_bin_json.rs`.


#### NOTES

- Mainnet only, so the bins pin `NEAR_KIT_CLIENT::mainnet()` rather than
  reading `NEAR_NETWORK` via `from_env()`.
- Token ids are human readable and contain spaces (e.g. `Bond 10-849-1`),
  so quote them on the command line.
- `nft_supply_for_owner` and `nft_total_supply` return counts as JSON
  strings (`"104"`), not numbers.
- Prefer `nft_token_ids_for_owners` to enumerate a large holder — it skips
  metadata and is not paginated, so a 100+ token wallet comes back in one
  call.


#### TRANSFER ARGS (verified)

Reference mainnet transactions:

- `nft_transfer` — [jRDQPvKv…](https://near.rocks/tx/jRDQPvKvdHMd9nMjv2ugqGXAvjF2wN2uocdEUFJqysL)
  `{"receiver_id":"nonresistant.near","token_id":"Bond 11-164-4"}`
  1 yocto deposit, 10 TGas attached.
- `nft_transfer_call` — [8fwpXybH…](https://near.rocks/tx/8fwpXybHJd8Vfo3RAdcaejq6gybRPconB73djA3MwY9o)
  `{"token_id":"Bond 10-845-1","receiver_id":"intents.near","msg":"…"}`
  1 yocto deposit, 100 TGas attached.

Arg shapes were checked against the live contract by issuing the change
method as a *view* call: args deserialize before the body runs, so a bad
shape returns `Failed to deserialize input from JSON` while a good one
reaches the `Requires attached deposit of exactly 1 yoctoNEAR` guard.

Results:

- `approval_id` is a raw `u64` number. A stringified `"1"` is rejected
  with `invalid type: string "1", expected u64` — it is not a `U64`.
- `memo` accepts a plain string.
- Sending `approval_id` / `memo` as explicit `null` deserializes the same
  as omitting them, so the helpers pass `Option` straight into `json!`.
- Gas 30 / 100 TGas matches `near-kit`'s own `tokens/nft.rs` and leaves
  headroom over the 10 TGas the reference transfer used. Unused gas is
  refunded.


==========================
<br/>
copyright 2026 by sleet.near