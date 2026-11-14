//! GPUI Native Rendering Shell for Discord Rust

use gpui::{
    div, prelude::*, px, rgb, App, Application, Context, IntoElement, Render, Window,
    WindowBounds, WindowOptions, Bounds, Point, Size,
};
use crate::state::{AppState, NavigationTarget};
use crate::theme::DiscordTheme;
use discord_core::models::*;
use discord_core::snowflake::Snowflake;

/// Main Root Desktop View for Discord Rust
pub struct DiscordRustAppView {
    pub state: AppState,
    pub theme: DiscordTheme,
    pub new_message_text: String,
}

impl DiscordRustAppView {
    pub fn new() -> Self {
        let mut state = AppState::default();
        let theme = DiscordTheme::default();

        // Populate initial Discord Rust state
        let current_user = User {
            id: Snowflake::new(1001),
            username: "rustacean".to_string(),
            discriminator: "0".to_string(),
            global_name: Some("Ferris".to_string()),
            avatar: None,
            bot: None,
            system: None,
            mfa_enabled: Some(true),
            banner: None,
            accent_color: Some(0xE05D44), // Rust Logo Copper
            locale: Some("en-US".to_string()),
            verified: Some(true),
            email: None,
            flags: None,
            premium_type: Some(2), // Nitro
            public_flags: None,
        };
        state.current_user = Some(current_user.clone());

        let rust_guild = Guild {
            id: Snowflake::new(5001),
            name: "Rust Community".to_string(),
            icon: None,
            description: Some("Official Rust Desktop Community".to_string()),
            banner: None,
            splash: None,
            owner_id: Snowflake::new(1001),
            afk_channel_id: None,
            afk_timeout: 300,
            roles: vec![
                Role {
                    id: Snowflake::new(5001),
                    name: "@everyone".to_string(),
                    color: 0,
                    hoist: false,
                    icon: None,
                    unicode_emoji: None,
                    position: 0,
                    permissions: discord_core::Permissions::VIEW_CHANNEL | discord_core::Permissions::SEND_MESSAGES,
                    managed: false,
                    mentionable: false,
                },
                Role {
                    id: Snowflake::new(6001),
                    name: "Moderator".to_string(),
                    color: 0x23A55A,
                    hoist: true,
                    icon: None,
                    unicode_emoji: None,
                    position: 1,
                    permissions: discord_core::Permissions::ADMINISTRATOR,
                    managed: false,
                    mentionable: true,
                },
            ],
            emojis: Vec::new(),
            verification_level: 1,
            default_message_notifications: 0,
            explicit_content_filter: 2,
            vanity_url_code: Some("rust-desktop".to_string()),
            premium_tier: 3,
            premium_subscription_count: Some(15),
            preferred_locale: "en-US".to_string(),
            approximate_member_count: Some(1420),
            approximate_presence_count: Some(680),
        };

        let general_text = Channel {
            id: Snowflake::new(7001),
            channel_type: ChannelType::GuildText,
            guild_id: Some(rust_guild.id),
            position: Some(0),
            permission_overwrites: None,
            name: Some("general".to_string()),
            topic: Some("General discussion about Discord Rust, GPUI and high performance UI".to_string()),
            nsfw: Some(false),
            last_message_id: None,
            bitrate: None,
            user_limit: None,
            rate_limit_per_user: None,
            recipients: None,
            icon: None,
            owner_id: None,
            parent_id: None,
            last_pin_timestamp: None,
            rtc_region: None,
            video_quality_mode: None,
        };

        let lounge_voice = Channel {
            id: Snowflake::new(7002),
            channel_type: ChannelType::GuildVoice,
            guild_id: Some(rust_guild.id),
            position: Some(1),
            permission_overwrites: None,
            name: Some("General Voice".to_string()),
            topic: None,
            nsfw: Some(false),
            last_message_id: None,
            bitrate: Some(96000),
            user_limit: Some(10),
            rate_limit_per_user: None,
            recipients: None,
            icon: None,
            owner_id: None,
            parent_id: None,
            last_pin_timestamp: None,
            rtc_region: Some("us-east".to_string()),
            video_quality_mode: Some(1),
        };

        let welcome_msg = Message {
            id: Snowflake::new(8001),
            channel_id: general_text.id,
            author: current_user.clone(),
            content: "Welcome to Discord Rust! Running natively at 120+ FPS with GPUI, DirectX 12, sub-100MB RAM, and DAVE E2EE.".to_string(),
            timestamp: "2026-11-13T23:45:00Z".to_string(),
            edited_timestamp: None,
            tts: false,
            mention_everyone: false,
            mentions: Vec::new(),
            mention_roles: Vec::new(),
            attachments: Vec::new(),
            embeds: vec![
                Embed {
                    title: Some("Discord Rust Performance Architecture".to_string()),
                    embed_type: Some("rich".to_string()),
                    description: Some("Built clean-room in pure Rust with DAVE E2EE (MLS RFC 9420) and low-latency custom RTP voice streaming.".to_string()),
                    url: Some("https://github.com/morluto/rea".to_string()),
                    timestamp: None,
                    color: Some(0xE05D44),
                    footer: Some(EmbedFooter {
                        text: "Discord Rust Desktop Client".to_string(),
                        icon_url: None,
                        proxy_icon_url: None,
                    }),
                    image: None,
                    thumbnail: None,
                    video: None,
                    author: None,
                    fields: Some(vec![
                        EmbedField {
                            name: "Cold Startup".to_string(),
                            value: "< 150 ms".to_string(),
                            inline: Some(true),
                        },
                        EmbedField {
                            name: "Memory Usage".to_string(),
                            value: "~45 - 80 MB".to_string(),
                            inline: Some(true),
                        },
                        EmbedField {
                            name: "Voice Encryption".to_string(),
                            value: "AEAD AES-256-GCM (_rtpsize)".to_string(),
                            inline: Some(true),
                        },
                    ]),
                }
            ],
            reactions: Some(vec![
                Reaction {
                    count: 12,
                    me: true,
                    emoji: ReactionEmoji {
                        id: None,
                        name: Some("🚀".to_string()),
                        animated: Some(false),
                    },
                },
                Reaction {
                    count: 8,
                    me: false,
                    emoji: ReactionEmoji {
                        id: None,
                        name: Some("🦀".to_string()),
                        animated: Some(false),
                    },
                },
            ]),
            pinned: true,
            webhook_id: None,
            member: None,
            thread: None,
        };

        state.guilds.push(rust_guild.clone());
        state.channels.insert(rust_guild.id, vec![general_text.clone(), lounge_voice.clone()]);
        state.messages.insert(general_text.id, vec![welcome_msg]);
        state.navigation_target = NavigationTarget::Guild(rust_guild.id);
        state.active_channel_id = Some(general_text.id);
        state.active_voice_channel = Some(lounge_voice.id);

        Self {
            state,
            theme,
            new_message_text: String::new(),
        }
    }
}

impl Render for DiscordRustAppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_voice_connected = self.state.active_voice_channel.is_some();
        let is_muted = self.state.is_muted;
        let is_deafened = self.state.is_deafened;
        let is_settings_open = self.state.is_settings_open;
        let active_channel_name = self.state.active_channel_id.map(|_| "general").unwrap_or("general");

        // 1. Root Container (Full Window)
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1f22))
            .text_color(rgb(0xdbdee1))
            // 2. Custom Windows 11 Titlebar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .h(px(32.0))
                    .w_full()
                    .bg(rgb(0x1e1f22))
                    .items_center()
                    .justify_between()
                    .px(px(12.0))
                    .text_xs()
                    .text_color(rgb(0x949ba4))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.0))
                            // Discord Logo in Rust Color (#E05D44)
                            .child(
                                div()
                                    .w(px(16.0))
                                    .h(px(16.0))
                                    .bg(rgb(0xe05d44))
                                    .rounded(px(4.0))
                            )
                            .child("Discord Rust — #general | Rust Community")
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(12.0))
                            .child("_")
                            .child("□")
                            .child("✕")
                    )
            )
            // 3. Main Workspace Area (Rail + Sidebar + Chat + Members)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .size_full()
                    // 3A. Server Rail (72px)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w(px(72.0))
                            .h_full()
                            .bg(rgb(0x1e1f22))
                            .items_center()
                            .py(px(12.0))
                            .gap(px(8.0))
                            // Discord Rust Home Button (Logo in Rust color)
                            .child(
                                div()
                                    .w(px(48.0))
                                    .h(px(48.0))
                                    .bg(rgb(0xe05d44))
                                    .rounded(px(16.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .child("🦀")
                                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                        this.state.navigation_target = NavigationTarget::DirectMessages;
                                        cx.notify();
                                    }))
                            )
                            // 32x2px Separator
                            .child(
                                div()
                                    .w(px(32.0))
                                    .h(px(2.0))
                                    .bg(rgb(0x35363c))
                                    .rounded(px(1.0))
                            )
                            // Rust Community Guild Icon
                            .child(
                                div()
                                    .w(px(48.0))
                                    .h(px(48.0))
                                    .bg(rgb(0x313338))
                                    .rounded(px(16.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_sm()
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .child("RC")
                                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                        this.state.navigation_target = NavigationTarget::Guild(Snowflake::new(5001));
                                        cx.notify();
                                    }))
                            )
                            // Add Server Button (+)
                            .child(
                                div()
                                    .w(px(48.0))
                                    .h(px(48.0))
                                    .bg(rgb(0x313338))
                                    .rounded(px(24.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(rgb(0x23a55a))
                                    .cursor_pointer()
                                    .child("+")
                            )
                    )
                    // 3B. Channels Sidebar (240px)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w(px(240.0))
                            .h_full()
                            .bg(rgb(0x2b2d31))
                            // Server Header (48px)
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .h(px(48.0))
                                    .w_full()
                                    .px(px(16.0))
                                    .items_center()
                                    .justify_between()
                                    .border_b_1()
                                    .border_color(rgb(0x1f2023))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .text_color(rgb(0xf2f3f5))
                                            .child("Rust Community")
                                    )
                                    .child("▼")
                            )
                            // Channel List
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .p(px(8.0))
                                    .gap(px(2.0))
                                    // Text Category
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .text_color(rgb(0x949ba4))
                                            .py(px(6.0))
                                            .child("▾ TEXT CHANNELS")
                                    )
                                    // # general
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .h(px(34.0))
                                            .px(px(8.0))
                                            .bg(rgb(0x3f4147))
                                            .rounded(px(4.0))
                                            .cursor_pointer()
                                            .child("# general")
                                            .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                this.state.active_channel_id = Some(Snowflake::new(7001));
                                                cx.notify();
                                            }))
                                    )
                                    // Voice Category
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .text_color(rgb(0x949ba4))
                                            .py(px(6.0))
                                            .child("▾ VOICE CHANNELS")
                                    )
                                    // 🔊 General Voice
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .h(px(34.0))
                                            .px(px(8.0))
                                            .rounded(px(4.0))
                                            .cursor_pointer()
                                            .child("🔊 General Voice")
                                            .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                this.state.active_voice_channel = Some(Snowflake::new(7002));
                                                cx.notify();
                                            }))
                                    )
                            )
                            // Voice Connected Status Pill (if in voice)
                            .when(is_voice_connected, |d| {
                                d.child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .h(px(52.0))
                                        .w_full()
                                        .bg(rgb(0x232428))
                                        .border_b_1()
                                        .border_color(rgb(0x1f2023))
                                        .px(px(16.0))
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(gpui::FontWeight::BOLD)
                                                        .text_color(rgb(0x23a55a))
                                                        .child("Voice Connected")
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x949ba4))
                                                        .child("RTC Connecting / 18ms")
                                                )
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(0xf23f43))
                                                .cursor_pointer()
                                                .child("Disconnect")
                                                .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                    this.state.active_voice_channel = None;
                                                    cx.notify();
                                                }))
                                        )
                                )
                            })
                            // User Account Footer Panel (52px)
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .h(px(52.0))
                                    .w_full()
                                    .bg(rgb(0x232428))
                                    .px(px(8.0))
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .w(px(32.0))
                                                    .h(px(32.0))
                                                    .bg(rgb(0xe05d44))
                                                    .rounded(px(16.0))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child("🦀")
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .font_weight(gpui::FontWeight::BOLD)
                                                            .child("Ferris")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x949ba4))
                                                            .child("@rustacean")
                                                    )
                                            )
                                    )
                                    // Control Buttons: Mic / Headphones / Settings
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .cursor_pointer()
                                                    .text_color(if is_muted { rgb(0xf23f43) } else { rgb(0xdbdee1) })
                                                    .child(if is_muted { "🎙✕" } else { "🎙" })
                                                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                        this.state.is_muted = !this.state.is_muted;
                                                        cx.notify();
                                                    }))
                                            )
                                            .child(
                                                div()
                                                    .cursor_pointer()
                                                    .text_color(if is_deafened { rgb(0xf23f43) } else { rgb(0xdbdee1) })
                                                    .child(if is_deafened { "🎧✕" } else { "🎧" })
                                                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                        this.state.is_deafened = !this.state.is_deafened;
                                                        cx.notify();
                                                    }))
                                            )
                                            .child(
                                                div()
                                                    .cursor_pointer()
                                                    .child("⚙")
                                                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                        this.state.is_settings_open = !this.state.is_settings_open;
                                                        cx.notify();
                                                    }))
                                            )
                                    )
                            )
                    )
                    // 3C. Main Chat View
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .h_full()
                            .bg(rgb(0x313338))
                            // Top Header (48px)
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .h(px(48.0))
                                    .w_full()
                                    .px(px(16.0))
                                    .items_center()
                                    .justify_between()
                                    .border_b_1()
                                    .border_color(rgb(0x1f2023))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_base()
                                                    .font_weight(gpui::FontWeight::BOLD)
                                                    .text_color(rgb(0xf2f3f5))
                                                    .child(format!("# {}", active_channel_name))
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0x949ba4))
                                                    .child("| General discussion about Discord Rust, GPUI and high performance UI")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .gap(px(16.0))
                                            .child("🔔")
                                            .child("📌")
                                            .child("👥")
                                            .child(
                                                div()
                                                    .px(px(8.0))
                                                    .py(px(4.0))
                                                    .bg(rgb(0x1e1f22))
                                                    .rounded(px(4.0))
                                                    .text_xs()
                                                    .child("Search")
                                            )
                                    )
                            )
                            // Message History Stream
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .p(px(16.0))
                                    .gap(px(16.0))
                                    // Cozy Message Group
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .gap(px(16.0))
                                            // Avatar
                                            .child(
                                                div()
                                                    .w(px(40.0))
                                                    .h(px(40.0))
                                                    .bg(rgb(0xe05d44))
                                                    .rounded(px(20.0))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child("🦀")
                                            )
                                            // Message Details
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    // Header line
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_row()
                                                            .items_center()
                                                            .gap(px(8.0))
                                                            .child(
                                                                div()
                                                                    .text_sm()
                                                                    .font_weight(gpui::FontWeight::BOLD)
                                                                    .text_color(rgb(0xe05d44))
                                                                    .child("Ferris")
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x949ba4))
                                                                    .child("Today at 11:45 PM")
                                                            )
                                                    )
                                                    // Content
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(rgb(0xdbdee1))
                                                            .child("Welcome to Discord Rust! Running natively at 120+ FPS with GPUI, DirectX 12, sub-100MB RAM, and DAVE E2EE.")
                                                    )
                                                    // Rich Embed
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_row()
                                                            .bg(rgb(0x2b2d31))
                                                            .rounded(px(4.0))
                                                            .border_l_4()
                                                            .border_color(rgb(0xe05d44))
                                                            .p(px(12.0))
                                                            .w(px(520.0))
                                                            .child(
                                                                div()
                                                                    .flex()
                                                                    .flex_col()
                                                                    .gap(px(6.0))
                                                                    .child(
                                                                        div()
                                                                            .text_sm()
                                                                            .font_weight(gpui::FontWeight::BOLD)
                                                                            .text_color(rgb(0xffffff))
                                                                            .child("Discord Rust Performance Architecture")
                                                                    )
                                                                    .child(
                                                                        div()
                                                                            .text_xs()
                                                                            .text_color(rgb(0xdbdee1))
                                                                            .child("Built clean-room in pure Rust with DAVE E2EE (MLS RFC 9420) and low-latency custom RTP voice streaming.")
                                                                    )
                                                            )
                                                    )
                                                    // Reactions
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_row()
                                                            .gap(px(8.0))
                                                            .child(
                                                                div()
                                                                    .flex()
                                                                    .flex_row()
                                                                    .items_center()
                                                                    .gap(px(4.0))
                                                                    .bg(rgb(0x2b2d31))
                                                                    .border_1()
                                                                    .border_color(rgb(0xe05d44))
                                                                    .rounded(px(8.0))
                                                                    .px(px(8.0))
                                                                    .py(px(4.0))
                                                                    .text_xs()
                                                                    .cursor_pointer()
                                                                    .child("🚀 12")
                                                            )
                                                            .child(
                                                                div()
                                                                    .flex()
                                                                    .flex_row()
                                                                    .items_center()
                                                                    .gap(px(4.0))
                                                                    .bg(rgb(0x2b2d31))
                                                                    .rounded(px(8.0))
                                                                    .px(px(8.0))
                                                                    .py(px(4.0))
                                                                    .text_xs()
                                                                    .cursor_pointer()
                                                                    .child("🦀 8")
                                                            )
                                                    )
                                            )
                                    )
                            )
                            // Message Composer Bar (Bottom)
                            .child(
                                div()
                                    .h(px(68.0))
                                    .w_full()
                                    .px(px(16.0))
                                    .pb(px(24.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .h(px(44.0))
                                            .w_full()
                                            .bg(rgb(0x383a40))
                                            .rounded(px(8.0))
                                            .items_center()
                                            .px(px(16.0))
                                            .justify_between()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .gap(px(12.0))
                                                    .child("+")
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(rgb(0x949ba4))
                                                            .child(format!("Message #{} ...", active_channel_name))
                                                    )
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .gap(px(12.0))
                                                    .child("🎁")
                                                    .child("GIF")
                                                    .child("😀")
                                            )
                                    )
                            )
                    )
                    // 3D. Server Members List (240px)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w(px(240.0))
                            .h_full()
                            .bg(rgb(0x2b2d31))
                            .p(px(16.0))
                            .gap(px(12.0))
                            // Category: ONLINE
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(rgb(0x949ba4))
                                    .child("ONLINE — 3")
                            )
                            // Member 1
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .w(px(32.0))
                                            .h(px(32.0))
                                            .bg(rgb(0xe05d44))
                                            .rounded(px(16.0))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .child("🦀")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(gpui::FontWeight::BOLD)
                                                    .text_color(rgb(0xe05d44))
                                                    .child("Ferris")
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0x949ba4))
                                                    .child("Playing with Rust")
                                            )
                                    )
                            )
                    )
            )
            // 4. Modal Overlay: Settings View (when toggled)
            .when(is_settings_open, |d| {
                d.child(
                    div()
                        .absolute()
                        .size_full()
                        .bg(rgb(0x111214))
                        .flex()
                        .flex_row()
                        // Settings Sidebar
                        .child(
                            div()
                                .w(px(218.0))
                                .h_full()
                                .bg(rgb(0x2b2d31))
                                .p(px(24.0))
                                .flex()
                                .flex_col()
                                .gap(px(12.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(gpui::FontWeight::BOLD)
                                        .text_color(rgb(0x949ba4))
                                        .child("USER SETTINGS")
                                )
                                .child("My Account")
                                .child("Profiles")
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(gpui::FontWeight::BOLD)
                                        .text_color(rgb(0x949ba4))
                                        .child("APP SETTINGS")
                                )
                                .child("Appearance")
                                .child("Voice & Video")
                                .child("Keybinds")
                        )
                        // Settings Content
                        .child(
                            div()
                                .flex_1()
                                .h_full()
                                .bg(rgb(0x313338))
                                .p(px(40.0))
                                .flex()
                                .flex_col()
                                .gap(px(20.0))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            div()
                                                .text_xl()
                                                .font_weight(gpui::FontWeight::BOLD)
                                                .child("Voice & Video Settings")
                                        )
                                        .child(
                                            div()
                                                .cursor_pointer()
                                                .text_lg()
                                                .child("✕ ESC")
                                                .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                    this.state.is_settings_open = false;
                                                    cx.notify();
                                                }))
                                        )
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(rgb(0x949ba4))
                                        .child("INPUT DEVICE: Default (Windows Core Audio WASAPI)")
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(rgb(0x949ba4))
                                        .child("OUTPUT DEVICE: Default (DirectSound / WASAPI)")
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(rgb(0x23a55a))
                                        .child("✓ Windows MMCSS (Pro Audio profile active)")
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(rgb(0xe05d44))
                                        .child("✓ DAVE E2EE (MLS RFC 9420 ratcheting active)")
                                )
                        )
                )
            })
    }
}

pub fn run_desktop_app() {
    tracing::info!("Initializing GPUI application window for Discord Rust...");

    Application::new().run(|cx: &mut App| {
        let bounds = Bounds {
            origin: Point {
                x: px(150.0),
                y: px(50.0),
            },
            size: Size {
                width: px(1280.0),
                height: px(800.0),
            },
        };

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: None, // Custom Frameless Window
            focus: true,
            show: true,
            kind: gpui::WindowKind::Normal,
            is_movable: true,
            display_id: None,
            window_background: gpui::WindowBackgroundAppearance::Opaque,
            window_min_size: Some(Size {
                width: px(940.0),
                height: px(500.0),
            }),
            ..Default::default()
        };

        cx.open_window(options, |_, cx| {
            cx.new(|_| DiscordRustAppView::new())
        })
        .expect("Failed to open Discord Rust native window");

        tracing::info!("Discord Rust window successfully opened at 1280x800 with GPUI.");
    });
}
