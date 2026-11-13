//! Discord HTTP REST Client & Bucket Rate Limiting

pub mod ratelimit;
pub mod client;

pub use ratelimit::{RateLimiter, BucketState};
pub use client::{DiscordRestClient, RestError};
use discord_core::snowflake::Snowflake;

pub const DISCORD_API_BASE: &str = "https://discord.com/api/v10";

/// Typed REST API Routes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    GetMe,
    GetGuild { guild_id: Snowflake },
    GetChannel { channel_id: Snowflake },
    GetChannelMessages { channel_id: Snowflake, limit: u32 },
    CreateMessage { channel_id: Snowflake },
    EditMessage { channel_id: Snowflake, message_id: Snowflake },
    DeleteMessage { channel_id: Snowflake, message_id: Snowflake },
    CreateReaction { channel_id: Snowflake, message_id: Snowflake, emoji: String },
    DeleteReaction { channel_id: Snowflake, message_id: Snowflake, emoji: String },
    GetPinnedMessages { channel_id: Snowflake },
    PinMessage { channel_id: Snowflake, message_id: Snowflake },
    UnpinMessage { channel_id: Snowflake, message_id: Snowflake },
}

impl Route {
    pub fn path(&self) -> String {
        match self {
            Route::GetMe => "/users/@me".to_string(),
            Route::GetGuild { guild_id } => format!("/guilds/{}", guild_id),
            Route::GetChannel { channel_id } => format!("/channels/{}", channel_id),
            Route::GetChannelMessages { channel_id, limit } => {
                format!("/channels/{}/messages?limit={}", channel_id, limit)
            }
            Route::CreateMessage { channel_id } => format!("/channels/{}/messages", channel_id),
            Route::EditMessage { channel_id, message_id } => {
                format!("/channels/{}/messages/{}", channel_id, message_id)
            }
            Route::DeleteMessage { channel_id, message_id } => {
                format!("/channels/{}/messages/{}", channel_id, message_id)
            }
            Route::CreateReaction { channel_id, message_id, emoji } => {
                format!("/channels/{}/messages/{}/reactions/{}/@me", channel_id, message_id, emoji)
            }
            Route::DeleteReaction { channel_id, message_id, emoji } => {
                format!("/channels/{}/messages/{}/reactions/{}/@me", channel_id, message_id, emoji)
            }
            Route::GetPinnedMessages { channel_id } => format!("/channels/{}/pins", channel_id),
            Route::PinMessage { channel_id, message_id } => {
                format!("/channels/{}/pins/{}", channel_id, message_id)
            }
            Route::UnpinMessage { channel_id, message_id } => {
                format!("/channels/{}/pins/{}", channel_id, message_id)
            }
        }
    }

    /// Key for grouping route rate limits by endpoint bucket
    pub fn bucket_key(&self) -> String {
        match self {
            Route::GetMe => "users:@me".to_string(),
            Route::GetGuild { guild_id } => format!("guilds:{}", guild_id),
            Route::GetChannel { channel_id } => format!("channels:{}", channel_id),
            Route::GetChannelMessages { channel_id, .. } => format!("channels:{}:messages", channel_id),
            Route::CreateMessage { channel_id } => format!("channels:{}:messages", channel_id),
            Route::EditMessage { channel_id, .. } => format!("channels:{}:messages", channel_id),
            Route::DeleteMessage { channel_id, .. } => format!("channels:{}:messages:delete", channel_id),
            Route::CreateReaction { channel_id, message_id, .. } => {
                format!("channels:{}:messages:{}:reactions", channel_id, message_id)
            }
            Route::DeleteReaction { channel_id, message_id, .. } => {
                format!("channels:{}:messages:{}:reactions", channel_id, message_id)
            }
            Route::GetPinnedMessages { channel_id } => format!("channels:{}:pins", channel_id),
            Route::PinMessage { channel_id, .. } => format!("channels:{}:pins", channel_id),
            Route::UnpinMessage { channel_id, .. } => format!("channels:{}:pins", channel_id),
        }
    }
}
