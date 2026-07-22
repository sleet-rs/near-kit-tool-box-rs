// =================================================
/// Method name constants for the v1 standard staking-pool contract
/// (`code_hash`: `J1arLz48fgXcGyCPVckFwLnewNH6j1uw79thsvwqGYTY`).
///
/// Legacy v1 staking pool — a child contract deployed by the
/// `poolv1.near` factory. Each pool has its own account id, so no global
/// contract-id const is provided.
pub const POOL_STAKINGV1_METHODS_CONST: POOL_STAKINGV1_METHODS_CONST_TYPE =
    POOL_STAKINGV1_METHODS_CONST_TYPE {
        new: "new",
        ping: "ping",
        deposit: "deposit",
        deposit_and_stake: "deposit_and_stake",
        withdraw_all: "withdraw_all",
        withdraw: "withdraw",
        stake_all: "stake_all",
        stake: "stake",
        unstake_all: "unstake_all",
        unstake: "unstake",
        get_account_unstaked_balance: "get_account_unstaked_balance",
        get_account_staked_balance: "get_account_staked_balance",
        get_account_total_balance: "get_account_total_balance",
        is_account_unstaked_balance_available: "is_account_unstaked_balance_available",
        get_total_staked_balance: "get_total_staked_balance",
        get_owner_id: "get_owner_id",
        get_reward_fee_fraction: "get_reward_fee_fraction",
        get_staking_key: "get_staking_key",
        is_staking_paused: "is_staking_paused",
        get_account: "get_account",
        get_number_of_accounts: "get_number_of_accounts",
        get_accounts: "get_accounts",
        on_stake_action: "on_stake_action",
        update_staking_key: "update_staking_key",
        update_reward_fee_fraction: "update_reward_fee_fraction",
        vote: "vote",
        pause_staking: "pause_staking",
        resume_staking: "resume_staking",
    };
// =================================================
/// String constants for the v1 standard staking-pool contract method names.
pub struct POOL_STAKINGV1_METHODS_CONST_TYPE {
    pub new: &'static str,
    pub ping: &'static str,
    pub deposit: &'static str,
    pub deposit_and_stake: &'static str,
    pub withdraw_all: &'static str,
    pub withdraw: &'static str,
    pub stake_all: &'static str,
    pub stake: &'static str,
    pub unstake_all: &'static str,
    pub unstake: &'static str,
    pub get_account_unstaked_balance: &'static str,
    pub get_account_staked_balance: &'static str,
    pub get_account_total_balance: &'static str,
    pub is_account_unstaked_balance_available: &'static str,
    pub get_total_staked_balance: &'static str,
    pub get_owner_id: &'static str,
    pub get_reward_fee_fraction: &'static str,
    pub get_staking_key: &'static str,
    pub is_staking_paused: &'static str,
    pub get_account: &'static str,
    pub get_number_of_accounts: &'static str,
    pub get_accounts: &'static str,
    pub on_stake_action: &'static str,
    pub update_staking_key: &'static str,
    pub update_reward_fee_fraction: &'static str,
    pub vote: &'static str,
    pub pause_staking: &'static str,
    pub resume_staking: &'static str,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
