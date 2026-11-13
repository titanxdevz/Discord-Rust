use serde::{Deserialize, Serialize};
use discord_core::models::*;
use discord_core::snowflake::Snowflake;

/// Event payload received in Opcode 0 Dispatch
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum GatewayEvent {
    #[serde(rename = "READY")]
    Ready(ReadyPayload),

    #[serde(rename = "RESUMED")]
    Resumed,

    #[serde(rename = "MESSAGE_CREATE")]
    MessageCreate(Message),

    #[serde(rename = "MESSAGE_UPDATE")]
    MessageUpdate(Message),

    #[serde(rename = "MESSAGE_DELETE")]
    MessageDelete(MessageDeletePayload),

    #[serde(rename = "GUILD_CREATE")]
    GuildCreate(Guild),

    #[serde(rename = "GUILD_UPDATE")]
    GuildUpdate(Guild),

    #[serde(rename = "GUILD_DELETE")]
    GuildDelete(UnavailableGuild),

    #[serde(rename = "CHANNEL_CREATE")]
    ChannelCreate(Channel),

    #[serde(rename = "CHANNEL_UPDATE")]
    ChannelUpdate(Channel),

    #[serde(rename = "CHANNEL_DELETE")]
    ChannelDelete(Channel),

    #[serde(rename = "VOICE_STATE_UPDATE")]
    VoiceStateUpdate(VoiceState),

    #[serde(rename = "VOICE_SERVER_UPDATE")]
    VoiceServerUpdate(VoiceServerUpdatePayload),

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyPayload {
    pub v: u8,
    pub user: User,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub resume_gateway_url: String,
    pub shard: Option<[u32; 2]>,
    pub application: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnavailableGuild {
    pub id: Snowflake,
    pub unavailable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDeletePayload {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceServerUpdatePayload {
    pub token: String,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub endpoint: Option<String>,
}
