//! Marshall Threat Intelligence Module
//!
//! Real-time URL and domain threat analysis with local threat database.
//! Checks URLs against known malicious patterns, phishing indicators,
//! and suspicious domain characteristics.

use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

/// Threat classification levels
#[derive(Debug, Clone, PartialEq)]
pub enum ThreatLevel {
    Safe,
    Suspicious,
    Malicious,
    Phishing,
    Malware,
}

impl std::fmt::Display for ThreatLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreatLevel::Safe => write!(f, "SAFE"),
            ThreatLevel::Suspicious => write!(f, "SUSPICIOUS"),
            ThreatLevel::Malicious => write!(f, "MALICIOUS"),
            ThreatLevel::Phishing => write!(f, "PHISHING"),
            ThreatLevel::Malware => write!(f, "MALWARE"),
        }
    }
}

/// Result of a threat analysis
#[derive(Debug, Clone)]
pub struct ThreatAnalysis {
    pub url: String,
    pub domain: String,
    pub level: ThreatLevel,
    pub score: u32,       // 0-100, higher = more dangerous
    pub indicators: Vec<ThreatIndicator>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct ThreatIndicator {
    pub category: String,
    pub description: String,
    pub weight: u32,
}

/// Threat intelligence engine
pub struct ThreatIntel {
    /// Known malicious TLDs
    suspicious_tlds: HashSet<String>,
    /// Known phishing patterns
    phishing_patterns: Vec<String>,
    /// Homoglyph character mappings
    homoglyphs: Vec<(char, char)>,
    /// Trusted domains
    trusted_domains: HashSet<String>,
}

impl ThreatIntel {
    pub fn new() -> Self {
        let suspicious_tlds: HashSet<String> = [
            "tk", "ml", "ga", "cf", "gq",  // Free TLDs abused by phishing
            "xyz", "top", "club", "work", "buzz",
            "cam", "icu", "cyou", "rest",
        ].iter().map(|s| s.to_string()).collect();

        let phishing_patterns = vec![
            "login".to_string(),
            "signin".to_string(),
            "verify".to_string(),
            "secure".to_string(),
            "account".to_string(),
            "update".to_string(),
            "confirm".to_string(),
            "banking".to_string(),
            "paypal".to_string(),
            "apple-id".to_string(),
            "microsoft-login".to_string(),
            "amazon-security".to_string(),
        ];

        let homoglyphs = vec![
            ('o', '0'), ('l', '1'), ('i', '1'),
            ('a', 'а'), ('e', 'е'), ('o', 'о'),  // Cyrillic
            ('c', 'с'), ('p', 'р'), ('x', 'х'),
        ];

        let trusted_domains: HashSet<String> = [
            "google.com", "github.com", "microsoft.com", "apple.com",
            "amazon.com", "cloudflare.com", "mozilla.org", "wikipedia.org",
            "stackoverflow.com", "rust-lang.org", "python.org",
        ].iter().map(|s| s.to_string()).collect();

        Self {
            suspicious_tlds,
            phishing_patterns,
            homoglyphs,
            trusted_domains,
        }
    }

    /// Analyze a URL for threats
    pub fn analyze_url(&self, url: &str) -> ThreatAnalysis {
        let domain = self.extract_domain(url);
        let mut indicators = Vec::new();
        let mut score: u32 = 0;

        // Check suspicious TLD
        if let Some(tld) = domain.split('.').last() {
            if self.suspicious_tlds.contains(tld) {
                indicators.push(ThreatIndicator {
                    category: "TLD".to_string(),
                    description: format!("Suspicious TLD: .{}", tld),
                    weight: 20,
                });
                score += 20;
            }
        }

        // Check for phishing patterns in domain
        let domain_lower = domain.to_lowercase();
        for pattern in &self.phishing_patterns {
            if domain_lower.contains(pattern) && !self.trusted_domains.contains(&domain) {
                indicators.push(ThreatIndicator {
                    category: "PHISHING".to_string(),
                    description: format!("Phishing keyword '{}' in domain", pattern),
                    weight: 25,
                });
                score += 25;
            }
        }

        // Check for homoglyph attacks
        for &(real, fake) in &self.homoglyphs {
            if domain.contains(fake) {
                indicators.push(ThreatIndicator {
                    category: "HOMOGLYPH".to_string(),
                    description: format!("Possible homoglyph: '{}' may impersonate '{}'", fake, real),
                    weight: 30,
                });
                score += 30;
            }
        }

        // Check for IP address instead of domain
        if domain.chars().all(|c| c.is_ascii_digit() || c == '.') {
            indicators.push(ThreatIndicator {
                category: "IP_URL".to_string(),
                description: "URL uses IP address instead of domain name".to_string(),
                weight: 15,
            });
            score += 15;
        }

        // Check for excessive subdomains (common in phishing)
        let subdomain_count = domain.matches('.').count();
        if subdomain_count > 3 {
            indicators.push(ThreatIndicator {
                category: "SUBDOMAIN".to_string(),
                description: format!("Excessive subdomains: {} levels", subdomain_count + 1),
                weight: 15,
            });
            score += 15;
        }

        // Check for suspicious URL patterns
        if url.contains("@") {
            indicators.push(ThreatIndicator {
                category: "URL_OBFUSCATION".to_string(),
                description: "URL contains @ symbol — possible credential harvesting redirect".to_string(),
                weight: 35,
            });
            score += 35;
        }

        // Check for data URIs
        if url.starts_with("data:") {
            indicators.push(ThreatIndicator {
                category: "DATA_URI".to_string(),
                description: "Data URI scheme — content embedded in URL".to_string(),
                weight: 25,
            });
            score += 25;
        }

        // Check for HTTP (not HTTPS)
        if url.starts_with("http://") && !domain_lower.starts_with("localhost") {
            indicators.push(ThreatIndicator {
                category: "NO_TLS".to_string(),
                description: "Connection not encrypted (HTTP)".to_string(),
                weight: 10,
            });
            score += 10;
        }

        // Check for very long URLs (common in phishing)
        if url.len() > 200 {
            indicators.push(ThreatIndicator {
                category: "LONG_URL".to_string(),
                description: format!("Unusually long URL: {} characters", url.len()),
                weight: 10,
            });
            score += 10;
        }

        // Check for URL shortener domains
        let shorteners = ["bit.ly", "t.co", "goo.gl", "tinyurl.com", "is.gd", "ow.ly"];
        if shorteners.iter().any(|s| domain_lower.contains(s)) {
            indicators.push(ThreatIndicator {
                category: "SHORTENER".to_string(),
                description: "URL shortener detected — destination unknown".to_string(),
                weight: 15,
            });
            score += 15;
        }

        // Cap score at 100
        score = score.min(100);

        // Determine threat level
        let level = if self.trusted_domains.contains(&domain) {
            ThreatLevel::Safe
        } else if score >= 70 {
            if indicators.iter().any(|i| i.category == "PHISHING" || i.category == "HOMOGLYPH") {
                ThreatLevel::Phishing
            } else {
                ThreatLevel::Malicious
            }
        } else if score >= 40 {
            ThreatLevel::Suspicious
        } else {
            ThreatLevel::Safe
        };

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        ThreatAnalysis {
            url: url.to_string(),
            domain,
            level,
            score,
            indicators,
            timestamp,
        }
    }

    fn extract_domain(&self, url: &str) -> String {
        let url = url.trim();
        let without_scheme = if let Some(pos) = url.find("://") {
            &url[pos + 3..]
        } else {
            url
        };
        let without_path = if let Some(pos) = without_scheme.find('/') {
            &without_scheme[..pos]
        } else {
            without_scheme
        };
        let without_port = if let Some(pos) = without_path.rfind(':') {
            &without_path[..pos]
        } else {
            without_path
        };
        without_port.to_string()
    }

    /// Check if a domain is trusted
    pub fn is_trusted(&self, domain: &str) -> bool {
        self.trusted_domains.contains(domain)
    }

    /// Add a domain to the trusted list
    pub fn trust_domain(&mut self, domain: &str) {
        self.trusted_domains.insert(domain.to_string());
    }

    /// Block a domain
    pub fn block_domain(&mut self, domain: &str) {
        self.trusted_domains.remove(domain);
    }
}

impl Default for ThreatIntel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_url() {
        let intel = ThreatIntel::new();
        let result = intel.analyze_url("https://github.com/bad-antics");
        assert_eq!(result.level, ThreatLevel::Safe);
        assert_eq!(result.score, 0);
    }

    #[test]
    fn test_phishing_url() {
        let intel = ThreatIntel::new();
        let result = intel.analyze_url("http://paypal-login-verify.tk/account");
        assert!(result.score >= 40);
    }

    #[test]
    fn test_ip_url() {
        let intel = ThreatIntel::new();
        let result = intel.analyze_url("http://192.168.1.1/admin");
        assert!(result.indicators.iter().any(|i| i.category == "IP_URL"));
    }

    #[test]
    fn test_extract_domain() {
        let intel = ThreatIntel::new();
        assert_eq!(intel.extract_domain("https://example.com/path"), "example.com");
        assert_eq!(intel.extract_domain("http://test.org:8080/"), "test.org");
    }
}
