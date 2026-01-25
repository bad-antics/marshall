// Copyright (c) 2026 bad-antics
// OSINT Search - WHOIS Lookup

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::Duration;

/// WHOIS information for a domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhoisInfo {
    pub domain: String,
    pub registrar: Option<String>,
    pub registrant: Option<RegistrantInfo>,
    pub creation_date: Option<DateTime<Utc>>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub updated_date: Option<DateTime<Utc>>,
    pub name_servers: Vec<String>,
    pub status: Vec<String>,
    pub dnssec: Option<bool>,
    pub raw_response: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrantInfo {
    pub name: Option<String>,
    pub organization: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

/// WHOIS lookup service
pub struct WhoisLookup {
    timeout: Duration,
}

impl WhoisLookup {
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(10),
        }
    }

    /// Lookup WHOIS information for a domain
    pub async fn lookup(&self, domain: &str) -> Result<WhoisInfo, String> {
        let tld = domain.rsplit('.').next().unwrap_or("com");
        let whois_server = self.get_whois_server(tld);

        let raw_response = self.query_whois(&whois_server, domain)?;
        let info = self.parse_whois(&raw_response, domain);

        Ok(info)
    }

    /// Get appropriate WHOIS server for TLD
    fn get_whois_server(&self, tld: &str) -> String {
        match tld.to_lowercase().as_str() {
            "com" | "net" => "whois.verisign-grs.com".to_string(),
            "org" => "whois.pir.org".to_string(),
            "info" => "whois.afilias.net".to_string(),
            "io" => "whois.nic.io".to_string(),
            "co" => "whois.nic.co".to_string(),
            "me" => "whois.nic.me".to_string(),
            "biz" => "whois.biz".to_string(),
            "us" => "whois.nic.us".to_string(),
            "uk" | "co.uk" => "whois.nic.uk".to_string(),
            "de" => "whois.denic.de".to_string(),
            "fr" => "whois.afnic.fr".to_string(),
            "nl" => "whois.sidn.nl".to_string(),
            "eu" => "whois.eu".to_string(),
            "ru" => "whois.tcinet.ru".to_string(),
            "cn" => "whois.cnnic.cn".to_string(),
            "jp" => "whois.jprs.jp".to_string(),
            "au" => "whois.auda.org.au".to_string(),
            "ca" => "whois.cira.ca".to_string(),
            "br" => "whois.registro.br".to_string(),
            "in" => "whois.registry.in".to_string(),
            "mx" => "whois.mx".to_string(),
            _ => format!("whois.nic.{}", tld),
        }
    }

    /// Query WHOIS server
    fn query_whois(&self, server: &str, domain: &str) -> Result<String, String> {
        let addr = format!("{}:43", server);
        
        let mut stream = TcpStream::connect_timeout(
            &addr.parse().map_err(|e| format!("Invalid address: {}", e))?,
            self.timeout,
        ).map_err(|e| format!("Connection failed: {}", e))?;

        stream.set_read_timeout(Some(self.timeout))
            .map_err(|e| format!("Failed to set timeout: {}", e))?;

        let query = format!("{}\r\n", domain);
        stream.write_all(query.as_bytes())
            .map_err(|e| format!("Write failed: {}", e))?;

        let mut response = String::new();
        stream.read_to_string(&mut response)
            .map_err(|e| format!("Read failed: {}", e))?;

        Ok(response)
    }

    /// Parse WHOIS response into structured data
    fn parse_whois(&self, raw: &str, domain: &str) -> WhoisInfo {
        let mut info = WhoisInfo {
            domain: domain.to_string(),
            registrar: None,
            registrant: None,
            creation_date: None,
            expiration_date: None,
            updated_date: None,
            name_servers: Vec::new(),
            status: Vec::new(),
            dnssec: None,
            raw_response: raw.to_string(),
        };

        let mut registrant = RegistrantInfo {
            name: None,
            organization: None,
            street: None,
            city: None,
            state: None,
            country: None,
            postal_code: None,
            email: None,
            phone: None,
        };

        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('%') || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim().to_lowercase();
                let value = value.trim();

                match key.as_str() {
                    "registrar" | "registrar name" => {
                        info.registrar = Some(value.to_string());
                    }
                    "creation date" | "created" | "created date" | "registration date" => {
                        info.creation_date = parse_date(value);
                    }
                    "expiration date" | "expires" | "expiry date" | "registry expiry date" => {
                        info.expiration_date = parse_date(value);
                    }
                    "updated date" | "updated" | "last updated" => {
                        info.updated_date = parse_date(value);
                    }
                    "name server" | "nameserver" | "nserver" => {
                        if !value.is_empty() {
                            info.name_servers.push(value.to_lowercase());
                        }
                    }
                    "domain status" | "status" => {
                        info.status.push(value.to_string());
                    }
                    "dnssec" => {
                        info.dnssec = Some(value.to_lowercase().contains("signed") || 
                                          value.to_lowercase() == "yes");
                    }
                    "registrant name" => {
                        registrant.name = Some(value.to_string());
                    }
                    "registrant organization" | "registrant org" => {
                        registrant.organization = Some(value.to_string());
                    }
                    "registrant street" => {
                        registrant.street = Some(value.to_string());
                    }
                    "registrant city" => {
                        registrant.city = Some(value.to_string());
                    }
                    "registrant state/province" | "registrant state" => {
                        registrant.state = Some(value.to_string());
                    }
                    "registrant country" => {
                        registrant.country = Some(value.to_string());
                    }
                    "registrant postal code" => {
                        registrant.postal_code = Some(value.to_string());
                    }
                    "registrant email" => {
                        registrant.email = Some(value.to_string());
                    }
                    "registrant phone" => {
                        registrant.phone = Some(value.to_string());
                    }
                    _ => {}
                }
            }
        }

        // Only include registrant if we have some data
        if registrant.name.is_some() || registrant.organization.is_some() {
            info.registrant = Some(registrant);
        }

        info
    }
}

impl Default for WhoisLookup {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse various date formats from WHOIS responses
fn parse_date(s: &str) -> Option<DateTime<Utc>> {
    // Try common formats
    let formats = [
        "%Y-%m-%dT%H:%M:%SZ",
        "%Y-%m-%dT%H:%M:%S%z",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d",
        "%d-%b-%Y",
        "%Y/%m/%d",
        "%d/%m/%Y",
    ];

    for fmt in &formats {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, fmt) {
            return Some(DateTime::from_naive_utc_and_offset(dt, Utc));
        }
        if let Ok(dt) = chrono::NaiveDate::parse_from_str(s, fmt) {
            return Some(DateTime::from_naive_utc_and_offset(
                dt.and_hms_opt(0, 0, 0).unwrap(),
                Utc,
            ));
        }
    }

    // Try parsing ISO 8601
    s.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_whois_server() {
        let lookup = WhoisLookup::new();
        assert_eq!(lookup.get_whois_server("com"), "whois.verisign-grs.com");
        assert_eq!(lookup.get_whois_server("org"), "whois.pir.org");
        assert_eq!(lookup.get_whois_server("io"), "whois.nic.io");
    }

    #[test]
    fn test_parse_date() {
        assert!(parse_date("2024-01-15T10:30:00Z").is_some());
        assert!(parse_date("2024-01-15").is_some());
    }
}
