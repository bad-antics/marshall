// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! OSINT-Enhanced Search Engine
//! Custom branded search using Qwant with WHOIS, ports, vulns, and exploits

pub mod qwant;
pub mod whois;
pub mod portscan;
pub mod vulnerability;
pub mod osint;

use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

pub use qwant::*;
pub use whois::*;
pub use portscan::*;
pub use vulnerability::*;
pub use osint::*;

/// Search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub qwant_enabled: bool,
    pub whois_enabled: bool,
    pub portscan_enabled: bool,
    pub vuln_scan_enabled: bool,
    pub exploit_lookup_enabled: bool,
    pub safe_search: bool,
    pub region: String,
    pub max_results: usize,
    pub timeout_seconds: u64,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            qwant_enabled: true,
            whois_enabled: true,
            portscan_enabled: true,
            vuln_scan_enabled: true,
            exploit_lookup_enabled: true,
            safe_search: false,
            region: "en_US".to_string(),
            max_results: 50,
            timeout_seconds: 30,
        }
    }
}

/// Complete search result with all OSINT data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub query: String,
    pub timestamp: DateTime<Utc>,
    pub web_results: Vec<WebResult>,
    pub osint_data: Option<OSINTData>,
    pub total_results: usize,
    pub search_time_ms: u64,
}

/// Individual web search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebResult {
    pub title: String,
    pub url: String,
    pub domain: String,
    pub snippet: String,
    pub favicon_url: Option<String>,
    pub cached_url: Option<String>,
    pub osint: Option<DomainOSINT>,
}

/// OSINT data for a domain/target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainOSINT {
    pub domain: String,
    pub whois: Option<WhoisInfo>,
    pub open_ports: Vec<PortInfo>,
    pub vulnerabilities: Vec<VulnInfo>,
    pub exploits: Vec<ExploitInfo>,
    pub dns_records: Vec<DnsRecord>,
    pub ssl_info: Option<SSLInfo>,
    pub technologies: Vec<String>,
    pub risk_score: u8,
}

/// DNS record information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    pub record_type: String,
    pub value: String,
    pub ttl: Option<u32>,
}

/// SSL certificate information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSLInfo {
    pub issuer: String,
    pub subject: String,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub is_valid: bool,
    pub grade: String,
}

/// Main OSINT Search Engine
pub struct SearchEngine {
    pub config: SearchConfig,
    pub qwant: QwantClient,
    pub whois: WhoisLookup,
    pub portscan: PortScanner,
    pub vulns: VulnerabilityScanner,
    pub cache: Arc<RwLock<SearchCache>>,
}

impl SearchEngine {
    pub fn new(config: SearchConfig) -> Self {
        Self {
            qwant: QwantClient::new(&config.region),
            whois: WhoisLookup::new(),
            portscan: PortScanner::new(config.timeout_seconds),
            vulns: VulnerabilityScanner::new(),
            cache: Arc::new(RwLock::new(SearchCache::new())),
            config,
        }
    }

    /// Perform a full OSINT-enhanced search
    pub async fn search(&self, query: &str) -> Result<SearchResult, String> {
        let start = std::time::Instant::now();
        let id = uuid::Uuid::new_v4().to_string();

        // Check cache first
        if let Some(cached) = self.cache.read().get(query) {
            return Ok(cached.clone());
        }

        // Search via Qwant
        let mut web_results = if self.config.qwant_enabled {
            self.qwant.search(query, self.config.max_results).await?
        } else {
            Vec::new()
        };

        // Extract unique domains for OSINT
        let domains: Vec<String> = web_results
            .iter()
            .map(|r| r.domain.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .take(10) // Limit OSINT to first 10 unique domains
            .collect();

        // Enrich results with OSINT data
        for result in &mut web_results {
            if domains.contains(&result.domain) {
                let osint = self.gather_domain_osint(&result.domain).await;
                result.osint = Some(osint);
            }
        }

        // Determine if query is a domain/IP for full OSINT
        let osint_data = if self.is_target_query(query) {
            Some(OSINTData {
                target: query.to_string(),
                domain_osint: Some(self.gather_domain_osint(query).await),
                reconnaissance: self.full_recon(query).await,
            })
        } else {
            None
        };

        let elapsed = start.elapsed().as_millis() as u64;
        let total = web_results.len();

        let result = SearchResult {
            id,
            query: query.to_string(),
            timestamp: Utc::now(),
            web_results,
            osint_data,
            total_results: total,
            search_time_ms: elapsed,
        };

        // Cache the result
        self.cache.write().insert(query, result.clone());

        Ok(result)
    }

    /// Gather OSINT data for a specific domain
    async fn gather_domain_osint(&self, domain: &str) -> DomainOSINT {
        let mut osint = DomainOSINT {
            domain: domain.to_string(),
            whois: None,
            open_ports: Vec::new(),
            vulnerabilities: Vec::new(),
            exploits: Vec::new(),
            dns_records: Vec::new(),
            ssl_info: None,
            technologies: Vec::new(),
            risk_score: 0,
        };

        // WHOIS lookup
        if self.config.whois_enabled {
            osint.whois = self.whois.lookup(domain).await.ok();
        }

        // Port scan
        if self.config.portscan_enabled {
            osint.open_ports = self.portscan.scan(domain).await.unwrap_or_default();
        }

        // Vulnerability scan
        if self.config.vuln_scan_enabled {
            osint.vulnerabilities = self.vulns.scan(domain, &osint.open_ports).await;
        }

        // Exploit lookup
        if self.config.exploit_lookup_enabled {
            osint.exploits = self.vulns.find_exploits(&osint.vulnerabilities).await;
        }

        // Calculate risk score
        osint.risk_score = self.calculate_risk_score(&osint);

        osint
    }

    /// Full reconnaissance on a target
    async fn full_recon(&self, _target: &str) -> ReconData {
        ReconData::default()
    }

    /// Check if query looks like a domain or IP
    fn is_target_query(&self, query: &str) -> bool {
        // Check for domain pattern
        let domain_pattern = regex::Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z]{2,})+$").unwrap();
        // Check for IP pattern
        let ip_pattern = regex::Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap();
        
        domain_pattern.is_match(query) || ip_pattern.is_match(query)
    }

    /// Calculate risk score based on OSINT data
    fn calculate_risk_score(&self, osint: &DomainOSINT) -> u8 {
        let mut score: u8 = 0;

        // Open ports increase risk
        score = score.saturating_add((osint.open_ports.len() * 5).min(30) as u8);

        // Critical vulnerabilities
        for vuln in &osint.vulnerabilities {
            match vuln.severity.as_str() {
                "critical" => score = score.saturating_add(25),
                "high" => score = score.saturating_add(15),
                "medium" => score = score.saturating_add(8),
                "low" => score = score.saturating_add(3),
                _ => {}
            }
        }

        // Exploits available
        score = score.saturating_add((osint.exploits.len() * 10).min(30) as u8);

        score.min(100)
    }
}

/// Search cache
pub struct SearchCache {
    entries: std::collections::HashMap<String, SearchResult>,
    max_size: usize,
}

impl SearchCache {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            max_size: 100,
        }
    }

    pub fn get(&self, query: &str) -> Option<&SearchResult> {
        self.entries.get(query)
    }

    pub fn insert(&mut self, query: &str, result: SearchResult) {
        if self.entries.len() >= self.max_size {
            // Remove oldest entry
            if let Some(oldest) = self.entries.keys().next().cloned() {
                self.entries.remove(&oldest);
            }
        }
        self.entries.insert(query.to_string(), result);
    }
}

impl Default for SearchCache {
    fn default() -> Self {
        Self::new()
    }
}
