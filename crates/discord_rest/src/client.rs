use std::sync::Arc;
use discord_core::models::*;
use discord_core::snowflake::Snowflake;
use crate::ratelimit::RateLimiter;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RestError {
    #[error("HTTP request error: {0}")]
    Http(String),
    #[error("Discord API error (code {code}): {message}")]
    Api { code: u32, message: String },
    #[error("Rate limited (retry after {0} seconds)")]
    RateLimited(f64),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Unauthorized: invalid or missing authentication token")]
    Unauthorized,
}

/// Discord REST Client with connection pooling and rate limiting
#[derive(Clone)]
pub struct DiscordRestClient {
    token: String,
    rate_limiter: Arc<RateLimiter>,
    base_url: String,
}

impl DiscordRestClient {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            rate_limiter: Arc::new(RateLimiter::new()),
            base_url: "https://discord.com/api/v10".to_string(),
        }
    }

    pub fn rate_limiter(&self) -> &Arc<RateLimiter> {
        &self.rate_limiter
    }

    pub fn auth_header(&self) -> String {
        if self.token.starts_with("Bot ") || self.token.starts_with("Bearer ") {
            self.token.clone()
        } else {
            format!("Bot {}", self.token)
        }
    }

    /// Construct URL for route
    pub fn endpoint_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Prepares a create message payload
    pub fn build_message_payload(
        content: Option<&str>,
        embeds: Option<Vec<Embed>>,
        message_reference_id: Option<Snowflake>,
    ) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        if let Some(c) = content {
            map.insert("content".to_string(), serde_json::json!(c));
        }
        if let Some(e) = embeds {
            map.insert("embeds".to_string(), serde_json::json!(e));
        }
        if let Some(ref_id) = message_reference_id {
            map.insert(
                "message_reference".to_string(),
                serde_json::json!({
                    "message_id": ref_id.to_string()
                }),
            );
        }
        serde_json::Value::Object(map)
    }
}
