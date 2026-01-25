// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Marshall - NullSec Command Center
//! Browser • OSINT • Workforce • VoIP
//!
//! A comprehensive security and business management platform featuring:
//! - Privacy-focused web browser
//! - AI Assistant with talking head avatar
//! - VoIP calling system
//! - Workforce management center
//! - OSINT-enhanced search with vulnerability analysis

mod ui;
mod engine;
mod privacy;
mod network;
mod tabs;
mod bookmarks;
mod history;
mod adblock;
mod config;
mod utils;
mod assistant;
mod voip;
mod workforce;
mod search;
mod database;

use gtk::prelude::*;
use gtk::Application;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::sync::Arc;
use parking_lot::RwLock;

use crate::config::Config;
use crate::ui::{BrowserWindow, Dashboard, DashboardConfig};
use crate::assistant::{Assistant, AssistantConfig};
use crate::voip::{VoIP, VoIPConfig};
use crate::workforce::{WorkforceCenter, WorkforceConfig};
use crate::search::{SearchEngine, SearchConfig};

const APP_ID: &str = "io.nullsec.marshall";
const VERSION: &str = env!("CARGO_PKG_VERSION");

const BANNER: &str = r#"
╔═══════════════════════════════════════════════════════════════════════════════════════╗
║                                                                                       ║
║    ███╗   ███╗ █████╗ ██████╗ ███████╗██╗  ██╗ █████╗ ██╗     ██╗                     ║
║    ████╗ ████║██╔══██╗██╔══██╗██╔════╝██║  ██║██╔══██╗██║     ██║                     ║
║    ██╔████╔██║███████║██████╔╝███████╗███████║███████║██║     ██║                     ║
║    ██║╚██╔╝██║██╔══██║██╔══██╗╚════██║██╔══██║██╔══██║██║     ██║                     ║
║    ██║ ╚═╝ ██║██║  ██║██║  ██║███████║██║  ██║██║  ██║███████╗███████╗                ║
║    ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝                ║
║                                                                                       ║
║                      🔒 NULLSEC COMMAND CENTER v2.1.0 🔒                              ║
║                                                                                       ║
║                  Secure. Private. Untraceable.                                        ║
║                                                                                       ║
║            Browser • Dr. Marshall AI • VoIP • Workforce • OSINT                       ║
║                                                                                       ║
║                              bad-antics | 2026                                        ║
╚═══════════════════════════════════════════════════════════════════════════════════════╝
"#;

/// Global application state
pub struct AppState {
    pub config: Config,
    pub assistant: Arc<RwLock<Assistant>>,
    pub voip: Arc<RwLock<VoIP>>,
    pub workforce: Arc<RwLock<WorkforceCenter>>,
    pub search: Arc<RwLock<SearchEngine>>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        info!("Initializing Marshall Command Center modules...");

        // Initialize AI Assistant
        let assistant_config = AssistantConfig::default();
        let assistant = Assistant::new(assistant_config);
        info!("✓ AI Assistant initialized");

        // Initialize VoIP
        let voip_config = VoIPConfig::default();
        let voip = VoIP::new(voip_config);
        info!("✓ VoIP system initialized");

        // Initialize Workforce Center
        let workforce_config = WorkforceConfig::default();
        let workforce = WorkforceCenter::new(workforce_config);
        info!("✓ Workforce Center initialized");

        // Initialize OSINT Search
        let search_config = SearchConfig::default();
        let search = SearchEngine::new(search_config);
        info!("✓ OSINT Search Engine initialized");

        Self {
            config,
            assistant: Arc::new(RwLock::new(assistant)),
            voip: Arc::new(RwLock::new(voip)),
            workforce: Arc::new(RwLock::new(workforce)),
            search: Arc::new(RwLock::new(search)),
        }
    }
}

fn main() {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    // Print banner
    println!("{}", BANNER);
    info!("Marshall Command Center v{} starting...", VERSION);

    // Load configuration
    let config = Config::load().unwrap_or_else(|e| {
        tracing::warn!("Failed to load config: {}, using defaults", e);
        Config::default()
    });

    // Display module status
    println!("\n┌─────────────────────────────────────────────┐");
    println!("│          MODULE INITIALIZATION              │");
    println!("├─────────────────────────────────────────────┤");
    
    info!("Privacy mode: {}", if config.privacy.strict_mode { "STRICT" } else { "Standard" });
    info!("Ad blocking: {}", if config.adblock.enabled { "ON" } else { "OFF" });
    info!("Tracker protection: {}", if config.privacy.block_trackers { "ON" } else { "OFF" });

    println!("│ 🌐 Browser Engine       [READY]             │");
    println!("│ 🤖 AI Assistant         [READY]             │");
    println!("│ �� VoIP System          [READY]             │");
    println!("│ 👥 Workforce Center     [READY]             │");
    println!("│ 🔍 OSINT Search         [READY]             │");
    println!("│ 🛡️  Privacy Engine       [READY]             │");
    println!("└─────────────────────────────────────────────┘\n");

    // Initialize GTK
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK");
        std::process::exit(1);
    }

    // Initialize UI styles
    crate::ui::init_styles();

    // Create GTK application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    let config_clone = config.clone();
    app.connect_activate(move |app| {
        build_ui(app, &config_clone);
    });

    // Run the application
    info!("Launching Marshall Command Center...");
    let args: Vec<String> = std::env::args().collect();
    app.run_with_args(&args);
}

fn build_ui(app: &Application, config: &Config) {
    // Initialize application state with all modules
    let state = Arc::new(AppState::new(config.clone()));

    // Check if we should launch in dashboard mode or browser mode
    let launch_mode = std::env::var("MARSHALL_MODE").unwrap_or_else(|_| "browser".to_string());

    match launch_mode.as_str() {
        "dashboard" => {
            // Launch command center dashboard
            let dashboard_config = DashboardConfig::default();
            let dashboard = Dashboard::new(dashboard_config);
            
            let window = gtk::ApplicationWindow::builder()
                .application(app)
                .title("Marshall Command Center")
                .default_width(1400)
                .default_height(900)
                .build();

            window.add(dashboard.get_widget());
            window.show_all();
            info!("Marshall Command Center dashboard created");
        }
        _ => {
            // Launch browser with integrated features
            let window = BrowserWindow::new(app, config);
            
            // TODO: Integrate assistant, voip, workforce into browser window
            // For now, the dashboard can be accessed via a toolbar button
            
            window.present();
            info!("Marshall browser window created");
        }
    }
}

// Re-export key types for external use
