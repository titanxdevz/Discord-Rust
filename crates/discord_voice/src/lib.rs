//! Discord Real-Time Voice Engine, Custom UDP/RTP Transport & DAVE E2EE

pub mod crypto;
pub mod transport;
pub mod dave;

use discord_core::snowflake::Snowflake;
pub use crypto::{CryptoError, EncryptionMode};
pub use transport::{IpDiscoveryPacket, RtpHeader};
pub use dave::{DaveOpcode, DaveSessionState, MAX_DAVE_PROTOCOL_VERSION};

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
    pub encryption_mode: EncryptionMode,
    pub dave_state: DaveSessionState,
}

#[cfg(target_os = "windows")]
pub mod mmcss {
    /// Enable Windows Multimedia Class Scheduler Service (MMCSS) "Pro Audio"
    /// characteristics on the current calling thread to prevent crackling.
    /// Matches the exact reverse-engineered behavior of Discord's discord_voice.node (avrt.dll).
    pub fn enable_pro_audio_thread_priority() -> Result<*mut std::ffi::c_void, String> {
        unsafe {
            use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
            let h_mod = LoadLibraryA(b"avrt.dll\0".as_ptr());
            if h_mod.is_null() {
                return Err("Failed to load avrt.dll".into());
            }
            let proc = GetProcAddress(h_mod, b"AvSetMmThreadCharacteristicsW\0".as_ptr());
            if proc.is_none() {
                return Err("Failed to find AvSetMmThreadCharacteristicsW in avrt.dll".into());
            }
            type FnAvSetMmThreadCharacteristicsW =
                unsafe extern "system" fn(*const u16, *mut u32) -> *mut std::ffi::c_void;
            let av_set: FnAvSetMmThreadCharacteristicsW = std::mem::transmute(proc);
            let mut task_index = 0u32;
            let task_name: Vec<u16> = "Pro Audio\0".encode_utf16().collect();
            let handle = av_set(task_name.as_ptr(), &mut task_index);
            if handle.is_null() {
                Err("AvSetMmThreadCharacteristicsW returned null handle".into())
            } else {
                tracing::info!(
                    "Successfully registered thread with Windows MMCSS (Pro Audio, task_index={})",
                    task_index
                );
                Ok(handle)
            }
        }
    }
}
