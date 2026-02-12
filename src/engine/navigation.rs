// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Navigation management — URL sanitisation, scheme handling, search routing

use url::Url;
use tracing::info;

/// Supported internal URI schemes
const INTERNAL_SCHEME: &str = "marshall://";

/// Default search engine template (DuckDuckGo dark-mode)
const SEARCH_TEMPLATE: &str =
    "https://duckduckgo.com/?q={QUERY}&kae=d&k1=-1&kaj=m&kam=osm&kp=-2";

/// Navigation request types returned by the router
#[derive(Debug, Clone, PartialEq)]
pub enum NavTarget {
    /// An internal marshall:// page
    Internal(String),
    /// A regular HTTPS/HTTP URL
    Web(String),
    /// A search query to route through the default engine
    Search(String),
}

/// Route a raw URL-bar string to the appropriate target
pub fn resolve(input: &str) -> NavTarget {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed == "marshall:home" {
        return NavTarget::Internal("marshall://home".into());
    }
    if trimmed.starts_with(INTERNAL_SCHEME) {
        return NavTarget::Internal(trimmed.into());
    }
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        return NavTarget::Web(trimmed.into());
    }
    // Looks like a domain (contains dot, no spaces)
    if trimmed.contains('.') && !trimmed.contains(' ') {
        return NavTarget::Web(format!("https://{}", trimmed));
    }
    // Treat as a search query
    let encoded = urlencoding::encode(trimmed);
    NavTarget::Search(SEARCH_TEMPLATE.replace("{QUERY}", &encoded))
}

/// Sanitise a URL for display in the URL bar
pub fn display_url(uri: &str) -> String {
    if uri.starts_with(INTERNAL_SCHEME) || uri == "about:blank" || uri.is_empty() {
        return "marshall://home".into();
    }
    // Strip tracking params
    if let Ok(mut parsed) = Url::parse(uri) {
        let clean: Vec<(String, String)> = parsed
            .query_pairs()
            .filter(|(k, _)| {
                !matches!(
                    k.as_ref(),
                    "utm_source" | "utm_medium" | "utm_campaign" | "utm_content"
                    | "utm_term" | "fbclid" | "gclid" | "ref" | "ref_"
                )
            })
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        parsed.query_pairs_mut().clear().extend_pairs(clean);
        if parsed.query() == Some("") {
            parsed.set_query(None);
        }
        return parsed.to_string();
    }
    uri.to_string()
}

/// Check whether a URI is an internal marshall:// page
pub fn is_internal(uri: &str) -> bool {
    uri.starts_with(INTERNAL_SCHEME)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_empty_goes_home() {
        assert_eq!(resolve(""), NavTarget::Internal("marshall://home".into()));
    }

    #[test]
    fn resolve_domain_adds_https() {
        assert_eq!(resolve("example.com"), NavTarget::Web("https://example.com".into()));
    }

    #[test]
    fn resolve_search_encodes() {
        match resolve("hello world") {
            NavTarget::Search(url) => assert!(url.contains("hello%20world")),
            other => panic!("expected Search, got {:?}", other),
        }
    }
}
