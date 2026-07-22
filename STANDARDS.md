# STANDARDS

Conventions for naming and layout across the near-kit-tool-box-rs codebase.
Following these keeps the tree consistent and makes new code obvious to read.

==========================
<br/>
copyright 2026 by sleet.near

# naming

## functions (snake_case)
- no `_fun`, `_json`, `_typed` suffix on function names — the **file** name carries the variant.
  - bad: `pub async fn get_greeting_json(...)` in `greeting_get_fun_json.rs`
  - good: `pub async fn get_greeting(...)` in `greeting_get_fun_json.rs`
- match the contract method name when one exists:
  - `ft_balance_of`, `ft_transfer`, `ft_total_supply`, `near_create_account`, `set_greeting`, `get_greeting`.
- when the topic already lives in the module path (e.g. `fun::greeting::*`), do **not** prefix the function with the topic again — just use the action: `get_greeting`, not `greeting_get_greeting`.
- when the topic is not implied by the path (e.g. `fun::near::*` covers many actions on NEAR level), prefix the function with the topic for clarity: `wrap_near_deposit`, `wrap_near_withdraw`, `near_create_account`.
- `pub async fn` for NEAR calls; `pub fn` for pure helpers.

## files (snake_case.rs)
- `src/fun/{topic}/{topic}_{action}_fun_json.rs` — raw `json!()` helper
- `src/fun/{topic}/{topic}_{action}_fun.rs` — typed contract helper (single typed variant, no suffix)
- `src/fun/{topic}/{topic}_{action}_fun_typed.rs` — typed contract helper when a `fun_json` sibling also exists
- `src/lib/const_id/{contract_slug}_contract_id_const.rs` — per-contract id constant + lookup fn
- `src/lib/methods/methods_{contract_slug}.rs` — per-contract method-name constant bag
- `src/lib/contract/contract_{contract_slug}.rs` — typed contract trait + args structs
- `src/lib/types/{descriptive_name}.rs` — shared response / data shapes (e.g. `FT_METADATA_GET_RESPONSE_TYPE`)
- `src/lib/helper/{descriptive_name}.rs` — generic client/utility helpers
- `src/bin/{topic}_{action}_bin_json.rs` / `..._bin_typed.rs` — example/test binaries
- the `bin` suffix is only used in `src/bin/` filenames, never in `src/fun/` filenames.

## consts (SCREAMING_SNAKE_CASE)
- network-aware contract ids:
  - struct: `{CONTRACT}_CONTRACT_ID_CONST_TYPE` (aliases `CONTRACT_ID_CONST_TYPE`)
  - value: `pub const {CONTRACT}_CONTRACT_ID_CONST: {CONTRACT}_CONTRACT_ID_CONST_TYPE = ...`
  - lookup fn: `pub fn {slug}_contractid(network: &str) -> &'static str` — **no `_fun` suffix**
- mainnet-only contract ids:
  - `pub const {CONTRACT}_CONTRACT_ID_CONST: &str = "...";`
- method-name bags:
  - struct: `{CONTRACT}_METHODS_CONST` holding `pub {method}: &str` fields (`ft_balance_of`, `create_account`, …)
- no `_CONST` suffix on method fields — only on the wrapping struct.

## structs / types (SCREAMING_SNAKE_CASE)
- typed contract trait: `{CONTRACT}_{SCOPE?}_CONTRACT_TRAIT` (e.g. `GREETING_CONTRACT_TRAIT`, `NEAR_TLD_CONTRACT_TRAIT`)
- args structs passed to trait methods: `{CONTRACT}_{METHOD}_ARGS` or scoped `{ACTION}_ARGS` (e.g. `GREETING_ARGS`, `NEAR_CREATE_ACCOUNT_ARGS`)
- shared shapes: `_TYPE` suffix (e.g. `CONTRACT_ID_CONST_TYPE { testnet, mainnet }`)
- trait field-method names mirror the on-chain method name in `snake_case` so call sites stay readable (`greeter.get_greeting()`, `tld.create_account(...)`).

## modules
- module names mirror the file slug (`pub mod ft_balance_of_fun_json;` etc.) — do not re-export; callers reach functions via the full path.

# layout

- top + bottom of every source file:
  ```rust
  // =================================================
  // copyright 2026 by sleet.near
  ```
  the leading `// ====` line sits **above** the file's content and the closing
  one sits **below** it, so the file body is framed between the two rules.
- use `// =================================================` between logically distinct
  sections inside a single file when more than one helper is present.
- `///` doc comments on every `pub fn` / `pub struct` / `pub const` — describe
  what it does, any signer / network requirement, and what it returns.
- `use` statements go at the top, after any file-level doc comment, before the
  opening `// ====` rule. `use near_kit::*;` is kept as a commented-out hint
  for editors.
- `#![allow(non_camel_case_types)]` is set in `src/lib.rs` so SCREAMING_SNAKE_CASE
  type names don't fight the lint; do not re-add it per file.

==========================
