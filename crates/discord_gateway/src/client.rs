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

use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use futures::{SinkExt, StreamExt};
use crate::events::GatewayEvent;

/// Establish a live connection to Discord Gateway v10 and forward dispatch events
pub async fn run_gateway_loop(
    token: String,
    event_tx: tokio::sync::mpsc::UnboundedSender<GatewayEvent>,
) -> anyhow::Result<()> {
    let session = Arc::new(GatewaySession::new(token.clone()));
    let gateway_url = "wss://gateway.discord.gg/?v=10&encoding=json";

    tracing::info!("Connecting to Discord Gateway at {}...", gateway_url);

    let (ws_stream, _) = connect_async(gateway_url).await?;
    let (mut write_half, mut read_half) = ws_stream.split();

    // 1. Await Hello (Opcode 10)
    let first_msg = read_half.next().await
        .ok_or_else(|| anyhow::anyhow!("Gateway connection closed prematurely before Hello"))??;

    let hello_text = match first_msg {
        WsMessage::Text(t) => t,
        _ => return Err(anyhow::anyhow!("Expected text frame for Hello")),
    };

    let hello_val: serde_json::Value = serde_json::from_str(&hello_text)?;
    let heartbeat_interval = hello_val.get("d")
        .and_then(|d| d.get("heartbeat_interval"))
        .and_then(|h| h.as_u64())
        .unwrap_or(41250);

    tracing::info!("Gateway Hello received. Heartbeat interval: {}ms", heartbeat_interval);
    session.heartbeat_interval_ms.store(heartbeat_interval, Ordering::SeqCst);

    // 2. Send Identify (Opcode 2)
    // Intents: Guilds (1) | GuildMessages (512) | MessageContent (32768) | GuildVoiceStates (128) | GuildMessageReactions (1024)
    let intents = 1 | 512 | 32768 | 128 | 1024;
    let identify_payload = session.build_identify_payload(intents);
    write_half.send(WsMessage::Text(identify_payload.to_string().into())).await?;
    tracing::info!("Identify frame dispatched to Discord Gateway.");

    // Channel for outgoing WebSocket messages from heartbeat or voice
    let (out_tx, mut out_rx) = tokio::sync::mpsc::unbounded_channel::<WsMessage>();

    // 3. Heartbeat loop task
    let session_hb = session.clone();
    let out_tx_hb = out_tx.clone();
    let heartbeat_handle = tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(heartbeat_interval));
        loop {
            interval.tick().await;
            let hb = session_hb.build_heartbeat_payload();
            if out_tx_hb.send(WsMessage::Text(hb.to_string().into())).is_err() {
                break;
            }
            tracing::trace!("Sent Gateway heartbeat");
        }
    });

    // 4. Writer forwarding task
    let write_handle = tokio::spawn(async move {
        while let Some(msg) = out_rx.recv().await {
            if write_half.send(msg).await.is_err() {
                break;
            }
        }
    });

    // 5. Reader loop
    while let Some(msg_res) = read_half.next().await {
        match msg_res {
            Ok(WsMessage::Text(text)) => {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&text) {
                    let op = val.get("op").and_then(|o| o.as_u64()).unwrap_or(99);
                    if let Some(seq) = val.get("s").and_then(|s| s.as_u64()) {
                        session.update_sequence(seq);
                    }

                    match op {
                        0 => {
                            // Opcode 0 Dispatch
                            if let Ok(ev) = serde_json::from_value::<GatewayEvent>(val.clone()) {
                                let _ = event_tx.send(ev);
                            }
                        }
                        1 => {
                            // Server requested heartbeat
                            let hb = session.build_heartbeat_payload();
                            let _ = out_tx.send(WsMessage::Text(hb.to_string().into()));
                        }
                        7 => {
                            tracing::warn!("Gateway requested reconnect (Opcode 7)");
                            break;
                        }
                        9 => {
                            tracing::warn!("Gateway invalid session (Opcode 9)");
                            break;
                        }
                        11 => {
                            session.on_heartbeat_ack();
                        }
                        _ => {}
                    }
                }
            }
            Ok(WsMessage::Close(frame)) => {
                tracing::warn!("Gateway connection closed by server: {:?}", frame);
                break;
            }
            Err(e) => {
                tracing::error!("Gateway read error: {}", e);
                break;
            }
            _ => {}
        }
    }

    heartbeat_handle.abort();
    write_handle.abort();

    Ok(())
}
