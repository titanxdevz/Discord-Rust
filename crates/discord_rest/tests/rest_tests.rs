use discord_rest::*;
use discord_core::snowflake::Snowflake;

#[tokio::test]
async fn test_rate_limiter_bucket_flow() {
    let limiter = RateLimiter::new();
    let channel_id = Snowflake::new(12345);
    let route = Route::CreateMessage { channel_id };
    let key = route.bucket_key();

    // First claim should succeed immediately without blocking
    limiter.claim(&key).await;

    // Simulate response with 0 remaining and 0.1s reset
    limiter.update_from_headers(&key, Some(5), Some(0), Some(0.05), false).await;

    // Next claim should block for the reset duration and then succeed
    let start = std::time::Instant::now();
    limiter.claim(&key).await;
    assert!(start.elapsed().as_millis() >= 40);
}
