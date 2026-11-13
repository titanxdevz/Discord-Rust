use crate::state::AppState;
use crate::theme::DiscordTheme;

/// Active Voice Channel & Video Grid Component
pub struct VoicePanelView;

impl VoicePanelView {
    pub fn render(state: &AppState, _theme: &DiscordTheme) {
        if state.active_voice_channel.is_none() {
            return;
        }

        tracing::debug!(
            "Rendering VoicePanel: channel={:?}, muted={}, deafened={}",
            state.active_voice_channel,
            state.is_muted,
            state.is_deafened
        );
        // Retained GPU element tree construction for Voice / Video Panel:
        // 1. Connection status pill (bottom-left): "Voice Connected" with RTC ping ms
        // 2. Active video / screen share grid with speaking 2px green outline
        // 3. Floating voice control dock: Camera toggle, Screen share, Mute, Deafen, Disconnect
    }
}
