// =================================================
// RPC load balancer for NEAR — distributes calls across multiple endpoints
// with round-robin pick and optional failover on retryable errors.
// =================================================
pub mod endpoints;
pub mod load_balancing_rpc_client;
pub mod strategies;
// =================================================
