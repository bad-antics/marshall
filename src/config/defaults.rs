// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Default configuration values

pub const DEFAULT_HOMEPAGE: &str = "about:blank";
pub const DEFAULT_SEARCH_ENGINE: &str = "https://duckduckgo.com/?q=";

pub const NULLSEC_USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:121.0) Gecko/20100101 Firefox/121.0",
];

pub const EASYLIST_URL: &str = "https://easylist.to/easylist/easylist.txt";
pub const EASYPRIVACY_URL: &str = "https://easylist.to/easylist/easyprivacy.txt";
pub const FANBOY_ANNOYANCE_URL: &str = "https://easylist.to/easylist/fanboy-annoyance.txt";
