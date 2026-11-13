//! Discord Button Components (Primary Blurple, Secondary Grey, Danger Red, Link)

use crate::theme::{Color, DiscordTheme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,   // Brand Blurple (#5865f2)
    Secondary, // Neutral Grey (#4e5058)
    Danger,    // Red (#f23f43)
    Success,   // Green (#23a55a)
    Ghost,     // Transparent with hover
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,  // Height 32px, text 14px
    Medium, // Height 38px, text 14px
    Large,  // Height 44px, text 16px
}

#[derive(Debug, Clone)]
pub struct ButtonProps {
    pub label: String,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
}

impl ButtonProps {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            disabled: false,
        }
    }

    pub fn bg_color(&self, theme: &DiscordTheme, hovered: bool) -> Color {
        if self.disabled {
            return Color::rgba(255, 255, 255, 20);
        }
        match self.variant {
            ButtonVariant::Primary => {
                if hovered {
                    Color::from_hex(0x4752c4)
                } else {
                    theme.brand_blurple
                }
            }
            ButtonVariant::Secondary => {
                if hovered {
                    Color::from_hex(0x6d6f78)
                } else {
                    Color::from_hex(0x4e5058)
                }
            }
            ButtonVariant::Danger => {
                if hovered {
                    Color::from_hex(0xda373c)
                } else {
                    theme.status_dnd
                }
            }
            ButtonVariant::Success => {
                if hovered {
                    Color::from_hex(0x1f8b4c)
                } else {
                    theme.status_online
                }
            }
            ButtonVariant::Ghost => {
                if hovered {
                    theme.bg_hover
                } else {
                    Color::rgba(0, 0, 0, 0)
                }
            }
        }
    }
}
