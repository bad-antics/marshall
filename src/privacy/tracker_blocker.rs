// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Tracker blocking

use std::collections::HashSet;
use once_cell::sync::Lazy;

/// Known tracking domains
static TRACKING_DOMAINS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    // Major trackers
    set.insert("google-analytics.com");
    set.insert("googletagmanager.com");
    set.insert("doubleclick.net");
    set.insert("facebook.com/tr");
    set.insert("connect.facebook.net");
    set.insert("pixel.facebook.com");
    set.insert("analytics.twitter.com");
    set.insert("ads.twitter.com");
    set.insert("scorecardresearch.com");
    set.insert("quantserve.com");
    set.insert("hotjar.com");
    set.insert("fullstory.com");
    set.insert("mixpanel.com");
    set.insert("segment.io");
    set.insert("amplitude.com");
    set.insert("newrelic.com");
    set.insert("bugsnag.com");
    set.insert("sentry.io");
    set.insert("crashlytics.com");
    set
});

/// Tracker blocker
pub struct TrackerBlocker {
    enabled: bool,
    blocked_count: u64,
    custom_blocklist: HashSet<String>,
}

impl TrackerBlocker {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            blocked_count: 0,
            custom_blocklist: HashSet::new(),
        }
    }

    pub fn should_block(&self, url: &str) -> bool {
        if !self.enabled {
            return false;
        }

        // Check against known trackers
        for domain in TRACKING_DOMAINS.iter() {
            if url.contains(domain) {
                return true;
            }
        }

        // Check custom blocklist
        for pattern in &self.custom_blocklist {
            if url.contains(pattern) {
                return true;
            }
        }

        false
    }

    pub fn add_to_blocklist(&mut self, pattern: String) {
        self.custom_blocklist.insert(pattern);
    }

    pub fn increment_blocked(&mut self) {
        self.blocked_count += 1;
    }

    pub fn blocked_count(&self) -> u64 {
        self.blocked_count
    }
}
