use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

/// Discord Gateway Connection States
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Handshaking,
    Identifying,
    Ready,
    Resuming,
    Reconnecting,
}

/// Gateway Session state for heartbeating and session resumption
pub struct GatewaySession {
    pub token: String,
    pub session_id: Option<String>,
    pub resume_gateway_url: Option<String>,
    pub sequence: Arc<AtomicU64>,
    pub heartbeat_interval_ms: Arc<AtomicU64>,
    pub has_received_ack: Arc<AtomicBool>,
    pub status: ConnectionStatus,
}

impl GatewaySession {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            session_id: None,
            resume_gateway_url: None,
            sequence: Arc::new(AtomicU64::new(0)),
            heartbeat_interval_ms: Arc::new(AtomicU64::new(41250)),
            has_received_ack: Arc::new(AtomicBool::new(true)),
            status: ConnectionStatus::Disconnected,
        }
    }

    /// Construct Identify payload (Opcode 2)
    pub fn build_identify_payload(&self, intents: u64) -> serde_json::Value {
        serde_json::json!({
            "op": 2,
            "d": {
                "token": self.token,
                "intents": intents,
                "properties": {
                    "os": "Windows",
                    "browser": "Discord-Rust",
                    "device": "Discord-Rust-Desktop"
                },
                "compress": false
            }
        })
    }

    /// Construct Resume payload (Opcode 6)
    pub fn build_resume_payload(&self) -> Option<serde_json::Value> {
        let sid = self.session_id.as_ref()?;
        let seq = self.sequence.load(Ordering::SeqCst);
        Some(serde_json::json!({
            "op": 6,
            "d": {
                "token": self.token,
                "session_id": sid,
                "seq": seq
            }
        }))
    }

    /// Construct Heartbeat payload (Opcode 1)
    pub fn build_heartbeat_payload(&self) -> serde_json::Value {
        let seq = self.sequence.load(Ordering::SeqCst);
        let seq_val = if seq == 0 { serde_json::Value::Null } else { serde_json::json!(seq) };
        serde_json::json!({
            "op": 1,
            "d": seq_val
        })
    }

    /// Update sequence from incoming dispatch frame
    pub fn update_sequence(&self, seq: u64) {
        self.sequence.store(seq, Ordering::SeqCst);
    }

    /// Handle Heartbeat ACK (Opcode 11)
    pub fn on_heartbeat_ack(&self) {
        self.has_received_ack.store(true, Ordering::SeqCst);
        tracing::debug!("Heartbeat ACK received from Gateway");
    }

    /// Check if connection is dead (no ACK before next heartbeat)
    pub fn check_heartbeat_ack(&self) -> bool {
        self.has_received_ack.swap(false, Ordering::SeqCst)
    }
}
