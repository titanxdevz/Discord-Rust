use serde::{Deserialize, Serialize};
use crate::snowflake::Snowflake;
use crate::permissions::Permissions;
use super::user::User;

/// Discord Channel Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum ChannelType {
    GuildText,
    Dm,
    GuildVoice,
    GroupDm,
    GuildCategory,
    GuildAnnouncement,
    AnnouncementThread,
    PublicThread,
    PrivateThread,
    GuildStageVoice,
    GuildDirectory,
    GuildForum,
    GuildMedia,
    Unknown(u8),
}

impl From<u8> for ChannelType {
    fn from(val: u8) -> Self {
        match val {
            0 => ChannelType::GuildText,
            1 => ChannelType::Dm,
            2 => ChannelType::GuildVoice,
            3 => ChannelType::GroupDm,
            4 => ChannelType::GuildCategory,
            5 => ChannelType::GuildAnnouncement,
            10 => ChannelType::AnnouncementThread,
            11 => ChannelType::PublicThread,
            12 => ChannelType::PrivateThread,
            13 => ChannelType::GuildStageVoice,
            14 => ChannelType::GuildDirectory,
            15 => ChannelType::GuildForum,
            16 => ChannelType::GuildMedia,
            other => ChannelType::Unknown(other),
        }
    }
}

impl From<ChannelType> for u8 {
    fn from(ct: ChannelType) -> Self {
        match ct {
            ChannelType::GuildText => 0,
            ChannelType::Dm => 1,
            ChannelType::GuildVoice => 2,
            ChannelType::GroupDm => 3,
            ChannelType::GuildCategory => 4,
            ChannelType::GuildAnnouncement => 5,
            ChannelType::AnnouncementThread => 10,
            ChannelType::PublicThread => 11,
            ChannelType::PrivateThread => 12,
            ChannelType::GuildStageVoice => 13,
            ChannelType::GuildDirectory => 14,
            ChannelType::GuildForum => 15,
            ChannelType::GuildMedia => 16,
            ChannelType::Unknown(val) => val,
        }
    }
}

/// Channel Permission Overwrite
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionOverwrite {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub overwrite_type: u8, // 0 = role, 1 = member
    pub allow: Permissions,
    pub deny: Permissions,
}

/// Discord Channel
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Channel {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    pub guild_id: Option<Snowflake>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<Snowflake>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u32>,
    pub rate_limit_per_user: Option<u32>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<Snowflake>,
    pub parent_id: Option<Snowflake>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<u8>,
}
