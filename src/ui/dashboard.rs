// Copyright (c) 2026 bad-antics
// Marshall UI - Main Dashboard

use gtk::prelude::*;
use gtk::{self, Box as GtkBox, Orientation, Label, Button, Frame, Grid, Notebook};
use std::sync::Arc;
use parking_lot::RwLock;

use crate::assistant::Assistant;
use crate::voip::VoIP;
use crate::workforce::WorkforceCenter;
use crate::search::SearchEngine;

/// Dashboard configuration
#[derive(Debug, Clone)]
pub struct DashboardConfig {
    pub show_assistant: bool,
    pub show_voip: bool,
    pub show_workforce: bool,
    pub show_osint: bool,
    pub theme: String,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            show_assistant: true,
            show_voip: true,
            show_workforce: true,
            show_osint: true,
            theme: "dark".to_string(),
        }
    }
}

/// Main dashboard widget
pub struct Dashboard {
    pub container: GtkBox,
    pub config: DashboardConfig,
    pub notebook: Notebook,
}

impl Dashboard {
    pub fn new(config: DashboardConfig) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 0);
        container.set_widget_name("marshall-dashboard");

        // Header bar
        let header = Self::create_header();
        container.pack_start(&header, false, false, 0);

        // Main notebook for different sections
        let notebook = Notebook::new();
        notebook.set_tab_pos(gtk::PositionType::Left);
        notebook.set_widget_name("dashboard-notebook");

        // Add tabs based on config
        if config.show_assistant {
            let assistant_panel = Self::create_assistant_panel();
            let label = Label::new(Some("🤖 Assistant"));
            notebook.append_page(&assistant_panel, Some(&label));
        }

        if config.show_voip {
            let voip_panel = Self::create_voip_panel();
            let label = Label::new(Some("📞 VoIP"));
            notebook.append_page(&voip_panel, Some(&label));
        }

        if config.show_workforce {
            let workforce_panel = Self::create_workforce_panel();
            let label = Label::new(Some("👥 Workforce"));
            notebook.append_page(&workforce_panel, Some(&label));
        }

        if config.show_osint {
            let osint_panel = Self::create_osint_panel();
            let label = Label::new(Some("🔍 OSINT"));
            notebook.append_page(&osint_panel, Some(&label));
        }

        container.pack_start(&notebook, true, true, 0);

        // Status bar
        let status = Self::create_status_bar();
        container.pack_end(&status, false, false, 0);

        Self {
            container,
            config,
            notebook,
        }
    }

    fn create_header() -> GtkBox {
        let header = GtkBox::new(Orientation::Horizontal, 10);
        header.set_widget_name("dashboard-header");
        header.set_margin_start(10);
        header.set_margin_end(10);
        header.set_margin_top(10);
        header.set_margin_bottom(10);

        // Logo/title
        let title = Label::new(Some("MARSHALL COMMAND CENTER"));
        title.set_widget_name("header-title");
        header.pack_start(&title, false, false, 0);

        // Spacer
        let spacer = GtkBox::new(Orientation::Horizontal, 0);
        header.pack_start(&spacer, true, true, 0);

        // Quick action buttons
        let search_btn = Button::with_label("🔍 Search");
        search_btn.set_widget_name("header-btn");
        header.pack_end(&search_btn, false, false, 5);

        let call_btn = Button::with_label("📞 Call");
        call_btn.set_widget_name("header-btn");
        header.pack_end(&call_btn, false, false, 5);

        let clock_btn = Button::with_label("⏱️ Clock In");
        clock_btn.set_widget_name("header-btn");
        header.pack_end(&clock_btn, false, false, 5);

        header
    }

    fn create_assistant_panel() -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, 10);
        panel.set_margin_start(20);
        panel.set_margin_end(20);
        panel.set_margin_top(20);

        // Avatar display area
        let avatar_frame = Frame::new(Some("Avatar"));
        let avatar_area = gtk::DrawingArea::new();
        avatar_area.set_size_request(300, 300);
        avatar_frame.add(&avatar_area);
        panel.pack_start(&avatar_frame, false, false, 0);

        // Chat/conversation area
        let chat_frame = Frame::new(Some("Conversation"));
        let chat_box = GtkBox::new(Orientation::Vertical, 5);
        
        let chat_scroll = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        chat_scroll.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        let chat_view = gtk::TextView::new();
        chat_view.set_editable(false);
        chat_view.set_wrap_mode(gtk::WrapMode::Word);
        chat_scroll.add(&chat_view);
        chat_box.pack_start(&chat_scroll, true, true, 0);

        // Input area
        let input_box = GtkBox::new(Orientation::Horizontal, 5);
        let input_entry = gtk::Entry::new();
        input_entry.set_placeholder_text(Some("Ask the assistant..."));
        input_box.pack_start(&input_entry, true, true, 0);

        let voice_btn = Button::with_label("🎤");
        input_box.pack_end(&voice_btn, false, false, 0);

        let send_btn = Button::with_label("Send");
        input_box.pack_end(&send_btn, false, false, 0);

        chat_box.pack_end(&input_box, false, false, 5);
        chat_frame.add(&chat_box);
        panel.pack_start(&chat_frame, true, true, 0);

        panel
    }

    fn create_voip_panel() -> GtkBox {
        let panel = GtkBox::new(Orientation::Horizontal, 10);
        panel.set_margin_start(20);
        panel.set_margin_end(20);
        panel.set_margin_top(20);

        // Left: Dialpad and controls
        let dialpad_frame = Frame::new(Some("Dialpad"));
        let dialpad_box = GtkBox::new(Orientation::Vertical, 5);
        dialpad_box.set_margin_start(10);
        dialpad_box.set_margin_end(10);
        dialpad_box.set_margin_top(10);
        dialpad_box.set_margin_bottom(10);

        // Number display
        let number_entry = gtk::Entry::new();
        number_entry.set_placeholder_text(Some("Enter number..."));
        dialpad_box.pack_start(&number_entry, false, false, 5);

        // Dialpad grid
        let grid = Grid::new();
        grid.set_row_spacing(5);
        grid.set_column_spacing(5);

        let buttons = [
            ("1", 0, 0), ("2", 0, 1), ("3", 0, 2),
            ("4", 1, 0), ("5", 1, 1), ("6", 1, 2),
            ("7", 2, 0), ("8", 2, 1), ("9", 2, 2),
            ("*", 3, 0), ("0", 3, 1), ("#", 3, 2),
        ];

        for (label, row, col) in buttons {
            let btn = Button::with_label(label);
            btn.set_size_request(60, 50);
            grid.attach(&btn, col, row, 1, 1);
        }

        dialpad_box.pack_start(&grid, false, false, 10);

        // Call/Hangup buttons
        let action_box = GtkBox::new(Orientation::Horizontal, 10);
        let call_btn = Button::with_label("📞 Call");
        call_btn.set_widget_name("call-btn");
        action_box.pack_start(&call_btn, true, true, 0);

        let hangup_btn = Button::with_label("📵 Hangup");
        hangup_btn.set_widget_name("hangup-btn");
        action_box.pack_start(&hangup_btn, true, true, 0);

        dialpad_box.pack_start(&action_box, false, false, 5);

        dialpad_frame.add(&dialpad_box);
        panel.pack_start(&dialpad_frame, false, false, 0);

        // Right: Call queue and history
        let right_box = GtkBox::new(Orientation::Vertical, 10);

        // Call queue
        let queue_frame = Frame::new(Some("Call Queue"));
        let queue_list = gtk::ListBox::new();
        let queue_scroll = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        queue_scroll.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        queue_scroll.add(&queue_list);
        queue_frame.add(&queue_scroll);
        right_box.pack_start(&queue_frame, true, true, 0);

        // Recent calls
        let history_frame = Frame::new(Some("Recent Calls"));
        let history_list = gtk::ListBox::new();
        let history_scroll = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        history_scroll.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        history_scroll.add(&history_list);
        history_frame.add(&history_scroll);
        right_box.pack_start(&history_frame, true, true, 0);

        panel.pack_start(&right_box, true, true, 0);

        panel
    }

    fn create_workforce_panel() -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, 10);
        panel.set_margin_start(20);
        panel.set_margin_end(20);
        panel.set_margin_top(20);

        // Stats bar
        let stats_box = GtkBox::new(Orientation::Horizontal, 20);
        
        let stat_items = [
            ("👥 Active Workers", "12"),
            ("⏱️ Hours Today", "87.5"),
            ("📋 Active Projects", "8"),
            ("💰 Pending Payouts", "$4,250"),
        ];

        for (label, value) in stat_items {
            let stat_frame = Frame::new(Some(label));
            let stat_label = Label::new(Some(value));
            stat_label.set_widget_name("stat-value");
            stat_frame.add(&stat_label);
            stats_box.pack_start(&stat_frame, true, true, 0);
        }

        panel.pack_start(&stats_box, false, false, 0);

        // Main content notebook
        let notebook = Notebook::new();
        notebook.set_tab_pos(gtk::PositionType::Top);

        // Employees tab
        let employees_box = Self::create_employees_tab();
        notebook.append_page(&employees_box, Some(&Label::new(Some("Employees"))));

        // Timecards tab
        let timecards_box = Self::create_timecards_tab();
        notebook.append_page(&timecards_box, Some(&Label::new(Some("Timecards"))));

        // Projects tab
        let projects_box = Self::create_projects_tab();
        notebook.append_page(&projects_box, Some(&Label::new(Some("Projects"))));

        // Payouts tab
        let payouts_box = Self::create_payouts_tab();
        notebook.append_page(&payouts_box, Some(&Label::new(Some("Payouts"))));

        panel.pack_start(&notebook, true, true, 0);

        panel
    }

    fn create_employees_tab() -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, 10);
        panel.set_margin_top(10);

        // Toolbar
        let toolbar = GtkBox::new(Orientation::Horizontal, 5);
        let add_btn = Button::with_label("➕ Add Employee");
        toolbar.pack_start(&add_btn, false, false, 5);
        let search_entry = gtk::Entry::new();
        search_entry.set_placeholder_text(Some("Search employees..."));
        toolbar.pack_end(&search_entry, false, false, 5);
        panel.pack_start(&toolbar, false, false, 0);

        // Employee list (placeholder tree view)
        let scroll = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        
        let list = gtk::ListBox::new();
        scroll.add(&list);
        panel.pack_start(&scroll, true, true, 0);

        panel
    }

    fn create_timecards_tab() -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, 10);
        panel.set_margin_top(10);

        // Date selector
        let date_box = GtkBox::new(Orientation::Horizontal, 10);
        let date_label = Label::new(Some("Date Range:"));
        date_box.pack_start(&date_label, false, false, 5);
        let start_entry = gtk::Entry::new();
        start_entry.set_placeholder_text(Some("Start date"));
        date_box.pack_start(&start_entry, false, false, 0);
        let to_label = Label::new(Some("to"));
        date_box.pack_start(&to_label, false, false, 5);
        let end_entry = gtk::Entry::new();
        end_entry.set_placeholder_text(Some("End date"));
        date_box.pack_start(&end_entry, false, false, 0);
        panel.pack_start(&date_box, false, false, 0);

        // Timecard list
        let scroll = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        let list = gtk::ListBox::new();
        scroll.add(&list);
        panel.pack_start(&scroll, true, true, 0);

        panel
    }

    fn create_projects_tab() -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, 10);
        panel.set_margin_top(10);

        // Toolbar
        let toolbar = GtkBox::new(Orientation::Horizontal, 5);
        let add_btn = Button::with_label("➕ New Project");
        toolbar.pack_start(&add_btn, false, false, 5);
        let filter_combo = gtk::ComboBoxText::new();
        filter_combo.append(Some("all"), "All Projects");
        filter_combo.append(Some("active"), "Active");
        filter_combo.append(Some("completed"), "Completed");
        filter_combo.set_active_id(Some("all"));
        toolbar.pack_end(&filter_combo, false, false, 5);
        panel.pack_start(&toolbar, false, false, 0);

        // Project cards area
        let scroll = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        scroll.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        let projects_flow = gtk::FlowBox::new();
        projects_flow.set_selection_mode(gtk::SelectionMode::None);
        projects_flow.set_max_children_per_line(3);
        scroll.add(&projects_flow);
        panel.pack_start(&scroll, true, true, 0);

        panel
    }

    fn create_payouts_tab() -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, 10);
        panel.set_margin_top(10);

        // Summary
        let summary_box = GtkBox::new(Orientation::Horizontal, 20);
        let pending_label = Label::new(Some("Pending: $4,250.00"));
        pending_label.set_widget_name("payout-pending");
        summary_box.pack_start(&pending_label, false, false, 10);
        let approved_label = Label::new(Some("Approved: $1,800.00"));
        approved_label.set_widget_name("payout-approved");
        summary_box.pack_start(&approved_label, false, false, 10);
        panel.pack_start(&summary_box, false, false, 0);

        // Payout list
        let scroll = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        let list = gtk::ListBox::new();
        scroll.add(&list);
        panel.pack_start(&scroll, true, true, 0);

        // Action buttons
        let action_box = GtkBox::new(Orientation::Horizontal, 10);
        let approve_btn = Button::with_label("✓ Approve Selected");
        action_box.pack_end(&approve_btn, false, false, 5);
        let process_btn = Button::with_label("💳 Process Payouts");
        action_box.pack_end(&process_btn, false, false, 5);
        panel.pack_end(&action_box, false, false, 10);

        panel
    }

    fn create_osint_panel() -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, 10);
        panel.set_margin_start(20);
        panel.set_margin_end(20);
        panel.set_margin_top(20);

        // Search bar
        let search_box = GtkBox::new(Orientation::Horizontal, 10);
        let search_entry = gtk::Entry::new();
        search_entry.set_placeholder_text(Some("Enter domain, IP, or search query..."));
        search_box.pack_start(&search_entry, true, true, 0);

        let options_btn = Button::with_label("⚙️");
        search_box.pack_end(&options_btn, false, false, 0);

        let search_btn = Button::with_label("🔍 Search");
        search_btn.set_widget_name("osint-search-btn");
        search_box.pack_end(&search_btn, false, false, 0);

        panel.pack_start(&search_box, false, false, 0);

        // OSINT options
        let options_box = GtkBox::new(Orientation::Horizontal, 15);
        let whois_check = gtk::CheckButton::with_label("WHOIS");
        whois_check.set_active(true);
        options_box.pack_start(&whois_check, false, false, 0);
        let ports_check = gtk::CheckButton::with_label("Port Scan");
        ports_check.set_active(true);
        options_box.pack_start(&ports_check, false, false, 0);
        let vuln_check = gtk::CheckButton::with_label("Vulnerabilities");
        vuln_check.set_active(true);
        options_box.pack_start(&vuln_check, false, false, 0);
        let exploit_check = gtk::CheckButton::with_label("Exploits");
        exploit_check.set_active(true);
        options_box.pack_start(&exploit_check, false, false, 0);
        panel.pack_start(&options_box, false, false, 0);

        // Results area
        let results_frame = Frame::new(Some("Search Results"));
        let results_scroll = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        results_scroll.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        
        // Use WebView for rich HTML results with dropdowns
        // For now, placeholder with text view
        let results_view = gtk::TextView::new();
        results_view.set_editable(false);
        results_view.set_wrap_mode(gtk::WrapMode::Word);
        results_scroll.add(&results_view);
        results_frame.add(&results_scroll);
        panel.pack_start(&results_frame, true, true, 0);

        // Quick actions
        let quick_box = GtkBox::new(Orientation::Horizontal, 10);
        let export_btn = Button::with_label("📄 Export Report");
        quick_box.pack_end(&export_btn, false, false, 5);
        let copy_btn = Button::with_label("📋 Copy Results");
        quick_box.pack_end(&copy_btn, false, false, 5);
        panel.pack_end(&quick_box, false, false, 10);

        panel
    }

    fn create_status_bar() -> GtkBox {
        let status = GtkBox::new(Orientation::Horizontal, 10);
        status.set_widget_name("status-bar");
        status.set_margin_start(10);
        status.set_margin_end(10);
        status.set_margin_top(5);
        status.set_margin_bottom(5);

        let status_label = Label::new(Some("🟢 Ready"));
        status.pack_start(&status_label, false, false, 0);

        let spacer = GtkBox::new(Orientation::Horizontal, 0);
        status.pack_start(&spacer, true, true, 0);

        let time_label = Label::new(Some(""));
        status.pack_end(&time_label, false, false, 0);

        let version_label = Label::new(Some("Marshall v2.0.0"));
        status.pack_end(&version_label, false, false, 10);

        status
    }

    pub fn get_widget(&self) -> &GtkBox {
        &self.container
    }
}

/// Dashboard CSS
pub const DASHBOARD_CSS: &str = r#"
#marshall-dashboard {
    background: linear-gradient(180deg, #0a0a14 0%, #141428 100%);
}

#dashboard-header {
    background: rgba(0, 255, 159, 0.05);
    border-bottom: 1px solid #00ff9f;
}

#header-title {
    font-family: 'Orbitron', 'JetBrains Mono', monospace;
    font-size: 24px;
    font-weight: bold;
    color: #00ff9f;
    text-shadow: 0 0 10px rgba(0, 255, 159, 0.5);
}

#header-btn {
    background: transparent;
    border: 1px solid #00ff9f;
    color: #00ff9f;
    border-radius: 4px;
    padding: 8px 15px;
}

#header-btn:hover {
    background: rgba(0, 255, 159, 0.2);
}

#dashboard-notebook {
    background: transparent;
}

#dashboard-notebook tab {
    background: #1a1a2e;
    color: #888;
    padding: 15px 20px;
    border: none;
}

#dashboard-notebook tab:checked {
    background: #252540;
    color: #00ff9f;
    border-left: 3px solid #00ff9f;
}

#status-bar {
    background: #0a0a14;
    border-top: 1px solid #333;
    color: #666;
    font-size: 12px;
}

.stat-value {
    font-size: 28px;
    font-weight: bold;
    color: #00d4ff;
}

#call-btn {
    background: #22c55e;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 10px;
}

#hangup-btn {
    background: #ef4444;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 10px;
}

#osint-search-btn {
    background: linear-gradient(135deg, #00ff9f 0%, #00d4ff 100%);
    color: #000;
    font-weight: bold;
    border: none;
    border-radius: 4px;
    padding: 10px 20px;
}

#payout-pending {
    color: #fbbf24;
    font-weight: bold;
}

#payout-approved {
    color: #22c55e;
    font-weight: bold;
}
"#;
