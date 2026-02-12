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
        // ── Google Ads & Analytics ──
        blocklist.insert("doubleclick.net".to_string());
        blocklist.insert("googlesyndication.com".to_string());
        blocklist.insert("googleadservices.com".to_string());
        blocklist.insert("google-analytics.com".to_string());
        blocklist.insert("googletagmanager.com".to_string());
        blocklist.insert("googletagservices.com".to_string());
        blocklist.insert("pagead2.googlesyndication.com".to_string());
        blocklist.insert("adservice.google.com".to_string());
        blocklist.insert("ssl.google-analytics.com".to_string());
        // ── Meta / Facebook ──
        blocklist.insert("facebook.com/tr".to_string());
        blocklist.insert("connect.facebook.net".to_string());
        blocklist.insert("pixel.facebook.com".to_string());
        blocklist.insert("an.facebook.com".to_string());
        // ── Twitter / X ──
        blocklist.insert("ads.twitter.com".to_string());
        blocklist.insert("analytics.twitter.com".to_string());
        blocklist.insert("ads-api.twitter.com".to_string());
        // ── Microsoft / LinkedIn ──
        blocklist.insert("bat.bing.com".to_string());
        blocklist.insert("ads.linkedin.com".to_string());
        blocklist.insert("px.ads.linkedin.com".to_string());
        blocklist.insert("analytics.pointdrive.linkedin.com".to_string());
        // ── Amazon Ads ──
        blocklist.insert("aax.amazon-adsystem.com".to_string());
        blocklist.insert("z-na.amazon-adsystem.com".to_string());
        blocklist.insert("fls-na.amazon.com".to_string());
        // ── Common Trackers ──
        blocklist.insert("scorecardresearch.com".to_string());
        blocklist.insert("quantserve.com".to_string());
        blocklist.insert("hotjar.com".to_string());
        blocklist.insert("fullstory.com".to_string());
        blocklist.insert("mouseflow.com".to_string());
        blocklist.insert("crazyegg.com".to_string());
        blocklist.insert("luckyorange.com".to_string());
        blocklist.insert("clarity.ms".to_string());
        blocklist.insert("newrelic.com".to_string());
        blocklist.insert("nr-data.net".to_string());
        blocklist.insert("mixpanel.com".to_string());
        blocklist.insert("segment.io".to_string());
        blocklist.insert("segment.com".to_string());
        blocklist.insert("amplitude.com".to_string());
        blocklist.insert("heapanalytics.com".to_string());
        blocklist.insert("kissmetrics.com".to_string());
        blocklist.insert("optimizely.com".to_string());
        // ── Ad Networks ──
        blocklist.insert("pubmatic.com".to_string());
        blocklist.insert("rubiconproject.com".to_string());
        blocklist.insert("openx.net".to_string());
        blocklist.insert("indexexchange.com".to_string());
        blocklist.insert("casalemedia.com".to_string());
        blocklist.insert("outbrain.com".to_string());
        blocklist.insert("taboola.com".to_string());
        blocklist.insert("criteo.com".to_string());
        blocklist.insert("criteo.net".to_string());
        blocklist.insert("adnxs.com".to_string());
        blocklist.insert("adsrvr.org".to_string());
        blocklist.insert("moatads.com".to_string());
        blocklist.insert("yieldmanager.com".to_string());
        blocklist.insert("turn.com".to_string());
        blocklist.insert("demdex.net".to_string());
        blocklist.insert("bluekai.com".to_string());
        blocklist.insert("krxd.net".to_string());
        blocklist.insert("exelator.com".to_string());
        blocklist.insert("adform.net".to_string());
        blocklist.insert("mediamath.com".to_string());
        blocklist.insert("bidswitch.net".to_string());
        blocklist.insert("mathtag.com".to_string());
        blocklist.insert("serving-sys.com".to_string());
        blocklist.insert("sizmek.com".to_string());
        // ── Social Widgets / Beacons ──
        blocklist.insert("platform.twitter.com/widgets".to_string());
        blocklist.insert("snap.licdn.com".to_string());
        blocklist.insert("static.ads-twitter.com".to_string());
        blocklist.insert("ct.pinterest.com".to_string());
        blocklist.insert("t.co/i/adsct".to_string());
        // ── Fingerprinting / Telemetry ──
        blocklist.insert("cdn.mxpnl.com".to_string());
        blocklist.insert("browser-intake-datadoghq.com".to_string());
        blocklist.insert("sentry.io".to_string());
        blocklist.insert("bugsnag.com".to_string());
        blocklist.insert("raygun.com".to_string());
        blocklist.insert("rollbar.com".to_string());
        
        info!("AdBlocker loaded {} rules", blocklist.len());
        
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
