// Copyright (c) 2026 bad-antics
// OSINT Search - Comprehensive OSINT Data Structures

use serde::{Deserialize, Serialize};
use super::{DomainOSINT, VulnInfo, ExploitInfo, PortInfo};

/// Full OSINT data for a target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSINTData {
    pub target: String,
    pub domain_osint: Option<DomainOSINT>,
    pub reconnaissance: ReconData,
}

/// Reconnaissance data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReconData {
    pub subdomains: Vec<SubdomainInfo>,
    pub email_addresses: Vec<String>,
    pub related_domains: Vec<String>,
    pub ip_ranges: Vec<String>,
    pub asn_info: Option<ASNInfo>,
    pub geo_location: Option<GeoLocation>,
    pub social_links: Vec<SocialLink>,
    pub metadata: Vec<MetadataEntry>,
}

/// Subdomain information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubdomainInfo {
    pub subdomain: String,
    pub ip_addresses: Vec<String>,
    pub status: SubdomainStatus,
    pub technologies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubdomainStatus {
    Active,
    Inactive,
    Redirect,
    Unknown,
}

/// ASN (Autonomous System Number) information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASNInfo {
    pub asn: String,
    pub name: String,
    pub description: String,
    pub country: String,
    pub ip_ranges: Vec<String>,
}

/// Geographic location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub country: String,
    pub country_code: String,
    pub region: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub isp: String,
}

/// Social media link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLink {
    pub platform: String,
    pub url: String,
    pub username: Option<String>,
}

/// Metadata entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataEntry {
    pub key: String,
    pub value: String,
    pub source: String,
}

/// OSINT report generator
pub struct OSINTReport {
    pub target: String,
    pub data: OSINTData,
}

impl OSINTReport {
    pub fn new(target: &str, data: OSINTData) -> Self {
        Self {
            target: target.to_string(),
            data,
        }
    }

    /// Generate markdown report
    pub fn to_markdown(&self) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("# OSINT Report: {}\n\n", self.target));
        report.push_str(&format!("Generated: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        // Domain OSINT section
        if let Some(domain) = &self.data.domain_osint {
            report.push_str("## Domain Information\n\n");
            
            // WHOIS
            if let Some(whois) = &domain.whois {
                report.push_str("### WHOIS Data\n\n");
                if let Some(registrar) = &whois.registrar {
                    report.push_str(&format!("- **Registrar:** {}\n", registrar));
                }
                if let Some(created) = &whois.creation_date {
                    report.push_str(&format!("- **Created:** {}\n", created));
                }
                if let Some(expires) = &whois.expiration_date {
                    report.push_str(&format!("- **Expires:** {}\n", expires));
                }
                if !whois.name_servers.is_empty() {
                    report.push_str(&format!("- **Name Servers:** {}\n", whois.name_servers.join(", ")));
                }
                report.push('\n');
            }

            // Open Ports
            if !domain.open_ports.is_empty() {
                report.push_str("### Open Ports\n\n");
                report.push_str("| Port | Service | State | Risk |\n");
                report.push_str("|------|---------|-------|------|\n");
                for port in &domain.open_ports {
                    report.push_str(&format!(
                        "| {} | {} | {:?} | {:?} |\n",
                        port.port, port.service, port.state, port.risk_level
                    ));
                }
                report.push('\n');
            }

            // Vulnerabilities
            if !domain.vulnerabilities.is_empty() {
                report.push_str("### Vulnerabilities\n\n");
                for vuln in &domain.vulnerabilities {
                    report.push_str(&format!(
                        "#### {} - {}\n",
                        vuln.cve_id, vuln.title
                    ));
                    report.push_str(&format!("- **Severity:** {} (CVSS: {:.1})\n", vuln.severity, vuln.cvss_score));
                    report.push_str(&format!("- **Description:** {}\n", vuln.description));
                    if vuln.has_exploit {
                        report.push_str("- **⚠️ Exploit Available**\n");
                    }
                    report.push('\n');
                }
            }

            // Exploits
            if !domain.exploits.is_empty() {
                report.push_str("### Available Exploits\n\n");
                for exploit in &domain.exploits {
                    report.push_str(&format!(
                        "- **{}** - {} ({:?})\n",
                        exploit.id, exploit.title, exploit.exploit_type
                    ));
                }
                report.push('\n');
            }

            // Risk Score
            report.push_str(&format!("### Risk Score: **{}/100**\n\n", domain.risk_score));
        }

        // Recon Data
        let recon = &self.data.reconnaissance;
        
        if !recon.subdomains.is_empty() {
            report.push_str("## Subdomains\n\n");
            for sub in &recon.subdomains {
                report.push_str(&format!("- {} ({:?})\n", sub.subdomain, sub.status));
            }
            report.push('\n');
        }

        if let Some(geo) = &recon.geo_location {
            report.push_str("## Geolocation\n\n");
            report.push_str(&format!("- **Country:** {} ({})\n", geo.country, geo.country_code));
            report.push_str(&format!("- **City:** {}, {}\n", geo.city, geo.region));
            report.push_str(&format!("- **ISP:** {}\n", geo.isp));
            report.push_str(&format!("- **Coordinates:** {:.4}, {:.4}\n", geo.latitude, geo.longitude));
            report.push('\n');
        }

        if let Some(asn) = &recon.asn_info {
            report.push_str("## ASN Information\n\n");
            report.push_str(&format!("- **ASN:** {}\n", asn.asn));
            report.push_str(&format!("- **Name:** {}\n", asn.name));
            report.push_str(&format!("- **Country:** {}\n", asn.country));
            report.push('\n');
        }

        report
    }

    /// Generate JSON report
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.data)
            .map_err(|e| format!("JSON serialization error: {}", e))
    }
}

/// OSINT dropdown display item for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSINTDropdown {
    pub domain: String,
    pub risk_score: u8,
    pub summary: OSINTSummary,
    pub expanded: bool,
}

/// Summary for dropdown display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSINTSummary {
    pub whois_registrar: Option<String>,
    pub whois_created: Option<String>,
    pub open_port_count: usize,
    pub high_risk_ports: Vec<String>,
    pub vuln_count: usize,
    pub critical_vulns: Vec<String>,
    pub exploit_count: usize,
    pub exploit_names: Vec<String>,
}

impl OSINTDropdown {
    pub fn from_domain_osint(domain: &str, osint: &DomainOSINT) -> Self {
        let high_risk_ports: Vec<String> = osint.open_ports
            .iter()
            .filter(|p| matches!(p.risk_level, super::portscan::RiskLevel::High | super::portscan::RiskLevel::Critical))
            .map(|p| format!("{}/{}", p.port, p.service))
            .collect();

        let critical_vulns: Vec<String> = osint.vulnerabilities
            .iter()
            .filter(|v| v.severity == "critical" || v.severity == "high")
            .map(|v| v.cve_id.clone())
            .collect();

        let exploit_names: Vec<String> = osint.exploits
            .iter()
            .map(|e| e.title.clone())
            .collect();

        Self {
            domain: domain.to_string(),
            risk_score: osint.risk_score,
            summary: OSINTSummary {
                whois_registrar: osint.whois.as_ref().and_then(|w| w.registrar.clone()),
                whois_created: osint.whois.as_ref().and_then(|w| w.creation_date.map(|d| d.format("%Y-%m-%d").to_string())),
                open_port_count: osint.open_ports.len(),
                high_risk_ports,
                vuln_count: osint.vulnerabilities.len(),
                critical_vulns,
                exploit_count: osint.exploits.len(),
                exploit_names,
            },
            expanded: false,
        }
    }

    /// Generate HTML for dropdown display
    pub fn to_html(&self) -> String {
        let risk_color = if self.risk_score >= 70 {
            "#ff4444"
        } else if self.risk_score >= 40 {
            "#ffaa00"
        } else {
            "#44ff44"
        };

        let mut html = format!(r#"
<details class="osint-dropdown">
    <summary class="osint-summary">
        <span class="domain">{}</span>
        <span class="risk-badge" style="background: {}">Risk: {}/100</span>
    </summary>
    <div class="osint-content">
"#, self.domain, risk_color, self.risk_score);

        // WHOIS section
        if self.summary.whois_registrar.is_some() || self.summary.whois_created.is_some() {
            html.push_str(r#"<div class="osint-section"><h4>📋 WHOIS</h4><ul>"#);
            if let Some(registrar) = &self.summary.whois_registrar {
                html.push_str(&format!("<li>Registrar: {}</li>", registrar));
            }
            if let Some(created) = &self.summary.whois_created {
                html.push_str(&format!("<li>Created: {}</li>", created));
            }
            html.push_str("</ul></div>");
        }

        // Ports section
        if self.summary.open_port_count > 0 {
            html.push_str(&format!(
                r#"<div class="osint-section"><h4>🔌 Open Ports ({})</h4>"#,
                self.summary.open_port_count
            ));
            if !self.summary.high_risk_ports.is_empty() {
                html.push_str("<ul class=\"high-risk\">");
                for port in &self.summary.high_risk_ports {
                    html.push_str(&format!("<li>⚠️ {}</li>", port));
                }
                html.push_str("</ul>");
            }
            html.push_str("</div>");
        }

        // Vulnerabilities section
        if self.summary.vuln_count > 0 {
            html.push_str(&format!(
                r#"<div class="osint-section"><h4>🛡️ Vulnerabilities ({})</h4>"#,
                self.summary.vuln_count
            ));
            if !self.summary.critical_vulns.is_empty() {
                html.push_str("<ul class=\"critical\">");
                for cve in &self.summary.critical_vulns {
                    html.push_str(&format!("<li>🔴 {}</li>", cve));
                }
                html.push_str("</ul>");
            }
            html.push_str("</div>");
        }

        // Exploits section
        if self.summary.exploit_count > 0 {
            html.push_str(&format!(
                r#"<div class="osint-section"><h4>⚡ Exploits ({})</h4><ul>"#,
                self.summary.exploit_count
            ));
            for exploit in &self.summary.exploit_names {
                html.push_str(&format!("<li>{}</li>", exploit));
            }
            html.push_str("</ul></div>");
        }

        html.push_str("</div></details>");
        html
    }
}

/// CSS styles for OSINT dropdowns
pub const OSINT_DROPDOWN_CSS: &str = r#"
.osint-dropdown {
    background: #1a1a2e;
    border: 1px solid #00ff9f;
    border-radius: 4px;
    margin: 8px 0;
    font-family: 'JetBrains Mono', monospace;
}

.osint-summary {
    padding: 10px 15px;
    cursor: pointer;
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: linear-gradient(135deg, #0f0f1a 0%, #1a1a2e 100%);
}

.osint-summary:hover {
    background: #252540;
}

.osint-summary .domain {
    color: #00ff9f;
    font-weight: bold;
}

.risk-badge {
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 12px;
    color: #000;
    font-weight: bold;
}

.osint-content {
    padding: 15px;
    border-top: 1px solid #333;
}

.osint-section {
    margin-bottom: 15px;
}

.osint-section h4 {
    color: #00d4ff;
    margin: 0 0 8px 0;
    font-size: 14px;
}

.osint-section ul {
    margin: 0;
    padding-left: 20px;
    color: #ccc;
}

.osint-section ul.high-risk li,
.osint-section ul.critical li {
    color: #ff6b6b;
}
"#;
