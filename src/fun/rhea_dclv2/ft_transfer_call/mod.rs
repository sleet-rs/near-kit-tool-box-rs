// =================================================
//! Helpers for the DCL v2 flows that are **not** methods of the DCL v2
//! contract itself.
//!
//! Every token inflow into DCL v2 goes through `ft_transfer_call` on the
//! *token* contract and lands in DCL's `ft_on_transfer`. These helpers
//! therefore call the **token** contract, not DCL, and carry the DCL call
//! inside the `msg` field.
//!
//! The `msg` variants they send are:
//! - `Deposit` — credit the signer's DCL internal balance
//! - `Swap` — exact-input swap
//! - `LimitOrder` — a limit order that rests entirely
//! - `LimitOrderWithSwap` — a limit order that fills what it can first
//!
//! The contract methods themselves live in the parent `rhea_dclv2` module.
pub mod dclv2_deposit_fun_json;
/// Change helper: place a DCL v2 limit order via `ft_transfer_call` on the sell token via raw JSON args.
pub mod dclv2_limit_order_fun_json;
/// Change helper: swap on DCL v2 by calling `ft_transfer_call` on the INPUT token via raw JSON args.
pub mod dclv2_swap_ft_transfer_call_fun_json;
// =================================================
// =================================================
// copyright 2026 by sleet.near
