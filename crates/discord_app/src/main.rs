//! Discord Native Rust Desktop Client
//! Architected with GPUI / DirectX 12 GPU Rendering Pipeline

pub mod theme;
pub mod state;
pub mod views;

use theme::DiscordTheme;
use state::AppState;
use views::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "discord_app=info,discord_core=info,discord_voice=info".into()),
        )
        .init();

    tracing::info!("Starting Discord Native Desktop Client (Rust + GPU Retained Rendering)");

    #[cfg(target_os = "windows")]
    {
        tracing::info!("Initializing Windows platform integrations (MMCSS Pro Audio, DPAPI)");
        if let Err(e) = discord_voice::mmcss::enable_pro_audio_thread_priority() {
            tracing::warn!("Could not enable MMCSS priority: {}", e);
        }
    }

    let theme = DiscordTheme::default();
    let state = AppState::default();

    tracing::info!(
        "Application Layout initialized: Rail={}px, Sidebar={}px, Header={}px",
        theme.server_rail_width,
        theme.sidebar_width,
        theme.header_height
    );

    // Initial view composition test
    ServerRailView::render(&state, &theme);
    ChannelListView::render(&state, &theme);
    ChatView::render(&state, &theme);
    MemberListView::render(&state, &theme);
    VoicePanelView::render(&state, &theme);

    tracing::info!("Discord Native Client core architecture successfully booted.");
    Ok(())
}
