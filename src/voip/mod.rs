// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! VoIP Module - Voice over IP calling system
//! Supports SIP-based calls and contact management

pub mod sip;
pub mod contacts;
pub mod call_manager;
pub mod audio;

use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;

pub use sip::*;
pub use contacts::*;
pub use call_manager::*;

/// VoIP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoIPConfig {
    pub sip_server: String,
    pub sip_port: u16,
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub stun_server: Option<String>,
    pub auto_answer: bool,
    pub record_calls: bool,
    pub echo_cancellation: bool,
    pub noise_suppression: bool,
}

impl Default for VoIPConfig {
    fn default() -> Self {
        Self {
            sip_server: "sip.example.com".to_string(),
            sip_port: 5060,
            username: String::new(),
            password: String::new(),
            display_name: "Marshall User".to_string(),
            stun_server: Some("stun.l.google.com:19302".to_string()),
            auto_answer: false,
            record_calls: false,
            echo_cancellation: true,
            noise_suppression: true,
        }
    }
}

/// Call state
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CallState {
    Idle,
    Dialing,
    Ringing,
    Connected,
    OnHold,
    Ended,
    Failed,
}

/// Call direction
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CallDirection {
    Inbound,
    Outbound,
}

/// Active call information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Call {
    pub id: String,
    pub remote_number: String,
    pub remote_name: Option<String>,
    pub direction: CallDirection,
    pub state: CallState,
    pub started_at: DateTime<Utc>,
    pub connected_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub is_muted: bool,
    pub is_on_hold: bool,
    pub is_recording: bool,
}

impl Call {
    pub fn new(remote_number: &str, direction: CallDirection) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            remote_number: remote_number.to_string(),
            remote_name: None,
            direction,
            state: CallState::Idle,
            started_at: Utc::now(),
            connected_at: None,
            ended_at: None,
            is_muted: false,
            is_on_hold: false,
            is_recording: false,
        }
    }

    pub fn duration(&self) -> Option<Duration> {
        if let Some(connected) = self.connected_at {
            let end = self.ended_at.unwrap_or_else(Utc::now);
            Some(end - connected)
        } else {
            None
        }
    }

    pub fn duration_string(&self) -> String {
        if let Some(dur) = self.duration() {
            let secs = dur.num_seconds();
            let mins = secs / 60;
            let hours = mins / 60;
            if hours > 0 {
                format!("{:02}:{:02}:{:02}", hours, mins % 60, secs % 60)
            } else {
                format!("{:02}:{:02}", mins, secs % 60)
            }
        } else {
            "00:00".to_string()
        }
    }
}

/// Call history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallHistoryEntry {
    pub id: String,
    pub remote_number: String,
    pub remote_name: Option<String>,
    pub direction: CallDirection,
    pub started_at: DateTime<Utc>,
    pub duration_seconds: i64,
    pub outcome: CallOutcome,
    pub recording_path: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CallOutcome {
    Completed,
    Missed,
    Declined,
    Failed,
    Voicemail,
}

/// Main VoIP controller
pub struct VoIP {
    pub config: VoIPConfig,
    pub state: Arc<RwLock<VoIPState>>,
    pub active_call: Arc<RwLock<Option<Call>>>,
    pub contacts: ContactManager,
    pub call_history: Arc<RwLock<Vec<CallHistoryEntry>>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoIPState {
    Disconnected,
    Connecting,
    Registered,
    InCall,
    Error,
}

impl VoIP {
    pub fn new(config: VoIPConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(VoIPState::Disconnected)),
            active_call: Arc::new(RwLock::new(None)),
            contacts: ContactManager::new(),
            call_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn connect(&self) -> Result<(), String> {
        *self.state.write() = VoIPState::Connecting;
        // In real implementation, connect to SIP server
        tracing::info!("Connecting to SIP server {}:{}", self.config.sip_server, self.config.sip_port);
        *self.state.write() = VoIPState::Registered;
        Ok(())
    }

    pub fn disconnect(&self) {
        *self.state.write() = VoIPState::Disconnected;
        self.hangup();
    }

    pub fn call(&self, number: &str) -> Result<Call, String> {
        if *self.state.read() != VoIPState::Registered {
            return Err("Not registered with SIP server".to_string());
        }

        if self.active_call.read().is_some() {
            return Err("Already in a call".to_string());
        }

        let mut call = Call::new(number, CallDirection::Outbound);
        call.state = CallState::Dialing;
        
        // Look up contact name
        if let Some(contact) = self.contacts.find_by_number(number) {
            call.remote_name = Some(contact.name.clone());
        }

        *self.active_call.write() = Some(call.clone());
        *self.state.write() = VoIPState::InCall;
        
        tracing::info!("Calling {}", number);
        Ok(call)
    }

    pub fn answer(&self) -> Result<(), String> {
        let mut call_guard = self.active_call.write();
        if let Some(ref mut call) = *call_guard {
            if call.state == CallState::Ringing && call.direction == CallDirection::Inbound {
                call.state = CallState::Connected;
                call.connected_at = Some(Utc::now());
                tracing::info!("Call answered");
                return Ok(());
            }
        }
        Err("No incoming call to answer".to_string())
    }

    pub fn hangup(&self) {
        let mut call_guard = self.active_call.write();
        if let Some(ref mut call) = *call_guard {
            call.state = CallState::Ended;
            call.ended_at = Some(Utc::now());
            
            // Add to history
            let entry = CallHistoryEntry {
                id: call.id.clone(),
                remote_number: call.remote_number.clone(),
                remote_name: call.remote_name.clone(),
                direction: call.direction,
                started_at: call.started_at,
                duration_seconds: call.duration().map(|d| d.num_seconds()).unwrap_or(0),
                outcome: if call.connected_at.is_some() { CallOutcome::Completed } else { CallOutcome::Missed },
                recording_path: None,
                notes: None,
            };
            self.call_history.write().push(entry);
            
            tracing::info!("Call ended");
        }
        *call_guard = None;
        *self.state.write() = VoIPState::Registered;
    }

    pub fn mute(&self, muted: bool) {
        if let Some(ref mut call) = *self.active_call.write() {
            call.is_muted = muted;
        }
    }

    pub fn hold(&self, on_hold: bool) {
        if let Some(ref mut call) = *self.active_call.write() {
            call.is_on_hold = on_hold;
            call.state = if on_hold { CallState::OnHold } else { CallState::Connected };
        }
    }

    pub fn send_dtmf(&self, digit: char) {
        if self.active_call.read().is_some() {
            tracing::info!("Sending DTMF: {}", digit);
        }
    }

    pub fn state(&self) -> VoIPState {
        *self.state.read()
    }

    pub fn get_active_call(&self) -> Option<Call> {
        self.active_call.read().clone()
    }

    pub fn get_call_history(&self) -> Vec<CallHistoryEntry> {
        self.call_history.read().clone()
    }
}
