//! Discord Badges, Unread Pills, and Role Tags

use crate::theme::{Color, DiscordTheme};

#[derive(Debug, Clone)]
pub struct MentionBadgeProps {
    pub count: u32,
}

impl MentionBadgeProps {
    pub fn new(count: u32) -> Self {
        Self { count }
    }

    pub fn bg_color(&self, theme: &DiscordTheme) -> Color {
        theme.status_dnd // Red #f23f43
    }
}

/// Server Rail Selection Pill (Left Edge)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RailPillState {
    None,
    Unread,   // 8px tall rounded pill
    Hovered,  // 20px tall rounded pill
    Selected, // 40px tall rounded pill
}

impl RailPillState {
    pub fn height(&self) -> f32 {
        match self {
            RailPillState::None => 0.0,
            RailPillState::Unread => 8.0,
            RailPillState::Hovered => 20.0,
            RailPillState::Selected => 40.0,
        }
    }
}
