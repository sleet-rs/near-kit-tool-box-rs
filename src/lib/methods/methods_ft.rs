// =================================================
/// Method name constants for the NEP-141 fungible token (FT) standard.
///
/// Mirrors the standard `near contract inspect` output of a typical
/// deployed FT contract. Not all FT contracts implement every method.
pub const FT_METHODS_CONST: FT_METHODS_CONST_TYPE = FT_METHODS_CONST_TYPE {
    new: "new",
    ft_transfer: "ft_transfer",
    ft_transfer_call: "ft_transfer_call",
    ft_total_supply: "ft_total_supply",
    ft_balance_of: "ft_balance_of",
    ft_resolve_transfer: "ft_resolve_transfer",
    storage_deposit: "storage_deposit",
    storage_withdraw: "storage_withdraw",
    storage_unregister: "storage_unregister",
    storage_balance_bounds: "storage_balance_bounds",
    storage_balance_of: "storage_balance_of",
    ft_metadata: "ft_metadata",
};
// =================================================
/// String constants for FT contract method names.
pub struct FT_METHODS_CONST_TYPE {
    pub new: &'static str,
    pub ft_transfer: &'static str,
    pub ft_transfer_call: &'static str,
    pub ft_total_supply: &'static str,
    pub ft_balance_of: &'static str,
    pub ft_resolve_transfer: &'static str,
    pub storage_deposit: &'static str,
    pub storage_withdraw: &'static str,
    pub storage_unregister: &'static str,
    pub storage_balance_bounds: &'static str,
    pub storage_balance_of: &'static str,
    pub ft_metadata: &'static str,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
