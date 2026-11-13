//! Discord HTTP REST Client & Bucket Rate Limiting

use discord_core::snowflake::Snowflake;

pub const DISCORD_API_BASE: &str = "https://discord.com/api/v10";

/// Typed REST API Routes
pub enum Route {
    GetMe,
    GetGuild { guild_id: Snowflake },
    GetChannel { channel_id: Snowflake },
    GetChannelMessages { channel_id: Snowflake, limit: u32 },
    CreateMessage { channel_id: Snowflake },
    CreateReaction { channel_id: Snowflake, message_id: Snowflake, emoji: String },
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
            Route::CreateReaction { channel_id, message_id, emoji } => {
                format!("/channels/{}/messages/{}/reactions/{}/@me", channel_id, message_id, emoji)
            }
        }
    }
}
