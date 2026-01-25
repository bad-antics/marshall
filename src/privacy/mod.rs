// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Privacy protection features

mod tracker_blocker;
mod fingerprint_protection;
mod cookie_manager;

pub use tracker_blocker::TrackerBlocker;
pub use fingerprint_protection::FingerprintProtection;
pub use cookie_manager::CookieManager;

use crate::config::PrivacyConfig;

/// Privacy engine that coordinates all privacy features
pub struct PrivacyEngine {
    pub tracker_blocker: TrackerBlocker,
    pub fingerprint_protection: FingerprintProtection,
    pub cookie_manager: CookieManager,
    config: PrivacyConfig,
}

impl PrivacyEngine {
    pub fn new(config: PrivacyConfig) -> Self {
        Self {
            tracker_blocker: TrackerBlocker::new(config.block_trackers),
            fingerprint_protection: FingerprintProtection::new(config.block_fingerprinting),
            cookie_manager: CookieManager::new(config.block_third_party_cookies),
            config,
        }
    }

    pub fn is_strict_mode(&self) -> bool {
        self.config.strict_mode
    }

    /// Check if a URL should be blocked
    pub fn should_block(&self, url: &str) -> bool {
        self.tracker_blocker.should_block(url)
    }
}
