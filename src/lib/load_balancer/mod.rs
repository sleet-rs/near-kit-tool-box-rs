// =================================================
// Load balancer: distributes calls across multiple configured `Near` clients
// with round-robin pick and optional failover on retryable errors.
// =================================================
pub mod endpoints;
pub mod load_balancing_near;
pub mod strategies;
// =================================================