use crate::state::AppState;
use crate::theme::DiscordTheme;

/// Server Member List Component (240px collapsible)
pub struct MemberListView;

impl MemberListView {
    pub fn render(state: &AppState, theme: &DiscordTheme) {
        if !state.is_member_list_visible {
            return;
        }

        tracing::debug!(
            "Rendering MemberList: width={}px",
            theme.member_list_width
        );
        // Retained GPU element tree construction for Member List:
        // 1. Role category headers (e.g. "ONLINE — 12", "ADMIN — 2")
        // 2. Member items with 32x32 avatar, presence status indicator, activity badge
        // 3. Right-click context menu (Profile, Mention, Direct Message, Kick, Ban)
    }
}
