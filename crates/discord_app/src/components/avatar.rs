//! Discord Avatar Component with Presence Status Cutout

use crate::theme::{Color, DiscordTheme};
use discord_core::models::PresenceStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarSize {
    Size24 = 24,
    Size32 = 32, // Channel list & User panel
    Size40 = 40, // Cozy chat message
    Size80 = 80, // User profile popout
}

#[derive(Debug, Clone)]
pub struct AvatarProps {
    pub image_url: Option<String>,
    pub fallback_initials: String,
    pub size: AvatarSize,
    pub status: Option<PresenceStatus>,
}

impl AvatarProps {
    pub fn new(fallback_initials: impl Into<String>, size: AvatarSize) -> Self {
        Self {
            image_url: None,
            fallback_initials: fallback_initials.into(),
            size,
            status: None,
        }
    }

    pub fn status_color(&self, theme: &DiscordTheme) -> Option<Color> {
        self.status.map(|s| match s {
            PresenceStatus::Online => theme.status_online,
            PresenceStatus::Idle => theme.status_idle,
            PresenceStatus::Dnd => theme.status_dnd,
            PresenceStatus::Offline | PresenceStatus::Invisible => theme.status_offline,
        })
    }
}
