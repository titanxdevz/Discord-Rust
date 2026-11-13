//! Discord Gateway Protocol and Real-Time Event Engine

use serde::{Deserialize, Serialize};

/// Discord Gateway Opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum GatewayOpcode {
    Dispatch,
    Heartbeat,
    Identify,
    PresenceUpdate,
    VoiceStateUpdate,
    Resume,
    Reconnect,
    RequestGuildMembers,
    InvalidSession,
    Hello,
    HeartbeatAck,
    Unknown(u8),
}

impl From<u8> for GatewayOpcode {
    fn from(val: u8) -> Self {
        match val {
            0 => GatewayOpcode::Dispatch,
            1 => GatewayOpcode::Heartbeat,
            2 => GatewayOpcode::Identify,
            3 => GatewayOpcode::PresenceUpdate,
            4 => GatewayOpcode::VoiceStateUpdate,
            6 => GatewayOpcode::Resume,
            7 => GatewayOpcode::Reconnect,
            8 => GatewayOpcode::RequestGuildMembers,
            9 => GatewayOpcode::InvalidSession,
            10 => GatewayOpcode::Hello,
            11 => GatewayOpcode::HeartbeatAck,
            other => GatewayOpcode::Unknown(other),
        }
    }
}

impl From<GatewayOpcode> for u8 {
    fn from(op: GatewayOpcode) -> Self {
        match op {
            GatewayOpcode::Dispatch => 0,
            GatewayOpcode::Heartbeat => 1,
            GatewayOpcode::Identify => 2,
            GatewayOpcode::PresenceUpdate => 3,
            GatewayOpcode::VoiceStateUpdate => 4,
            GatewayOpcode::Resume => 6,
            GatewayOpcode::Reconnect => 7,
            GatewayOpcode::RequestGuildMembers => 8,
            GatewayOpcode::InvalidSession => 9,
            GatewayOpcode::Hello => 10,
            GatewayOpcode::HeartbeatAck => 11,
            GatewayOpcode::Unknown(val) => val,
        }
    }
}

/// Generic Gateway Payload Frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayPayload<T = serde_json::Value> {
    pub op: GatewayOpcode,
    pub d: Option<T>,
    pub s: Option<u64>,
    pub t: Option<String>,
}

/// Gateway Hello Payload (Opcode 10)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayHello {
    pub heartbeat_interval: u64,
}
