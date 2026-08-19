// =================================================
// Public NEAR RPC endpoint lists for the load balancer.
// =================================================

/// Public NEAR testnet RPC endpoints used by the load balancer.
pub const NEAR_RPC_TESTNET_ENDPOINTS: &[&str] = &[
    "https://rpc.testnet.near.org",
    "https://test.rpc.fastnear.com",
    "https://testnet-rpc.intea.rs",
];

/// Public NEAR mainnet RPC endpoints used by the load balancer.
///
/// Note: `https://near.drpc.org` was removed because it does not support
/// the `EXPERIMENTAL_call_function` JSON-RPC method that near-kit uses
/// for view calls (returns HTTP 400 "method is not available").
pub const NEAR_RPC_MAINNET_ENDPOINTS: &[&str] = &[
    "https://rpc.mainnet.fastnear.com",
    "https://rpc.intea.rs",
    "https://rpc.shitzuapes.xyz",
];

// =================================================
