// =================================================
/// Method name constants for the `poolv1.near` v1 staking-pool factory contract.
///
/// Legacy / simpler factory for v1 staking pools. The two networks should match
/// the same code hash; verify with `near contract inspect`.
pub const POOL_FACTORYV1_METHODS_CONST: POOL_FACTORYV1_METHODS_CONST_TYPE =
    POOL_FACTORYV1_METHODS_CONST_TYPE {
        new: "new",
        get_min_attached_balance: "get_min_attached_balance",
        get_number_of_staking_pools_created: "get_number_of_staking_pools_created",
        create_staking_pool: "create_staking_pool",
        on_staking_pool_create: "on_staking_pool_create",
    };
// =================================================
/// String constants for poolv1.near factory contract method names.
pub struct POOL_FACTORYV1_METHODS_CONST_TYPE {
    pub new: &'static str,
    pub get_min_attached_balance: &'static str,
    pub get_number_of_staking_pools_created: &'static str,
    pub create_staking_pool: &'static str,
    pub on_staking_pool_create: &'static str,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
