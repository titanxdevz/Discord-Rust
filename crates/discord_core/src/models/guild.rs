use serde::{Deserialize, Serialize};
use crate::snowflake::Snowflake;
use crate::permissions::Permissions;
use super::user::User;

/// Guild Role
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Role {
    pub id: Snowflake,
    pub name: String,
    pub color: u32,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: i32,
    pub permissions: Permissions,
    pub managed: bool,
    pub mentionable: bool,
}

/// Guild Emoji
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Emoji {
    pub id: Option<Snowflake>,
    pub name: Option<String>,
    pub roles: Option<Vec<Snowflake>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

/// Guild Member
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuildMember {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<Snowflake>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub pending: Option<bool>,
    pub permissions: Option<Permissions>,
}

/// Discord Guild (Server)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Guild {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub splash: Option<String>,
    pub owner_id: Snowflake,
    pub afk_channel_id: Option<Snowflake>,
    pub afk_timeout: u32,
    pub roles: Vec<Role>,
    pub emojis: Vec<Emoji>,
    pub verification_level: u8,
    pub default_message_notifications: u8,
    pub explicit_content_filter: u8,
    pub vanity_url_code: Option<String>,
    pub premium_tier: u8,
    pub premium_subscription_count: Option<u32>,
    pub preferred_locale: String,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
}

impl Guild {
    pub fn icon_url(&self) -> Option<String> {
        self.icon.as_ref().map(|hash| {
            let ext = if hash.starts_with("a_") { "gif" } else { "webp" };
            format!("https://cdn.discordapp.com/icons/{}/{}.{}?size=128", self.id, hash, ext)
        })
    }

    /// Return guild initials for the fallback icon pill
    pub fn initials(&self) -> String {
        self.name
            .split_whitespace()
            .filter_map(|w| w.chars().next())
            .take(3)
            .collect()
    }
}
