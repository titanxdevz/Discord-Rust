//! Canonical Discord Design Tokens & Color Palette

/// RGBA 32-bit Color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a: 255,
        }
    }
}

pub struct DiscordTheme {
    // Surface backgrounds
    pub bg_chat: Color,           // #313338 - Main chat background
    pub bg_sidebar: Color,        // #2b2d31 - Channels and DMs
    pub bg_user_panel: Color,     // #232428 - Bottom user account bar
    pub bg_server_rail: Color,    // #1e1f22 - Left guild navigation rail
    pub bg_floating: Color,       // #111214 - Modals, context menus, tooltips
    pub bg_hover: Color,          // rgba(255, 255, 255, 0.07)
    pub bg_active: Color,         // rgba(255, 255, 255, 0.10)
    pub bg_selected: Color,       // rgba(255, 255, 255, 0.12)
    pub bg_composer: Color,       // #383a40 - Message input box

    // Brand and status accents
    pub brand_blurple: Color,     // #5865f2 - Discord Blurple
    pub status_online: Color,     // #23a55a - Green
    pub status_idle: Color,       // #f0b232 - Amber / Yellow
    pub status_dnd: Color,        // #f23f43 - Red
    pub status_offline: Color,    // #80848e - Grey
    pub status_streaming: Color,  // #593695 - Purple

    // Text hierarchy
    pub text_header: Color,       // #f2f3f5 - Primary headings
    pub text_normal: Color,       // #dbdee1 - Message body
    pub text_muted: Color,        // #949ba4 - Channels, timestamps, secondary
    pub text_link: Color,         // #00a8fc - Hyperlinks
    pub text_danger: Color,       // #fa777c - Errors / Leave server

    // Layout dimensions (exact pixel metrics)
    pub server_rail_width: f32,   // 72.0 px
    pub sidebar_width: f32,       // 240.0 px
    pub member_list_width: f32,   // 240.0 px
    pub header_height: f32,       // 48.0 px
    pub user_panel_height: f32,   // 52.0 px
    pub composer_min_height: f32, // 44.0 px
}

impl Default for DiscordTheme {
    fn default() -> Self {
        Self {
            bg_chat: Color::from_hex(0x313338),
            bg_sidebar: Color::from_hex(0x2b2d31),
            bg_user_panel: Color::from_hex(0x232428),
            bg_server_rail: Color::from_hex(0x1e1f22),
            bg_floating: Color::from_hex(0x111214),
            bg_hover: Color::rgba(255, 255, 255, 18),
            bg_active: Color::rgba(255, 255, 255, 26),
            bg_selected: Color::rgba(255, 255, 255, 31),
            bg_composer: Color::from_hex(0x383a40),

            brand_blurple: Color::from_hex(0x5865f2),
            status_online: Color::from_hex(0x23a55a),
            status_idle: Color::from_hex(0xf0b232),
            status_dnd: Color::from_hex(0xf23f43),
            status_offline: Color::from_hex(0x80848e),
            status_streaming: Color::from_hex(0x593695),

            text_header: Color::from_hex(0xf2f3f5),
            text_normal: Color::from_hex(0xdbdee1),
            text_muted: Color::from_hex(0x949ba4),
            text_link: Color::from_hex(0x00a8fc),
            text_danger: Color::from_hex(0xfa777c),

            server_rail_width: 72.0,
            sidebar_width: 240.0,
            member_list_width: 240.0,
            header_height: 48.0,
            user_panel_height: 52.0,
            composer_min_height: 44.0,
        }
    }
}
