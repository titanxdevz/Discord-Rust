//! Discord Voice Encryption Modes and Packet Layouts

use serde::{Deserialize, Serialize};

/// Supported Discord Voice Encryption Modes
///
/// Note: Legacy modes (xsalsa20_poly1305, xsalsa20_poly1305_suffix, plain aead_aes256_gcm)
/// have been discontinued by Discord. All current modes utilize the `_rtpsize` packet layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionMode {
    /// Preferred hardware-accelerated mode via AES-NI (12-byte nonce, 16-byte tag)
    #[serde(rename = "aead_aes256_gcm_rtpsize")]
    AeadAes256GcmRtpsize,

    /// Required constant-time fallback mode (24-byte extended nonce, 16-byte tag)
    #[serde(rename = "aead_xchacha20_poly1305_rtpsize")]
    AeadXChaCha20Poly1305Rtpsize,

    /// Legacy fallback mode maintained for compatibility
    #[serde(rename = "xsalsa20_poly1305_lite_rtpsize")]
    Xsalsa20Poly1305LiteRtpsize,
}

impl EncryptionMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            EncryptionMode::AeadAes256GcmRtpsize => "aead_aes256_gcm_rtpsize",
            EncryptionMode::AeadXChaCha20Poly1305Rtpsize => "aead_xchacha20_poly1305_rtpsize",
            EncryptionMode::Xsalsa20Poly1305LiteRtpsize => "xsalsa20_poly1305_lite_rtpsize",
        }
    }

    /// Return nonce size in bytes
    pub fn nonce_len(&self) -> usize {
        match self {
            EncryptionMode::AeadAes256GcmRtpsize => 12,
            EncryptionMode::AeadXChaCha20Poly1305Rtpsize => 24,
            EncryptionMode::Xsalsa20Poly1305LiteRtpsize => 4,
        }
    }

    /// Return authentication tag size in bytes
    pub fn tag_len(&self) -> usize {
        16
    }
}
