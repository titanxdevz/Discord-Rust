use discord_voice::*;

#[test]
fn test_ip_discovery_packet() {
    let ssrc = 0x12345678;
    let req = IpDiscoveryPacket::new_request(ssrc);
    assert_eq!(req.len(), 74);
    assert_eq!(req[0..2], [0x00, 0x01]);
    assert_eq!(req[2..4], [0x00, 0x46]);
    assert_eq!(req[4..8], ssrc.to_be_bytes());

    // Create a mock response
    let mut resp = [0u8; 74];
    resp[0..2].copy_from_slice(&[0x00, 0x02]);
    resp[2..4].copy_from_slice(&[0x00, 0x46]);
    resp[4..8].copy_from_slice(&ssrc.to_be_bytes());
    let ip = b"198.51.100.42\0";
    resp[8..8 + ip.len()].copy_from_slice(ip);
    resp[72..74].copy_from_slice(&51234u16.to_le_bytes());

    let (parsed_ip, parsed_port) = IpDiscoveryPacket::parse_response(&resp).unwrap();
    assert_eq!(parsed_ip, "198.51.100.42");
    assert_eq!(parsed_port, 51234);
}

#[test]
fn test_rtp_header() {
    let header = RtpHeader::new(105, 9600, 0xCAFEBABE);
    let mut buf = [0u8; 12];
    header.write_to(&mut buf);
    assert_eq!(buf[0], 0x80);
    assert_eq!(buf[1], 120); // Opus payload type
    assert_eq!(&buf[2..4], &105u16.to_be_bytes());
    assert_eq!(&buf[4..8], &9600u32.to_be_bytes());
    assert_eq!(&buf[8..12], &0xCAFEBABEu32.to_be_bytes());
}

#[test]
fn test_aes256_gcm_rtpsize_encryption_roundtrip() {
    let mode = EncryptionMode::AeadAes256GcmRtpsize;
    let secret_key = [0x42u8; 32];
    let rtp_header = [0x80, 120, 0, 1, 0, 0, 3, 192, 0, 0, 0, 7];
    let opus_payload = b"Hello, Discord Opus 48kHz Voice Frame!";
    let nonce_counter = 12345u32;

    let encrypted = mode
        .encrypt_rtp_packet(&secret_key, &rtp_header, opus_payload, nonce_counter)
        .unwrap();

    assert!(encrypted.len() > 12 + opus_payload.len());

    let (decrypted, parsed_counter) = mode
        .decrypt_rtp_packet(&secret_key, &encrypted)
        .unwrap();

    assert_eq!(decrypted, opus_payload);
    assert_eq!(parsed_counter, nonce_counter);
}

#[test]
fn test_xchacha20_poly1305_rtpsize_encryption_roundtrip() {
    let mode = EncryptionMode::AeadXChaCha20Poly1305Rtpsize;
    let secret_key = [0x77u8; 32];
    let rtp_header = [0x80, 120, 0, 2, 0, 0, 7, 128, 0, 0, 0, 9];
    let opus_payload = b"XChaCha20Poly1305 fallback voice payload";
    let nonce_counter = 99999u32;

    let encrypted = mode
        .encrypt_rtp_packet(&secret_key, &rtp_header, opus_payload, nonce_counter)
        .unwrap();

    let (decrypted, parsed_counter) = mode
        .decrypt_rtp_packet(&secret_key, &encrypted)
        .unwrap();

    assert_eq!(decrypted, opus_payload);
    assert_eq!(parsed_counter, nonce_counter);
}

#[test]
fn test_dave_mls_epoch_transition() {
    let mut dave = DaveSessionState::new();
    assert_eq!(dave.current_epoch, 0);
    assert!(!dave.is_active);

    dave.on_prepare_transition(10);
    assert_eq!(dave.pending_epoch, Some(10));

    dave.on_execute_transition().unwrap();
    assert_eq!(dave.current_epoch, 10);
    assert!(dave.is_active);
    assert_eq!(dave.pending_epoch, None);
}

#[cfg(target_os = "windows")]
#[test]
fn test_windows_mmcss_priority() {
    let res = mmcss::enable_pro_audio_thread_priority();
    assert!(res.is_ok());
}
