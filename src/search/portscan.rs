// Copyright (c) 2026 bad-antics
// OSINT Search - Port Scanner

use serde::{Deserialize, Serialize};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::sync::Arc;

/// Information about an open port
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortInfo {
    pub port: u16,
    pub service: String,
    pub state: PortState,
    pub banner: Option<String>,
    pub version: Option<String>,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Common ports to scan with their services
pub const COMMON_PORTS: &[(u16, &str, RiskLevel)] = &[
    // High-risk ports
    (21, "FTP", RiskLevel::High),
    (22, "SSH", RiskLevel::Medium),
    (23, "Telnet", RiskLevel::Critical),
    (25, "SMTP", RiskLevel::Medium),
    (53, "DNS", RiskLevel::Medium),
    (80, "HTTP", RiskLevel::Low),
    (110, "POP3", RiskLevel::Medium),
    (111, "RPC", RiskLevel::High),
    (135, "MSRPC", RiskLevel::High),
    (139, "NetBIOS", RiskLevel::High),
    (143, "IMAP", RiskLevel::Medium),
    (443, "HTTPS", RiskLevel::Low),
    (445, "SMB", RiskLevel::Critical),
    (993, "IMAPS", RiskLevel::Low),
    (995, "POP3S", RiskLevel::Low),
    (1433, "MSSQL", RiskLevel::High),
    (1521, "Oracle", RiskLevel::High),
    (3306, "MySQL", RiskLevel::High),
    (3389, "RDP", RiskLevel::Critical),
    (5432, "PostgreSQL", RiskLevel::High),
    (5900, "VNC", RiskLevel::High),
    (6379, "Redis", RiskLevel::High),
    (8080, "HTTP-Proxy", RiskLevel::Medium),
    (8443, "HTTPS-Alt", RiskLevel::Low),
    (27017, "MongoDB", RiskLevel::High),
];

/// Extended port list for thorough scans
pub const EXTENDED_PORTS: &[(u16, &str)] = &[
    (69, "TFTP"),
    (79, "Finger"),
    (88, "Kerberos"),
    (113, "Auth"),
    (119, "NNTP"),
    (123, "NTP"),
    (137, "NetBIOS-NS"),
    (138, "NetBIOS-DGM"),
    (161, "SNMP"),
    (162, "SNMP-Trap"),
    (179, "BGP"),
    (389, "LDAP"),
    (427, "SLP"),
    (464, "Kerberos"),
    (500, "ISAKMP"),
    (514, "Syslog"),
    (515, "LPD"),
    (520, "RIP"),
    (523, "IBM-DB2"),
    (548, "AFP"),
    (554, "RTSP"),
    (587, "SMTP-Submission"),
    (623, "IPMI"),
    (631, "IPP"),
    (636, "LDAPS"),
    (873, "Rsync"),
    (902, "VMware"),
    (912, "VMware"),
    (1080, "SOCKS"),
    (1099, "Java-RMI"),
    (1194, "OpenVPN"),
    (1723, "PPTP"),
    (1883, "MQTT"),
    (2049, "NFS"),
    (2181, "ZooKeeper"),
    (2375, "Docker"),
    (2376, "Docker-TLS"),
    (3000, "Dev-Server"),
    (3128, "Squid"),
    (4444, "Metasploit"),
    (4443, "HTTPS-Alt"),
    (4848, "GlassFish"),
    (5000, "Flask/Docker"),
    (5001, "Synology"),
    (5060, "SIP"),
    (5061, "SIP-TLS"),
    (5672, "RabbitMQ"),
    (5901, "VNC-1"),
    (5984, "CouchDB"),
    (6000, "X11"),
    (6443, "Kubernetes"),
    (7001, "WebLogic"),
    (8000, "HTTP-Alt"),
    (8008, "HTTP-Alt"),
    (8081, "HTTP-Alt"),
    (8083, "HTTP-Alt"),
    (8181, "HTTP-Alt"),
    (8888, "HTTP-Alt"),
    (9000, "PHP-FPM"),
    (9090, "WebSM"),
    (9200, "Elasticsearch"),
    (9418, "Git"),
    (10000, "Webmin"),
    (11211, "Memcached"),
    (27018, "MongoDB"),
];

/// Port scanner
pub struct PortScanner {
    timeout: Duration,
    max_concurrent: usize,
}

impl PortScanner {
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            timeout: Duration::from_secs(timeout_secs.min(30)),
            max_concurrent: 50,
        }
    }

    /// Scan common ports on a target
    pub async fn scan(&self, target: &str) -> Result<Vec<PortInfo>, String> {
        let mut results = Vec::new();
        
        // Resolve hostname first
        let addr = format!("{}:80", target);
        let socket_addrs: Vec<_> = addr.to_socket_addrs()
            .map_err(|e| format!("Failed to resolve {}: {}", target, e))?
            .collect();
        
        if socket_addrs.is_empty() {
            return Err(format!("Could not resolve hostname: {}", target));
        }

        let ip = socket_addrs[0].ip();

        // Scan common ports
        for (port, service, risk_level) in COMMON_PORTS {
            let port_info = self.scan_port(ip.to_string().as_str(), *port, service, risk_level.clone());
            if port_info.state == PortState::Open {
                results.push(port_info);
            }
        }

        Ok(results)
    }

    /// Scan all extended ports
    pub async fn scan_extended(&self, target: &str) -> Result<Vec<PortInfo>, String> {
        let mut results = self.scan(target).await?;
        
        let addr = format!("{}:80", target);
        let socket_addrs: Vec<_> = addr.to_socket_addrs()
            .map_err(|_| "Resolution failed".to_string())?
            .collect();
        
        let ip = socket_addrs[0].ip();

        for (port, service) in EXTENDED_PORTS {
            let port_info = self.scan_port(&ip.to_string(), *port, service, RiskLevel::Medium);
            if port_info.state == PortState::Open {
                results.push(port_info);
            }
        }

        results.sort_by_key(|p| p.port);
        Ok(results)
    }

    /// Scan specific port range
    pub async fn scan_range(&self, target: &str, start: u16, end: u16) -> Result<Vec<PortInfo>, String> {
        let mut results = Vec::new();
        
        let addr = format!("{}:80", target);
        let socket_addrs: Vec<_> = addr.to_socket_addrs()
            .map_err(|_| "Resolution failed".to_string())?
            .collect();
        
        let ip = socket_addrs[0].ip();

        for port in start..=end {
            let service = self.identify_service(port);
            let port_info = self.scan_port(&ip.to_string(), port, &service, RiskLevel::Medium);
            if port_info.state == PortState::Open {
                results.push(port_info);
            }
        }

        Ok(results)
    }

    /// Scan a single port
    fn scan_port(&self, ip: &str, port: u16, service: &str, risk_level: RiskLevel) -> PortInfo {
        let addr = format!("{}:{}", ip, port);
        
        let state = match TcpStream::connect_timeout(
            &addr.parse().unwrap(),
            Duration::from_millis(500),
        ) {
            Ok(mut stream) => {
                // Try to grab banner
                let banner = self.grab_banner(&mut stream, port);
                return PortInfo {
                    port,
                    service: service.to_string(),
                    state: PortState::Open,
                    banner,
                    version: None,
                    risk_level,
                };
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::ConnectionRefused {
                    PortState::Closed
                } else {
                    PortState::Filtered
                }
            }
        };

        PortInfo {
            port,
            service: service.to_string(),
            state,
            banner: None,
            version: None,
            risk_level,
        }
    }

    /// Try to grab a service banner
    fn grab_banner(&self, stream: &mut TcpStream, port: u16) -> Option<String> {
        use std::io::{Read, Write};
        
        stream.set_read_timeout(Some(Duration::from_secs(2))).ok()?;
        stream.set_write_timeout(Some(Duration::from_secs(2))).ok()?;

        // Send appropriate probe based on port
        let probe = match port {
            80 | 8080 | 8000 | 8443 => b"HEAD / HTTP/1.0\r\n\r\n".to_vec(),
            443 => return None, // TLS needs special handling
            21 | 25 | 110 | 143 => vec![], // These send banners immediately
            _ => vec![],
        };

        if !probe.is_empty() {
            stream.write_all(&probe).ok()?;
        }

        let mut buffer = [0u8; 1024];
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let banner = String::from_utf8_lossy(&buffer[..n])
                    .trim()
                    .chars()
                    .filter(|c| c.is_ascii_graphic() || c.is_ascii_whitespace())
                    .take(200)
                    .collect();
                Some(banner)
            }
            _ => None,
        }
    }

    /// Identify service by port number
    fn identify_service(&self, port: u16) -> String {
        // Check common ports first
        for (p, service, _) in COMMON_PORTS {
            if *p == port {
                return service.to_string();
            }
        }
        
        // Check extended ports
        for (p, service) in EXTENDED_PORTS {
            if *p == port {
                return service.to_string();
            }
        }

        "Unknown".to_string()
    }
}

impl Default for PortScanner {
    fn default() -> Self {
        Self::new(10)
    }
}

/// Format port scan results for display
pub fn format_scan_results(ports: &[PortInfo]) -> String {
    let mut output = String::new();
    
    output.push_str("┌─────────────────────────────────────────┐\n");
    output.push_str("│           PORT SCAN RESULTS             │\n");
    output.push_str("├───────┬──────────────┬─────────┬────────┤\n");
    output.push_str("│ PORT  │ SERVICE      │ STATE   │ RISK   │\n");
    output.push_str("├───────┼──────────────┼─────────┼────────┤\n");

    for port in ports {
        let risk = match port.risk_level {
            RiskLevel::Low => "LOW",
            RiskLevel::Medium => "MED",
            RiskLevel::High => "HIGH",
            RiskLevel::Critical => "CRIT",
        };
        output.push_str(&format!(
            "│ {:5} │ {:12} │ {:7} │ {:6} │\n",
            port.port,
            &port.service[..port.service.len().min(12)],
            "OPEN",
            risk
        ));
    }

    output.push_str("└───────┴──────────────┴─────────┴────────┘\n");
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_identification() {
        let scanner = PortScanner::new(5);
        assert_eq!(scanner.identify_service(22), "SSH");
        assert_eq!(scanner.identify_service(80), "HTTP");
        assert_eq!(scanner.identify_service(443), "HTTPS");
    }
}
