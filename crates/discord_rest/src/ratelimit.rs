use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Per-route bucket state tracking Discord's X-RateLimit headers
#[derive(Debug, Clone)]
pub struct BucketState {
    pub limit: u32,
    pub remaining: u32,
    pub reset_at: Instant,
}

impl Default for BucketState {
    fn default() -> Self {
        Self {
            limit: 1,
            remaining: 1,
            reset_at: Instant::now(),
        }
    }
}

/// Token Bucket Rate Limiter for Discord HTTP REST API
#[derive(Debug, Clone, Default)]
pub struct RateLimiter {
    buckets: Arc<Mutex<HashMap<String, BucketState>>>,
    global_reset_at: Arc<Mutex<Option<Instant>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
            global_reset_at: Arc::new(Mutex::new(None)),
        }
    }

    /// Pre-request claim: waits if bucket is currently exhausted or if a global rate limit is active
    pub async fn claim(&self, route_key: &str) {
        // 1. Check global rate limit
        let global_wait = {
            let global = self.global_reset_at.lock().await;
            if let Some(reset) = *global {
                let now = Instant::now();
                if reset > now {
                    Some(reset - now)
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(duration) = global_wait {
            tracing::warn!("Global rate limit active, waiting {:?}", duration);
            tokio::time::sleep(duration).await;
            let mut global = self.global_reset_at.lock().await;
            *global = None;
        }

        // 2. Check route-specific bucket
        let route_wait = {
            let mut buckets = self.buckets.lock().await;
            let state = buckets.entry(route_key.to_string()).or_default();
            let now = Instant::now();

            if now >= state.reset_at {
                state.remaining = state.limit;
            }

            if state.remaining == 0 {
                if state.reset_at > now {
                    Some(state.reset_at - now)
                } else {
                    None
                }
            } else {
                state.remaining -= 1;
                None
            }
        };

        if let Some(duration) = route_wait {
            tracing::warn!("Route bucket '{}' exhausted, waiting {:?}", route_key, duration);
            tokio::time::sleep(duration).await;
            // Replenish bucket after sleep
            let mut buckets = self.buckets.lock().await;
            if let Some(state) = buckets.get_mut(route_key) {
                state.remaining = state.limit.saturating_sub(1);
            }
        }
    }

    /// Post-response update: updates bucket state from Discord HTTP response headers
    pub async fn update_from_headers(
        &self,
        route_key: &str,
        limit: Option<u32>,
        remaining: Option<u32>,
        reset_after_secs: Option<f64>,
        is_global: bool,
    ) {
        if is_global {
            if let Some(secs) = reset_after_secs {
                let mut global = self.global_reset_at.lock().await;
                *global = Some(Instant::now() + Duration::from_secs_f64(secs));
            }
            return;
        }

        if let (Some(l), Some(r), Some(secs)) = (limit, remaining, reset_after_secs) {
            let mut buckets = self.buckets.lock().await;
            let state = buckets.entry(route_key.to_string()).or_default();
            state.limit = l;
            state.remaining = r;
            state.reset_at = Instant::now() + Duration::from_secs_f64(secs);
        }
    }
}
