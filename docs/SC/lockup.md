# house-of-stake lockup contract

the house-of-stake per-user lockup contract

---

#### DETAILS

The lockup contract is **deployed per-user** by the `venear.dao` contract — there
is no global lockup account id. Each user interacts with their own lockup
sub-account (e.g. `<user>.lockup.near`).

```sh
# near cli rs - get a list of methods
# replace <lockup_account> with the specific user's lockup account
near contract inspect <lockup_account> network-config mainnet now
```



#### FILES

- `src/lib/methods/methods_hos_lockup.rs`

reference: <https://github.com/houseofstake/house-of-stake-contracts>


==========================
<br/>
copyright 2026 by sleet.near