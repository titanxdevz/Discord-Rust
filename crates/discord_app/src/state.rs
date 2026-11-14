use std::collections::HashMap;
use discord_core::models::*;
use discord_core::snowflake::Snowflake;

/// Navigation state for the left sidebar and rail
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavigationTarget {
    DirectMessages,
    Guild(Snowflake),
}

/// Central application state
#[derive(Debug, Clone)]
pub struct AppState {
    pub current_user: Option<User>,
    pub navigation_target: NavigationTarget,
    pub active_channel_id: Option<Snowflake>,
    
    // Guild and channel stores
    pub guilds: Vec<Guild>,
    pub channels: HashMap<Snowflake, Vec<Channel>>, // Key: Guild ID (or DM parent)
    pub messages: HashMap<Snowflake, Vec<Message>>, // Key: Channel ID
    pub members: HashMap<Snowflake, Vec<GuildMember>>, // Key: Guild ID
    
    // Audio & Voice state
    pub active_voice_channel: Option<Snowflake>,
    pub is_muted: bool,
    pub is_deafened: bool,
    pub is_screen_sharing: bool,
    
    // UI toggles
    pub is_member_list_visible: bool,
    pub is_settings_open: bool,
    pub is_login_modal_open: bool,
    pub token_input: String,
    pub auth_status_message: Option<String>,
    pub search_query: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_user: None,
            navigation_target: NavigationTarget::DirectMessages,
            active_channel_id: None,
            guilds: Vec::new(),
            channels: HashMap::new(),
            messages: HashMap::new(),
            members: HashMap::new(),
            active_voice_channel: None,
            is_muted: false,
            is_deafened: false,
            is_screen_sharing: false,
            is_member_list_visible: true,
            is_settings_open: false,
            is_login_modal_open: false,
            token_input: String::new(),
            auth_status_message: None,
            search_query: String::new(),
        }
    }
}
