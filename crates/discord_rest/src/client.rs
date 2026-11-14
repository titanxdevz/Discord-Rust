use std::sync::Arc;
use discord_core::models::*;
use discord_core::snowflake::Snowflake;
use crate::ratelimit::RateLimiter;
use crate::Route;
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
    http_client: reqwest::Client,
}

impl DiscordRestClient {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            rate_limiter: Arc::new(RateLimiter::new()),
            base_url: "https://discord.com/api/v10".to_string(),
            http_client: reqwest::Client::builder()
                .user_agent("DiscordRust/0.1.0 (DirectX12, GPUI)")
                .build()
                .unwrap_or_default(),
        }
    }

    pub fn rate_limiter(&self) -> &Arc<RateLimiter> {
        &self.rate_limiter
    }

    pub fn auth_header(&self) -> String {
        if self.token.starts_with("Bot ") || self.token.starts_with("Bearer ") {
            self.token.clone()
        } else {
            // User tokens or Bot tokens
            self.token.clone()
        }
    }

    /// Construct URL for route
    pub fn endpoint_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Verify token and fetch current user profile (@me)
    pub async fn get_current_user(&self) -> Result<User, RestError> {
        let route = Route::GetMe;
        self.rate_limiter.claim(&route.bucket_key()).await;

        let url = self.endpoint_url(&route.path());
        let res = self.http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| RestError::Http(e.to_string()))?;

        let status = res.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(RestError::Unauthorized);
        }

        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(RestError::Api {
                code: status.as_u16() as u32,
                message: body,
            });
        }

        let user = res.json::<User>().await.map_err(|e| RestError::Http(e.to_string()))?;
        Ok(user)
    }

    /// Fetch all guilds for the current authenticated user
    pub async fn get_user_guilds(&self) -> Result<Vec<Guild>, RestError> {
        let route = Route::GetCurrentUserGuilds;
        self.rate_limiter.claim(&route.bucket_key()).await;

        let url = self.endpoint_url(&route.path());
        let res = self.http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| RestError::Http(e.to_string()))?;

        if !res.status().is_success() {
            return Err(RestError::Api {
                code: res.status().as_u16() as u32,
                message: res.text().await.unwrap_or_default(),
            });
        }

        let guilds = res.json::<Vec<Guild>>().await.map_err(|e| RestError::Http(e.to_string()))?;
        Ok(guilds)
    }

    /// Fetch a single channel by ID
    pub async fn get_channel(&self, channel_id: Snowflake) -> Result<Channel, RestError> {
        let route = Route::GetChannel { channel_id };
        self.rate_limiter.claim(&route.bucket_key()).await;

        let url = self.endpoint_url(&route.path());
        let res = self.http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| RestError::Http(e.to_string()))?;

        if !res.status().is_success() {
            return Err(RestError::Api {
                code: res.status().as_u16() as u32,
                message: res.text().await.unwrap_or_default(),
            });
        }

        let channel = res.json::<Channel>().await.map_err(|e| RestError::Http(e.to_string()))?;
        Ok(channel)
    }

    /// Fetch guild channels
    pub async fn get_guild_channels(&self, guild_id: Snowflake) -> Result<Vec<Channel>, RestError> {
        let route = Route::GetGuild { guild_id };
        self.rate_limiter.claim(&route.bucket_key()).await;

        let url = format!("{}/guilds/{}/channels", self.base_url, guild_id);
        let res = self.http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| RestError::Http(e.to_string()))?;

        if !res.status().is_success() {
            return Err(RestError::Api {
                code: res.status().as_u16() as u32,
                message: res.text().await.unwrap_or_default(),
            });
        }

        let channels = res.json::<Vec<Channel>>().await.map_err(|e| RestError::Http(e.to_string()))?;
        Ok(channels)
    }

    /// Fetch channel messages
    pub async fn get_channel_messages(&self, channel_id: Snowflake, limit: u32) -> Result<Vec<Message>, RestError> {
        let route = Route::GetChannelMessages { channel_id, limit };
        self.rate_limiter.claim(&route.bucket_key()).await;

        let url = self.endpoint_url(&route.path());
        let res = self.http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| RestError::Http(e.to_string()))?;

        if !res.status().is_success() {
            return Err(RestError::Api {
                code: res.status().as_u16() as u32,
                message: res.text().await.unwrap_or_default(),
            });
        }

        let messages = res.json::<Vec<Message>>().await.map_err(|e| RestError::Http(e.to_string()))?;
        Ok(messages)
    }

    /// Send a message to a channel
    pub async fn send_message(&self, channel_id: Snowflake, content: &str) -> Result<Message, RestError> {
        let route = Route::CreateMessage { channel_id };
        self.rate_limiter.claim(&route.bucket_key()).await;

        let payload = Self::build_message_payload(Some(content), None, None);
        let url = self.endpoint_url(&route.path());
        let res = self.http_client
            .post(&url)
            .header("Authorization", self.auth_header())
            .json(&payload)
            .send()
            .await
            .map_err(|e| RestError::Http(e.to_string()))?;

        if !res.status().is_success() {
            return Err(RestError::Api {
                code: res.status().as_u16() as u32,
                message: res.text().await.unwrap_or_default(),
            });
        }

        let message = res.json::<Message>().await.map_err(|e| RestError::Http(e.to_string()))?;
        Ok(message)
    }

    /// Add a reaction to a message
    pub async fn add_reaction(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        emoji: &str,
    ) -> Result<(), RestError> {
        let route = Route::CreateReaction {
            channel_id,
            message_id,
            emoji: emoji.to_string(),
        };
        self.rate_limiter.claim(&route.bucket_key()).await;

        let encoded_emoji = emoji;
        let url = format!("{}/channels/{}/messages/{}/reactions/{}/@me", self.base_url, channel_id, message_id, encoded_emoji);
        let res = self.http_client
            .put(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Length", "0")
            .send()
            .await
            .map_err(|e| RestError::Http(e.to_string()))?;

        if !res.status().is_success() {
            return Err(RestError::Api {
                code: res.status().as_u16() as u32,
                message: res.text().await.unwrap_or_default(),
            });
        }

        Ok(())
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
