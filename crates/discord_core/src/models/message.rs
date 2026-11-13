use serde::{Deserialize, Serialize};
use crate::snowflake::Snowflake;
use super::user::User;
use super::guild::GuildMember;

/// Message Attachment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attachment {
    pub id: Snowflake,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: u64,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub ephemeral: Option<bool>,
}

/// Message Embed
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Embed {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub embed_type: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<u32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedMedia>,
    pub thumbnail: Option<EmbedMedia>,
    pub video: Option<EmbedMedia>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbedMedia {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

/// Message Reaction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reaction {
    pub count: u32,
    pub me: bool,
    pub emoji: ReactionEmoji,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactionEmoji {
    pub id: Option<Snowflake>,
    pub name: Option<String>,
    pub animated: Option<bool>,
}

/// Discord Message object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub author: User,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User>,
    pub mention_roles: Vec<Snowflake>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    pub pinned: bool,
    pub webhook_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub thread: Option<Box<super::channel::Channel>>,
}
