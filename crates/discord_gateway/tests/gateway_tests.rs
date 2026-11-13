use discord_gateway::*;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

#[test]
fn test_gateway_session_payloads() {
    let session = GatewaySession::new("test_token_123");
    let identify = session.build_identify_payload(3276799);
    assert_eq!(identify["op"], 2);
    assert_eq!(identify["d"]["token"], "test_token_123");
    assert_eq!(identify["d"]["intents"], 3276799);

    let heartbeat = session.build_heartbeat_payload();
    assert_eq!(heartbeat["op"], 1);
    assert!(heartbeat["d"].is_null());

    session.update_sequence(42);
    let heartbeat_42 = session.build_heartbeat_payload();
    assert_eq!(heartbeat_42["d"], 42);

    assert!(session.check_heartbeat_ack());
    assert!(!session.check_heartbeat_ack());
    session.on_heartbeat_ack();
    assert!(session.check_heartbeat_ack());
}

#[test]
fn test_zlib_stream_decompression() {
    let mut decompressor = ZlibStreamDecompressor::new();

    let original_payload = r#"{"op":0,"t":"READY","d":{"v":10}}"#;
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(original_payload.as_bytes()).unwrap();
    let mut compressed = encoder.finish().unwrap();

    // Ensure Discord zlib suffix is appended
    if !compressed.ends_with(&ZLIB_SUFFIX) {
        compressed.extend_from_slice(&ZLIB_SUFFIX);
    }

    // Feed in two chunks
    let mid = compressed.len() / 2;
    let chunk1 = &compressed[..mid];
    let chunk2 = &compressed[mid..];

    let result1 = decompressor.feed(chunk1).unwrap();
    assert!(result1.is_none());

    let result2 = decompressor.feed(chunk2).unwrap();
    assert!(result2.is_some());
    assert_eq!(result2.unwrap(), original_payload);
}
