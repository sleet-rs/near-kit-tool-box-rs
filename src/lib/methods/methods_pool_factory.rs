// =================================================
/// Method name constants for the `pool.near` staking-pool factory contract.
///
/// Deploys new validator staking-pool contracts. The two networks should match
/// the same code hash; verify with `near contract inspect`.
pub const POOL_FACTORY_METHODS_CONST: POOL_FACTORY_METHODS_CONST_TYPE = POOL_FACTORY_METHODS_CONST_TYPE {
    store: "store",
    new: "new",
    get_min_attached_balance: "get_min_attached_balance",
    get_number_of_staking_pools_created: "get_number_of_staking_pools_created",
    create_staking_pool: "create_staking_pool",
    on_staking_pool_create: "on_staking_pool_create",
    get_code: "get_code",
    allow_contract: "allow_contract",
    disallow_contract: "disallow_contract",
    is_contract_allowed: "is_contract_allowed",
};
// =================================================
/// String constants for pool.near factory contract method names.
pub struct POOL_FACTORY_METHODS_CONST_TYPE {
    pub store: &'static str,
    pub new: &'static str,
    pub get_min_attached_balance: &'static str,
    pub get_number_of_staking_pools_created: &'static str,
    pub create_staking_pool: &'static str,
    pub on_staking_pool_create: &'static str,
    pub get_code: &'static str,
    pub allow_contract: &'static str,
    pub disallow_contract: &'static str,
    pub is_contract_allowed: &'static str,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near