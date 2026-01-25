// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! SIP Protocol implementation stubs

use serde::{Deserialize, Serialize};

/// SIP message types
#[derive(Debug, Clone, PartialEq)]
pub enum SipMethod {
    Invite,
    Ack,
    Bye,
    Cancel,
    Register,
    Options,
    Info,
    Update,
    Refer,
    Message,
}

/// SIP response codes
#[derive(Debug, Clone, Copy)]
pub struct SipResponse {
    pub code: u16,
    pub reason: &'static str,
}

impl SipResponse {
    pub const TRYING: Self = Self { code: 100, reason: "Trying" };
    pub const RINGING: Self = Self { code: 180, reason: "Ringing" };
    pub const OK: Self = Self { code: 200, reason: "OK" };
    pub const BAD_REQUEST: Self = Self { code: 400, reason: "Bad Request" };
    pub const UNAUTHORIZED: Self = Self { code: 401, reason: "Unauthorized" };
    pub const NOT_FOUND: Self = Self { code: 404, reason: "Not Found" };
    pub const BUSY: Self = Self { code: 486, reason: "Busy Here" };
    pub const DECLINED: Self = Self { code: 603, reason: "Declined" };
}

/// SIP URI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SipUri {
    pub scheme: String,
    pub user: Option<String>,
    pub host: String,
    pub port: Option<u16>,
    pub parameters: Vec<(String, String)>,
}

impl SipUri {
    pub fn new(user: &str, host: &str) -> Self {
        Self {
            scheme: "sip".to_string(),
            user: Some(user.to_string()),
            host: host.to_string(),
            port: None,
            parameters: Vec::new(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut uri = format!("{}:", self.scheme);
        if let Some(ref user) = self.user {
            uri.push_str(user);
            uri.push('@');
        }
        uri.push_str(&self.host);
        if let Some(port) = self.port {
            uri.push_str(&format!(":{}", port));
        }
        uri
    }
}

/// SIP session description
#[derive(Debug, Clone)]
pub struct SdpSession {
    pub version: u32,
    pub origin: String,
    pub session_name: String,
    pub media: Vec<SdpMedia>,
}

#[derive(Debug, Clone)]
pub struct SdpMedia {
    pub media_type: MediaType,
    pub port: u16,
    pub protocol: String,
    pub formats: Vec<String>,
    pub attributes: Vec<(String, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MediaType {
    Audio,
    Video,
    Application,
}
