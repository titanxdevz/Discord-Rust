use serde::{Deserialize, Serialize};

/// Discord Activity Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum ActivityType {
    Game,
    Streaming,
    Listening,
    Watching,
    Custom,
    Competing,
    Unknown(u8),
}

impl From<u8> for ActivityType {
    fn from(val: u8) -> Self {
        match val {
            0 => ActivityType::Game,
            1 => ActivityType::Streaming,
            2 => ActivityType::Listening,
            3 => ActivityType::Watching,
            4 => ActivityType::Custom,
            5 => ActivityType::Competing,
            other => ActivityType::Unknown(other),
        }
    }
}

impl From<ActivityType> for u8 {
    fn from(at: ActivityType) -> Self {
        match at {
            ActivityType::Game => 0,
            ActivityType::Streaming => 1,
            ActivityType::Listening => 2,
            ActivityType::Watching => 3,
            ActivityType::Custom => 4,
            ActivityType::Competing => 5,
            ActivityType::Unknown(val) => val,
        }
    }
}

/// Discord Rich Presence Activity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Activity {
    pub name: String,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    pub url: Option<String>,
    pub created_at: u64,
    pub timestamps: Option<ActivityTimestamps>,
    pub application_id: Option<String>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<ActivityEmoji>,
    pub party: Option<ActivityParty>,
    pub assets: Option<ActivityAssets>,
    pub secrets: Option<ActivitySecrets>,
    pub instance: Option<bool>,
    pub flags: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityTimestamps {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityEmoji {
    pub name: String,
    pub id: Option<String>,
    pub animated: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityParty {
    pub id: Option<String>,
    pub size: Option<[u32; 2]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivitySecrets {
    pub join: Option<String>,
    pub spectate: Option<String>,
    pub r#match: Option<String>,
}
