use crate::state::AppState;
use crate::theme::DiscordTheme;

/// Server Rail Component (72px fixed width)
pub struct ServerRailView;

impl ServerRailView {
    pub fn render(state: &AppState, theme: &DiscordTheme) {
        tracing::debug!(
            "Rendering ServerRail: width={}px, guilds_count={}",
            theme.server_rail_width,
            state.guilds.len()
        );
        // Retained GPU element tree construction for Server Rail:
        // 1. Home Direct Messages button (48x48 rounded rect / circle)
        // 2. 32x2px separator line
        // 3. Guild icons list with unread pills and selection pills
        // 4. Explore & Add Server action buttons
    }
}
