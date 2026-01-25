// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Ad blocking engine (stub when adblock feature disabled)

use std::collections::HashSet;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::info;

/// Ad blocker using custom rules
pub struct AdBlocker {
    enabled: bool,
    blocked_count: Arc<RwLock<u64>>,
    blocklist: HashSet<String>,
}

impl AdBlocker {
    pub fn new(enabled: bool) -> Self {
        let mut blocklist = HashSet::new();
        // Built-in blocklist
        blocklist.insert("doubleclick.net".to_string());
        blocklist.insert("googlesyndication.com".to_string());
        blocklist.insert("googleadservices.com".to_string());
        blocklist.insert("google-analytics.com".to_string());
        blocklist.insert("facebook.com/tr".to_string());
        blocklist.insert("connect.facebook.net".to_string());
        blocklist.insert("ads.twitter.com".to_string());
        blocklist.insert("analytics.twitter.com".to_string());
        blocklist.insert("scorecardresearch.com".to_string());
        blocklist.insert("quantserve.com".to_string());
        blocklist.insert("hotjar.com".to_string());
        
        Self {
            enabled,
            blocked_count: Arc::new(RwLock::new(0)),
            blocklist,
        }
    }

    /// Check if a request should be blocked
    pub fn should_block(&self, url: &str, _source_url: &str, _request_type: &str) -> bool {
        if !self.enabled {
            return false;
        }

        for domain in &self.blocklist {
            if url.contains(domain) {
                *self.blocked_count.write() += 1;
                return true;
            }
        }
        
        false
    }

    /// Get count of blocked requests
    pub fn blocked_count(&self) -> u64 {
        *self.blocked_count.read()
    }

    /// Enable or disable ad blocking
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    pub fn add_rule(&mut self, domain: String) {
        self.blocklist.insert(domain);
    }
}

impl Default for AdBlocker {
    fn default() -> Self {
        Self::new(true)
    }
}
