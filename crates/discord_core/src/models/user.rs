use serde::{Deserialize, Serialize};
use crate::snowflake::Snowflake;

/// User Presence Status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PresenceStatus {
    Online,
    Idle,
    Dnd,
    Offline,
    Invisible,
}

impl Default for PresenceStatus {
    fn default() -> Self {
        Self::Offline
    }
}

/// Discord User object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<u32>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<u64>,
    pub premium_type: Option<u8>,
    pub public_flags: Option<u64>,
}

impl User {
    /// Return the display name (global name if present, otherwise username)
    pub fn display_name(&self) -> &str {
        if let Some(ref gn) = self.global_name {
            gn.as_str()
        } else {
            self.username.as_str()
        }
    }

    /// Return CDN avatar URL or default avatar URL
    pub fn avatar_url(&self) -> String {
        if let Some(ref hash) = self.avatar {
            let ext = if hash.starts_with("a_") { "gif" } else { "webp" };
            format!("https://cdn.discordapp.com/avatars/{}/{}.{}?size=128", self.id, hash, ext)
        } else {
            let index = if self.discriminator == "0" {
                (self.id.as_u64() >> 22) % 6
            } else {
                self.discriminator.parse::<u64>().unwrap_or(0) % 5
            };
            format!("https://cdn.discordapp.com/embed/avatars/{}.png", index)
        }
    }
}
