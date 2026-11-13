use crate::state::AppState;
use crate::theme::DiscordTheme;

/// Channel Sidebar Component (240px fixed width)
pub struct ChannelListView;

impl ChannelListView {
    pub fn render(state: &AppState, theme: &DiscordTheme) {
        tracing::debug!(
            "Rendering ChannelSidebar: width={}px, active_channel={:?}",
            theme.sidebar_width,
            state.active_channel_id
        );
        // Retained GPU element tree construction for Channel Sidebar:
        // 1. Guild header banner / Direct Messages header (48px high)
        // 2. Channel categories with collapse chevrons
        // 3. Channels rows (# text, 🔊 voice, 📢 announcement)
        // 4. User footer panel (52px high) with avatar, status badge, audio toggles
    }
}
