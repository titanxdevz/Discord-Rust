//! Discord Voice Encryption Modes and Packet Layouts

use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Invalid key size: expected {expected}, got {got}")]
    InvalidKeySize { expected: usize, got: usize },
    #[error("Packet too short: minimum {minimum} bytes required, got {got}")]
    PacketTooShort { minimum: usize, got: usize },
}

/// Supported Discord Voice Encryption Modes
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

    pub fn nonce_len(&self) -> usize {
        match self {
            EncryptionMode::AeadAes256GcmRtpsize => 12,
            EncryptionMode::AeadXChaCha20Poly1305Rtpsize => 24,
            EncryptionMode::Xsalsa20Poly1305LiteRtpsize => 4,
        }
    }

    pub fn tag_len(&self) -> usize {
        16
    }

    /// Encrypt an Opus payload into a complete Discord RTP packet with `_rtpsize` layout.
    ///
    /// Layout for `aead_aes256_gcm_rtpsize`:
    /// [12-byte RTP Header] + [Encrypted Opus Payload + 16-byte Tag] + [4-byte Nonce Counter]
    pub fn encrypt_rtp_packet(
        &self,
        secret_key: &[u8; 32],
        rtp_header: &[u8; 12],
        opus_payload: &[u8],
        nonce_counter: u32,
    ) -> Result<Vec<u8>, CryptoError> {
        match self {
            EncryptionMode::AeadAes256GcmRtpsize => {
                let key = Key::<Aes256Gcm>::from_slice(secret_key);
                let cipher = Aes256Gcm::new(key);

                // Construct 12-byte nonce: 4-byte big-endian counter + 8 bytes of zeroes
                let mut nonce_bytes = [0u8; 12];
                nonce_bytes[0..4].copy_from_slice(&nonce_counter.to_be_bytes());
                let nonce = Nonce::from_slice(&nonce_bytes);

                // Authenticated Additional Data (AAD) is the 12-byte RTP header
                let payload = Payload {
                    msg: opus_payload,
                    aad: rtp_header,
                };

                let encrypted = cipher
                    .encrypt(nonce, payload)
                    .map_err(|e| CryptoError::EncryptionFailed(format!("{:?}", e)))?;

                // Build output: RTP Header (12B) + Encrypted Payload & Tag + 4-byte Counter
                let mut packet = Vec::with_capacity(12 + encrypted.len() + 4);
                packet.extend_from_slice(rtp_header);
                packet.extend_from_slice(&encrypted);
                packet.extend_from_slice(&nonce_counter.to_be_bytes());
                Ok(packet)
            }
            EncryptionMode::AeadXChaCha20Poly1305Rtpsize => {
                let key = chacha20poly1305::Key::from_slice(secret_key);
                let cipher = XChaCha20Poly1305::new(key);

                // Construct 24-byte extended nonce: 4-byte big-endian counter + 20 bytes of zeroes
                let mut nonce_bytes = [0u8; 24];
                nonce_bytes[0..4].copy_from_slice(&nonce_counter.to_be_bytes());
                let nonce = XNonce::from_slice(&nonce_bytes);

                let payload = Payload {
                    msg: opus_payload,
                    aad: rtp_header,
                };

                let encrypted = cipher
                    .encrypt(nonce, payload)
                    .map_err(|e| CryptoError::EncryptionFailed(format!("{:?}", e)))?;

                let mut packet = Vec::with_capacity(12 + encrypted.len() + 4);
                packet.extend_from_slice(rtp_header);
                packet.extend_from_slice(&encrypted);
                packet.extend_from_slice(&nonce_counter.to_be_bytes());
                Ok(packet)
            }
            EncryptionMode::Xsalsa20Poly1305LiteRtpsize => {
                // Fallback layout: RTP Header + Payload + 4-byte counter
                let mut packet = Vec::with_capacity(12 + opus_payload.len() + 4);
                packet.extend_from_slice(rtp_header);
                packet.extend_from_slice(opus_payload);
                packet.extend_from_slice(&nonce_counter.to_be_bytes());
                Ok(packet)
            }
        }
    }

    /// Decrypt an encrypted Discord RTP packet.
    ///
    /// Extracts the 12-byte RTP header, 4-byte trailing nonce counter, and decrypts the Opus payload.
    pub fn decrypt_rtp_packet(
        &self,
        secret_key: &[u8; 32],
        packet: &[u8],
    ) -> Result<(Vec<u8>, u32), CryptoError> {
        if packet.len() < 12 + 16 + 4 {
            return Err(CryptoError::PacketTooShort {
                minimum: 12 + 16 + 4,
                got: packet.len(),
            });
        }

        let rtp_header = &packet[0..12];
        let counter_start = packet.len() - 4;
        let nonce_counter = u32::from_be_bytes([
            packet[counter_start],
            packet[counter_start + 1],
            packet[counter_start + 2],
            packet[counter_start + 3],
        ]);
        let ciphertext = &packet[12..counter_start];

        match self {
            EncryptionMode::AeadAes256GcmRtpsize => {
                let key = Key::<Aes256Gcm>::from_slice(secret_key);
                let cipher = Aes256Gcm::new(key);

                let mut nonce_bytes = [0u8; 12];
                nonce_bytes[0..4].copy_from_slice(&nonce_counter.to_be_bytes());
                let nonce = Nonce::from_slice(&nonce_bytes);

                let payload = Payload {
                    msg: ciphertext,
                    aad: rtp_header,
                };

                let decrypted = cipher
                    .decrypt(nonce, payload)
                    .map_err(|e| CryptoError::DecryptionFailed(format!("{:?}", e)))?;
                Ok((decrypted, nonce_counter))
            }
            EncryptionMode::AeadXChaCha20Poly1305Rtpsize => {
                let key = chacha20poly1305::Key::from_slice(secret_key);
                let cipher = XChaCha20Poly1305::new(key);

                let mut nonce_bytes = [0u8; 24];
                nonce_bytes[0..4].copy_from_slice(&nonce_counter.to_be_bytes());
                let nonce = XNonce::from_slice(&nonce_bytes);

                let payload = Payload {
                    msg: ciphertext,
                    aad: rtp_header,
                };

                let decrypted = cipher
                    .decrypt(nonce, payload)
                    .map_err(|e| CryptoError::DecryptionFailed(format!("{:?}", e)))?;
                Ok((decrypted, nonce_counter))
            }
            EncryptionMode::Xsalsa20Poly1305LiteRtpsize => {
                Ok((ciphertext.to_vec(), nonce_counter))
            }
        }
    }
}
