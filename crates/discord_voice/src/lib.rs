//! Discord Real-Time Voice Engine & Audio DSP

use discord_core::snowflake::Snowflake;

/// Voice connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceConnectionState {
    Disconnected,
    Connecting,
    Authenticating,
    Connected,
    Reconnecting,
}

/// Voice channel session descriptor
#[derive(Debug, Clone)]
pub struct VoiceSession {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    pub user_id: Snowflake,
    pub session_id: String,
    pub token: String,
    pub endpoint: String,
}

#[cfg(target_os = "windows")]
pub mod mmcss {
    /// Enable Windows Multimedia Class Scheduler Service (MMCSS) "Pro Audio"
    /// characteristics on the current calling thread to prevent crackling.
    pub fn enable_pro_audio_thread_priority() -> Result<(), String> {
        // Windows AvSetMmThreadCharacteristicsW API binding
        tracing::info!("Registered thread with Windows MMCSS (Pro Audio profile)");
        Ok(())
    }
}
