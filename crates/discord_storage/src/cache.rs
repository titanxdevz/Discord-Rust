use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use discord_core::models::*;
use discord_core::snowflake::Snowflake;

/// Local Client Cache Database
#[derive(Debug, Clone, Default)]
pub struct CacheStore {
    pub current_user: Arc<RwLock<Option<User>>>,
    pub guilds: Arc<RwLock<HashMap<Snowflake, Guild>>>,
    pub channels: Arc<RwLock<HashMap<Snowflake, Channel>>>,
    pub messages: Arc<RwLock<HashMap<Snowflake, Vec<Message>>>>, // Channel ID -> Messages
    pub members: Arc<RwLock<HashMap<Snowflake, Vec<GuildMember>>>>, // Guild ID -> Members
    pub storage_dir: Option<PathBuf>,
}

impl CacheStore {
    pub fn new(storage_dir: Option<impl AsRef<Path>>) -> Self {
        Self {
            current_user: Arc::new(RwLock::new(None)),
            guilds: Arc::new(RwLock::new(HashMap::new())),
            channels: Arc::new(RwLock::new(HashMap::new())),
            messages: Arc::new(RwLock::new(HashMap::new())),
            members: Arc::new(RwLock::new(HashMap::new())),
            storage_dir: storage_dir.map(|p| p.as_ref().to_path_buf()),
        }
    }

    /// Set current logged in user
    pub async fn set_current_user(&self, user: User) {
        let mut u = self.current_user.write().await;
        *u = Some(user);
    }

    /// Insert or update guild
    pub async fn upsert_guild(&self, guild: Guild) {
        let mut g = self.guilds.write().await;
        g.insert(guild.id, guild);
    }

    /// Insert or update channel
    pub async fn upsert_channel(&self, channel: Channel) {
        let mut c = self.channels.write().await;
        c.insert(channel.id, channel);
    }

    /// Append message to channel message stream
    pub async fn append_message(&self, message: Message) {
        let mut m = self.messages.write().await;
        let list = m.entry(message.channel_id).or_default();
        // Prevent duplicate messages
        if !list.iter().any(|existing| existing.id == message.id) {
            list.push(message);
        }
    }

    /// Edit existing message in cache
    pub async fn edit_message(&self, updated: Message) {
        let mut m = self.messages.write().await;
        if let Some(list) = m.get_mut(&updated.channel_id) {
            if let Some(existing) = list.iter_mut().find(|msg| msg.id == updated.id) {
                *existing = updated;
            }
        }
    }

    /// Delete message from cache
    pub async fn delete_message(&self, channel_id: Snowflake, message_id: Snowflake) {
        let mut m = self.messages.write().await;
        if let Some(list) = m.get_mut(&channel_id) {
            list.retain(|msg| msg.id != message_id);
        }
    }

    /// Get messages for channel
    pub async fn get_messages(&self, channel_id: Snowflake) -> Vec<Message> {
        let m = self.messages.read().await;
        m.get(&channel_id).cloned().unwrap_or_default()
    }
}
