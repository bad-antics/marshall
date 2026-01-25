// Copyright (c) 2026 bad-antics
// Marshall UI - OSINT Search Results Display

use gtk::prelude::*;
use gtk::{self, Box as GtkBox, Orientation, Label, Button, Frame, Expander};
use crate::search::{SearchResult, WebResult, DomainOSINT, OSINTDropdown};

/// Search result card widget
pub struct SearchResultCard {
    pub container: GtkBox,
    pub url: String,
    pub expanded: bool,
}

impl SearchResultCard {
    pub fn new(result: &WebResult) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 5);
        container.set_widget_name("search-result-card");
        container.set_margin_start(10);
        container.set_margin_end(10);
        container.set_margin_top(5);
        container.set_margin_bottom(5);

        // Title (clickable)
        let title_btn = Button::with_label(&result.title);
        title_btn.set_widget_name("result-title");
        container.pack_start(&title_btn, false, false, 0);

        // URL
        let url_label = Label::new(Some(&result.url));
        url_label.set_widget_name("result-url");
        url_label.set_halign(gtk::Align::Start);
        url_label.set_ellipsize(pango::EllipsizeMode::End);
        container.pack_start(&url_label, false, false, 0);

        // OSINT Dropdown instead of regular snippet
        if let Some(osint) = &result.osint {
            let dropdown = Self::create_osint_dropdown(&result.domain, osint);
            container.pack_start(&dropdown, false, false, 5);
        } else {
            // Fallback to regular snippet
            let snippet_label = Label::new(Some(&result.snippet));
            snippet_label.set_widget_name("result-snippet");
            snippet_label.set_halign(gtk::Align::Start);
            snippet_label.set_line_wrap(true);
            snippet_label.set_max_width_chars(100);
            container.pack_start(&snippet_label, false, false, 0);
        }

        Self {
            container,
            url: result.url.clone(),
            expanded: false,
        }
    }

    fn create_osint_dropdown(domain: &str, osint: &DomainOSINT) -> Expander {
        let dropdown = OSINTDropdown::from_domain_osint(domain, osint);
        
        // Create expander with risk badge
        let risk_color = if dropdown.risk_score >= 70 {
            "#ff4444"
        } else if dropdown.risk_score >= 40 {
            "#ffaa00"
        } else {
            "#44ff44"
        };

        let header_box = GtkBox::new(Orientation::Horizontal, 10);
        let domain_label = Label::new(Some(&format!("📊 {} OSINT Data", domain)));
        domain_label.set_widget_name("osint-domain");
        header_box.pack_start(&domain_label, false, false, 0);

        let risk_label = Label::new(Some(&format!("Risk: {}/100", dropdown.risk_score)));
        risk_label.set_widget_name("risk-badge");
        header_box.pack_end(&risk_label, false, false, 0);

        let expander = Expander::new(None);
        expander.set_label_widget(Some(&header_box));
        expander.set_widget_name("osint-expander");

        // Content
        let content = GtkBox::new(Orientation::Vertical, 8);
        content.set_margin_start(15);
        content.set_margin_top(10);
        content.set_margin_bottom(10);

        // WHOIS Section
        if let Some(registrar) = &dropdown.summary.whois_registrar {
            let whois_frame = Frame::new(Some("📋 WHOIS"));
            let whois_box = GtkBox::new(Orientation::Vertical, 3);
            whois_box.set_margin_start(5);
            whois_box.set_margin_top(5);
            whois_box.set_margin_bottom(5);

            let registrar_label = Label::new(Some(&format!("Registrar: {}", registrar)));
            registrar_label.set_halign(gtk::Align::Start);
            whois_box.pack_start(&registrar_label, false, false, 0);

            if let Some(created) = &dropdown.summary.whois_created {
                let created_label = Label::new(Some(&format!("Created: {}", created)));
                created_label.set_halign(gtk::Align::Start);
                whois_box.pack_start(&created_label, false, false, 0);
            }

            whois_frame.add(&whois_box);
            content.pack_start(&whois_frame, false, false, 0);
        }

        // Ports Section
        if dropdown.summary.open_port_count > 0 {
            let ports_frame = Frame::new(Some(&format!("🔌 Open Ports ({})", dropdown.summary.open_port_count)));
            let ports_box = GtkBox::new(Orientation::Vertical, 3);
            ports_box.set_margin_start(5);
            ports_box.set_margin_top(5);
            ports_box.set_margin_bottom(5);

            for port in &dropdown.summary.high_risk_ports {
                let port_label = Label::new(Some(&format!("⚠️ {}", port)));
                port_label.set_widget_name("high-risk-port");
                port_label.set_halign(gtk::Align::Start);
                ports_box.pack_start(&port_label, false, false, 0);
            }

            ports_frame.add(&ports_box);
            content.pack_start(&ports_frame, false, false, 0);
        }

        // Vulnerabilities Section
        if dropdown.summary.vuln_count > 0 {
            let vuln_frame = Frame::new(Some(&format!("🛡️ Vulnerabilities ({})", dropdown.summary.vuln_count)));
            let vuln_box = GtkBox::new(Orientation::Vertical, 3);
            vuln_box.set_margin_start(5);
            vuln_box.set_margin_top(5);
            vuln_box.set_margin_bottom(5);

            for cve in &dropdown.summary.critical_vulns {
                let cve_label = Label::new(Some(&format!("🔴 {}", cve)));
                cve_label.set_widget_name("critical-vuln");
                cve_label.set_halign(gtk::Align::Start);
                vuln_box.pack_start(&cve_label, false, false, 0);
            }

            vuln_frame.add(&vuln_box);
            content.pack_start(&vuln_frame, false, false, 0);
        }

        // Exploits Section
        if dropdown.summary.exploit_count > 0 {
            let exploit_frame = Frame::new(Some(&format!("⚡ Exploits ({})", dropdown.summary.exploit_count)));
            let exploit_box = GtkBox::new(Orientation::Vertical, 3);
            exploit_box.set_margin_start(5);
            exploit_box.set_margin_top(5);
            exploit_box.set_margin_bottom(5);

            for exploit in &dropdown.summary.exploit_names {
                let exploit_label = Label::new(Some(&format!("💥 {}", exploit)));
                exploit_label.set_widget_name("exploit-item");
                exploit_label.set_halign(gtk::Align::Start);
                exploit_box.pack_start(&exploit_label, false, false, 0);
            }

            exploit_frame.add(&exploit_box);
            content.pack_start(&exploit_frame, false, false, 0);
        }

        expander.add(&content);
        expander
    }

    pub fn get_widget(&self) -> &GtkBox {
        &self.container
    }
}

/// Full search results panel
pub struct SearchResultsPanel {
    pub container: GtkBox,
    pub results: Vec<SearchResultCard>,
}

impl SearchResultsPanel {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 0);
        container.set_widget_name("search-results-panel");

        Self {
            container,
            results: Vec::new(),
        }
    }

    pub fn display_results(&mut self, search_result: &SearchResult) {
        // Clear previous results
        self.clear();

        // Header
        let header = GtkBox::new(Orientation::Horizontal, 10);
        header.set_margin_start(10);
        header.set_margin_top(10);
        header.set_margin_bottom(10);

        let query_label = Label::new(Some(&format!("Results for: \"{}\"", search_result.query)));
        query_label.set_widget_name("results-query");
        header.pack_start(&query_label, false, false, 0);

        let count_label = Label::new(Some(&format!(
            "{} results ({} ms)",
            search_result.total_results,
            search_result.search_time_ms
        )));
        count_label.set_widget_name("results-count");
        header.pack_end(&count_label, false, false, 0);

        self.container.pack_start(&header, false, false, 0);

        // Target OSINT section if available
        if let Some(osint_data) = &search_result.osint_data {
            if let Some(domain_osint) = &osint_data.domain_osint {
                let target_frame = Frame::new(Some(&format!("🎯 Target Analysis: {}", osint_data.target)));
                target_frame.set_widget_name("target-osint-frame");
                
                let target_content = Self::create_full_osint_panel(domain_osint);
                target_frame.add(&target_content);
                self.container.pack_start(&target_frame, false, false, 10);
            }
        }

        // Separator
        let sep = gtk::Separator::new(Orientation::Horizontal);
        self.container.pack_start(&sep, false, false, 5);

        // Web results with OSINT dropdowns
        for web_result in &search_result.web_results {
            let card = SearchResultCard::new(web_result);
            self.container.pack_start(card.get_widget(), false, false, 0);
            self.results.push(card);

            // Separator between results
            let sep = gtk::Separator::new(Orientation::Horizontal);
            sep.set_margin_start(10);
            sep.set_margin_end(10);
            self.container.pack_start(&sep, false, false, 2);
        }

        self.container.show_all();
    }

    fn create_full_osint_panel(osint: &DomainOSINT) -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, 10);
        panel.set_margin_start(10);
        panel.set_margin_end(10);
        panel.set_margin_top(10);
        panel.set_margin_bottom(10);

        // Risk score banner
        let risk_color = if osint.risk_score >= 70 {
            "🔴 CRITICAL"
        } else if osint.risk_score >= 40 {
            "🟠 ELEVATED"
        } else {
            "🟢 LOW"
        };

        let risk_box = GtkBox::new(Orientation::Horizontal, 10);
        let risk_label = Label::new(Some(&format!(
            "Risk Assessment: {} ({}/100)",
            risk_color, osint.risk_score
        )));
        risk_label.set_widget_name("risk-assessment");
        risk_box.pack_start(&risk_label, false, false, 0);
        panel.pack_start(&risk_box, false, false, 0);

        // Grid layout for OSINT sections
        let grid = gtk::Grid::new();
        grid.set_row_spacing(10);
        grid.set_column_spacing(15);

        let mut row = 0;

        // WHOIS
        if let Some(whois) = &osint.whois {
            let whois_label = Label::new(Some("📋 WHOIS"));
            whois_label.set_widget_name("osint-section-label");
            whois_label.set_halign(gtk::Align::Start);
            grid.attach(&whois_label, 0, row, 1, 1);

            let mut whois_text = String::new();
            if let Some(registrar) = &whois.registrar {
                whois_text.push_str(&format!("Registrar: {}\n", registrar));
            }
            if !whois.name_servers.is_empty() {
                whois_text.push_str(&format!("NS: {}\n", whois.name_servers.join(", ")));
            }
            let whois_value = Label::new(Some(&whois_text.trim()));
            whois_value.set_halign(gtk::Align::Start);
            grid.attach(&whois_value, 1, row, 1, 1);
            row += 1;
        }

        // Open Ports
        if !osint.open_ports.is_empty() {
            let ports_label = Label::new(Some("🔌 Open Ports"));
            ports_label.set_widget_name("osint-section-label");
            ports_label.set_halign(gtk::Align::Start);
            grid.attach(&ports_label, 0, row, 1, 1);

            let ports_text: String = osint.open_ports
                .iter()
                .map(|p| format!("{}/{}", p.port, p.service))
                .collect::<Vec<_>>()
                .join(", ");
            let ports_value = Label::new(Some(&ports_text));
            ports_value.set_halign(gtk::Align::Start);
            ports_value.set_line_wrap(true);
            grid.attach(&ports_value, 1, row, 1, 1);
            row += 1;
        }

        // Vulnerabilities
        if !osint.vulnerabilities.is_empty() {
            let vuln_label = Label::new(Some("🛡️ Vulnerabilities"));
            vuln_label.set_widget_name("osint-section-label");
            vuln_label.set_halign(gtk::Align::Start);
            grid.attach(&vuln_label, 0, row, 1, 1);

            let vuln_box = GtkBox::new(Orientation::Vertical, 2);
            for vuln in &osint.vulnerabilities {
                let severity_icon = match vuln.severity.as_str() {
                    "critical" => "🔴",
                    "high" => "🟠",
                    "medium" => "🟡",
                    _ => "🟢",
                };
                let vuln_line = Label::new(Some(&format!(
                    "{} {} - {} (CVSS: {:.1})",
                    severity_icon, vuln.cve_id, vuln.title, vuln.cvss_score
                )));
                vuln_line.set_halign(gtk::Align::Start);
                vuln_box.pack_start(&vuln_line, false, false, 0);
            }
            grid.attach(&vuln_box, 1, row, 1, 1);
            row += 1;
        }

        // Exploits
        if !osint.exploits.is_empty() {
            let exploit_label = Label::new(Some("⚡ Exploits"));
            exploit_label.set_widget_name("osint-section-label");
            exploit_label.set_halign(gtk::Align::Start);
            grid.attach(&exploit_label, 0, row, 1, 1);

            let exploit_box = GtkBox::new(Orientation::Vertical, 2);
            for exploit in &osint.exploits {
                let exploit_line = Label::new(Some(&format!(
                    "💥 {} - {}",
                    exploit.id, exploit.title
                )));
                exploit_line.set_halign(gtk::Align::Start);
                exploit_box.pack_start(&exploit_line, false, false, 0);
            }
            grid.attach(&exploit_box, 1, row, 1, 1);
        }

        panel.pack_start(&grid, false, false, 0);
        panel
    }

    pub fn clear(&mut self) {
        // Remove all children
        for child in self.container.children() {
            self.container.remove(&child);
        }
        self.results.clear();
    }

    pub fn get_widget(&self) -> &GtkBox {
        &self.container
    }
}

impl Default for SearchResultsPanel {
    fn default() -> Self {
        Self::new()
    }
}

/// CSS for search results
pub const SEARCH_RESULTS_CSS: &str = r#"
#search-results-panel {
    background: #0f0f1a;
}

#search-result-card {
    background: #1a1a2e;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 12px;
    margin: 5px 0;
}

#search-result-card:hover {
    border-color: #00ff9f;
    background: #1f1f35;
}

#result-title {
    background: transparent;
    border: none;
    color: #00d4ff;
    font-size: 16px;
    font-weight: bold;

    padding: 0;
}

#result-title:hover {
    color: #00ff9f;

}

#result-url {
    color: #22c55e;
    font-size: 12px;
    font-family: 'JetBrains Mono', monospace;
}

#result-snippet {
    color: #aaa;
    font-size: 13px;
    margin-top: 5px;
}

#osint-expander {
    background: rgba(0, 255, 159, 0.05);
    border: 1px solid #00ff9f;
    border-radius: 4px;
    padding: 8px;
}

#osint-domain {
    color: #00ff9f;
    font-weight: bold;
}

#risk-badge {
    padding: 2px 8px;
    border-radius: 10px;
    font-size: 11px;
    font-weight: bold;
}

.high-risk-port {
    color: #ff6b6b;
}

.critical-vuln {
    color: #ff4444;
    font-weight: bold;
}

.exploit-item {
    color: #fbbf24;
}

#target-osint-frame {
    background: rgba(0, 212, 255, 0.05);
    border: 2px solid #00d4ff;
    border-radius: 8px;
    margin: 10px;
}

#risk-assessment {
    font-size: 18px;
    font-weight: bold;
}

#osint-section-label {
    color: #00d4ff;
    font-weight: bold;
}

#results-query {
    font-size: 16px;
    color: #fff;
}

#results-count {
    color: #888;
    font-size: 12px;
}
"#;
