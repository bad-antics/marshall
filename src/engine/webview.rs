// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! WebView management — configures WebKit privacy, security, and session isolation

use webkit2gtk::{
    WebView, WebViewExt, WebContext, WebContextExt,
    CookieAcceptPolicy, Settings, SettingsExt,
    TLSErrorsPolicy,
};
use webkit2gtk::CookieManagerExt;
use tracing::info;

use crate::config::Config;

/// Hardened user-agent strings (rotated per session)
const HARDENED_USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:125.0) Gecko/20100101 Firefox/125.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:125.0) Gecko/20100101 Firefox/125.0",
    "Mozilla/5.0 (X11; Linux x86_64; rv:125.0) Gecko/20100101 Firefox/125.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Safari/605.1.15",
];

/// WebView configuration manager
pub struct WebViewManager {
    privacy_strict: bool,
    block_fingerprinting: bool,
    block_third_party_cookies: bool,
    user_agent: String,
}

impl WebViewManager {
    /// Create a new WebView manager from app config
    pub fn from_config(config: &Config) -> Self {
        let mut rng = rand::thread_rng();
        let idx = rand::Rng::gen_range(&mut rng, 0..HARDENED_USER_AGENTS.len());

        Self {
            privacy_strict: config.privacy.strict_mode,
            block_fingerprinting: config.privacy.block_fingerprinting,
            block_third_party_cookies: config.privacy.block_third_party_cookies,
            user_agent: HARDENED_USER_AGENTS[idx].to_string(),
        }
    }

    /// Create an ephemeral (non-persistent) web context for maximum privacy
    pub fn create_ephemeral_context(&self) -> WebContext {
        let context = WebContext::default().unwrap();

        // Cookie policy
        if let Some(cookie_manager) = context.cookie_manager() {
            let policy = if self.block_third_party_cookies {
                CookieAcceptPolicy::NoThirdParty
            } else {
                CookieAcceptPolicy::Always
            };
            cookie_manager.set_accept_policy(policy);
        }

        // Reject invalid TLS by default
        context.set_tls_errors_policy(TLSErrorsPolicy::Fail);

        context
    }

    /// Apply hardened settings to a WebView
    pub fn harden_settings(&self, webview: &WebView) {
        let settings: Settings = WebViewExt::settings(webview).unwrap();

        // JavaScript — must stay on for internal pages & DDG
        settings.set_enable_javascript(true);

        // Privacy: disable fingerprinting surfaces when strict
        if self.block_fingerprinting {
            settings.set_enable_webgl(false);
            settings.set_enable_webaudio(false);
        }

        // Disable prefetching & cache to reduce tracking surface
        settings.set_enable_dns_prefetching(false);
        settings.set_enable_page_cache(false);

        // User agent rotation
        settings.set_user_agent(Some(&self.user_agent));

        // Security extras
        settings.set_allow_modal_dialogs(false);

        info!("WebView hardened — UA rotated, fingerprint surfaces disabled");
    }

    /// Get the current spoofed user agent
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }
}

impl Default for WebViewManager {
    fn default() -> Self {
        Self {
            privacy_strict: true,
            block_fingerprinting: true,
            block_third_party_cookies: true,
            user_agent: HARDENED_USER_AGENTS[0].to_string(),
        }
    }
}
