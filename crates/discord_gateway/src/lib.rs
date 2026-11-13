//! Discord Gateway Protocol and Real-Time Event Engine

use serde::{Deserialize, Serialize};
use discord_core::snowflake::Snowflake;

/// Main Discord Gateway Opcodes (WebSocket API)
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

/// Discord Voice Gateway Opcodes (WebSocket v8 Protocol)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum VoiceGatewayOpcode {
    Identify,
    SelectProtocol,
    Ready,
    Heartbeat,
    SessionDescription,
    Speaking,
    HeartbeatAck,
    Resume,
    Hello,
    Resumed,
    ClientDisconnect,
    DAVEPrepareTransition,
    DAVEExecuteTransition,
    DAVETransitionReady,
    DAVEPrepareEpoch,
    DAVEMlsKeyPackage,
    DAVEMlsCommit,
    DAVEMlsProposal,
    Unknown(u8),
}

impl From<u8> for VoiceGatewayOpcode {
    fn from(val: u8) -> Self {
        match val {
            0 => VoiceGatewayOpcode::Identify,
            1 => VoiceGatewayOpcode::SelectProtocol,
            2 => VoiceGatewayOpcode::Ready,
            3 => VoiceGatewayOpcode::Heartbeat,
            4 => VoiceGatewayOpcode::SessionDescription,
            5 => VoiceGatewayOpcode::Speaking,
            6 => VoiceGatewayOpcode::HeartbeatAck,
            7 => VoiceGatewayOpcode::Resume,
            8 => VoiceGatewayOpcode::Hello,
            9 => VoiceGatewayOpcode::Resumed,
            13 => VoiceGatewayOpcode::ClientDisconnect,
            21 => VoiceGatewayOpcode::DAVEPrepareTransition,
            22 => VoiceGatewayOpcode::DAVEExecuteTransition,
            23 => VoiceGatewayOpcode::DAVETransitionReady,
            24 => VoiceGatewayOpcode::DAVEPrepareEpoch,
            25 => VoiceGatewayOpcode::DAVEMlsKeyPackage,
            26 => VoiceGatewayOpcode::DAVEMlsCommit,
            27 => VoiceGatewayOpcode::DAVEMlsProposal,
            other => VoiceGatewayOpcode::Unknown(other),
        }
    }
}

impl From<VoiceGatewayOpcode> for u8 {
    fn from(op: VoiceGatewayOpcode) -> Self {
        match op {
            VoiceGatewayOpcode::Identify => 0,
            VoiceGatewayOpcode::SelectProtocol => 1,
            VoiceGatewayOpcode::Ready => 2,
            VoiceGatewayOpcode::Heartbeat => 3,
            VoiceGatewayOpcode::SessionDescription => 4,
            VoiceGatewayOpcode::Speaking => 5,
            VoiceGatewayOpcode::HeartbeatAck => 6,
            VoiceGatewayOpcode::Resume => 7,
            VoiceGatewayOpcode::Hello => 8,
            VoiceGatewayOpcode::Resumed => 9,
            VoiceGatewayOpcode::ClientDisconnect => 13,
            VoiceGatewayOpcode::DAVEPrepareTransition => 21,
            VoiceGatewayOpcode::DAVEExecuteTransition => 22,
            VoiceGatewayOpcode::DAVETransitionReady => 23,
            VoiceGatewayOpcode::DAVEPrepareEpoch => 24,
            VoiceGatewayOpcode::DAVEMlsKeyPackage => 25,
            VoiceGatewayOpcode::DAVEMlsCommit => 26,
            VoiceGatewayOpcode::DAVEMlsProposal => 27,
            VoiceGatewayOpcode::Unknown(val) => val,
        }
    }
}

/// Voice Gateway Opcode 0 Identify Payload
/// Must include `max_dave_protocol_version` for all modern calls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceIdentifyPayload {
    pub server_id: Snowflake,
    pub user_id: Snowflake,
    pub session_id: String,
    pub token: String,
    pub video: bool,
    pub streams: Vec<serde_json::Value>,
    pub max_dave_protocol_version: u16,
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
