// =================================================
// Strategies for selecting the next RPC endpoint.
// =================================================

use std::sync::atomic::{AtomicUsize, Ordering};

/// Strategy for selecting the next RPC endpoint from a list of available endpoints.
///
/// Implementations must be `Send + Sync` so the [`LoadBalancingRpcClient`](crate::lib::load_balancer::load_balancer::LoadBalancingRpcClient)
/// can be shared across tasks. The picked index is always taken modulo `len`, so
/// implementations do not need to clamp themselves.
pub trait LoadBalancingStrategy: Send + Sync {
    /// Pick an index in `[0, len)`.
    fn next(&self, len: usize) -> usize;
}

/// Round-robin strategy — cycles through endpoints in order, advancing on every
/// call regardless of success or failure.
pub struct RoundRobin {
    counter: AtomicUsize,
}

impl RoundRobin {
    /// Create a new round-robin strategy starting at index 0.
    pub fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
        }
    }
}

impl Default for RoundRobin {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadBalancingStrategy for RoundRobin {
    fn next(&self, len: usize) -> usize {
        if len == 0 {
            return 0;
        }
        self.counter.fetch_add(1, Ordering::Relaxed) % len
    }
}

/// Random strategy — picks a pseudo-random endpoint for each call using the
/// system clock.
///
/// Cheap and stateless. Not cryptographically random — fine for spreading
/// load across public endpoints, not for anything security-sensitive.
pub struct Random;

impl LoadBalancingStrategy for Random {
    fn next(&self, len: usize) -> usize {
        if len == 0 {
            return 0;
        }
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as usize)
            .unwrap_or(0);
        nanos % len
    }
}

// =================================================
