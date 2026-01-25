// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Cookie management

use std::collections::HashMap;

/// Cookie manager with privacy controls
pub struct CookieManager {
    block_third_party: bool,
    whitelist: Vec<String>,
}

impl CookieManager {
    pub fn new(block_third_party: bool) -> Self {
        Self {
            block_third_party,
            whitelist: Vec::new(),
        }
    }

    pub fn should_allow_cookie(&self, cookie_domain: &str, page_domain: &str) -> bool {
        // Always allow first-party cookies
        if cookie_domain == page_domain || cookie_domain.ends_with(&format!(".{}", page_domain)) {
            return true;
        }

        // Check whitelist
        if self.whitelist.iter().any(|w| cookie_domain.contains(w)) {
            return true;
        }

        // Block third-party if enabled
        !self.block_third_party
    }

    pub fn add_to_whitelist(&mut self, domain: String) {
        self.whitelist.push(domain);
    }

    pub fn clear_whitelist(&mut self) {
        self.whitelist.clear();
    }
}
