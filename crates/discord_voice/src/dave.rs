//! Discord DAVE (Audio & Video End-to-End Encryption) Protocol Engine
//!
//! Mandatory since 1 March 2026.
//! DAVE utilizes Messaging Layer Security (MLS, RFC 9420) for ratcheted group key exchange,
//! epoch transitions, and per-sender frame key derivation.
//! Reference: Discord DAVE Protocol Whitepaper.

use serde::{Deserialize, Serialize};

/// Maximum supported DAVE protocol version to advertise in Opcode 0 Identify
pub const MAX_DAVE_PROTOCOL_VERSION: u16 = 1;

/// DAVE Voice Gateway Opcodes (JSON and Binary Frames)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum DaveOpcode {
    /// Server signals an upcoming MLS epoch transition
    PrepareTransition,
    /// Server commands immediate switch to target epoch
    ExecuteTransition,
    /// Client acknowledges readiness for transition
    TransitionReady,
    /// Server prepares epoch key schedule
    PrepareEpoch,
    /// Binary frame: MLS KeyPackage upload or distribution
    MlsKeyPackage,
    /// Binary frame: MLS Commit message carrying group ratchets
    MlsCommit,
    /// Binary frame: MLS Proposal message
    MlsProposal,
    Unknown(u8),
}

impl From<u8> for DaveOpcode {
    fn from(val: u8) -> Self {
        match val {
            12 => DaveOpcode::PrepareTransition,
            13 => DaveOpcode::ExecuteTransition,
            14 => DaveOpcode::TransitionReady,
            15 => DaveOpcode::PrepareEpoch,
            21 => DaveOpcode::MlsKeyPackage,
            22 => DaveOpcode::MlsCommit,
            23 => DaveOpcode::MlsProposal,
            other => DaveOpcode::Unknown(other),
        }
    }
}

impl From<DaveOpcode> for u8 {
    fn from(op: DaveOpcode) -> Self {
        match op {
            DaveOpcode::PrepareTransition => 12,
            DaveOpcode::ExecuteTransition => 13,
            DaveOpcode::TransitionReady => 14,
            DaveOpcode::PrepareEpoch => 15,
            DaveOpcode::MlsKeyPackage => 21,
            DaveOpcode::MlsCommit => 22,
            DaveOpcode::MlsProposal => 23,
            DaveOpcode::Unknown(val) => val,
        }
    }
}

/// DAVE E2EE Session State Machine
#[derive(Debug, Clone)]
pub struct DaveSessionState {
    pub protocol_version: u16,
    pub current_epoch: u64,
    pub pending_epoch: Option<u64>,
    pub is_active: bool,
}

impl DaveSessionState {
    pub fn new() -> Self {
        Self {
            protocol_version: MAX_DAVE_PROTOCOL_VERSION,
            current_epoch: 0,
            pending_epoch: None,
            is_active: false,
        }
    }

    /// Process a PrepareTransition event from Discord Voice Gateway
    pub fn on_prepare_transition(&mut self, target_epoch: u64) {
        tracing::info!("DAVE: Preparing transition to target MLS epoch {}", target_epoch);
        self.pending_epoch = Some(target_epoch);
    }

    /// Execute the transition to the target epoch
    pub fn on_execute_transition(&mut self) -> Result<(), &'static str> {
        if let Some(target) = self.pending_epoch.take() {
            tracing::info!("DAVE: Successfully ratcheted to MLS epoch {}", target);
            self.current_epoch = target;
            self.is_active = true;
            Ok(())
        } else {
            Err("No pending epoch transition to execute")
        }
    }
}

impl Default for DaveSessionState {
    fn default() -> Self {
        Self::new()
    }
}
