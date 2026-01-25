// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Marshall - NullSec Privacy Browser
//! Secure. Private. Untraceable.
//!
//! A privacy-focused web browser built for security researchers and
//! privacy-conscious users. Features built-in ad blocking, tracker
//! protection, and optional Tor integration.

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

use gtk::prelude::*;
use gtk::Application;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::config::Config;
use crate::ui::BrowserWindow;

const APP_ID: &str = "io.nullsec.marshall";
const VERSION: &str = env!("CARGO_PKG_VERSION");

const BANNER: &str = r#"
╔══════════════════════════════════════════════════════════════════╗
║                                                                  ║
║    ███╗   ███╗ █████╗ ██████╗ ███████╗██╗  ██╗ █████╗ ██╗     ██╗║
║    ████╗ ████║██╔══██╗██╔══██╗██╔════╝██║  ██║██╔══██╗██║     ██║║
║    ██╔████╔██║███████║██████╔╝███████╗███████║███████║██║     ██║║
║    ██║╚██╔╝██║██╔══██║██╔══██╗╚════██║██╔══██║██╔══██║██║     ██║║
║    ██║ ╚═╝ ██║██║  ██║██║  ██║███████║██║  ██║██║  ██║███████╗███████╗
║    ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝
║                                                                  ║
║              🔒 NULLSEC PRIVACY BROWSER v1.0.0 🔒                ║
║                  Secure. Private. Untraceable.                   ║
║                                                                  ║
║                      bad-antics | 2026                           ║
╚══════════════════════════════════════════════════════════════════╝
"#;

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
    info!("Marshall Browser v{} starting...", VERSION);

    // Load configuration
    let config = Config::load().unwrap_or_else(|e| {
        tracing::warn!("Failed to load config: {}, using defaults", e);
        Config::default()
    });

    info!("Privacy mode: {}", if config.privacy.strict_mode { "STRICT" } else { "Standard" });
    info!("Ad blocking: {}", if config.adblock.enabled { "ON" } else { "OFF" });
    info!("Tracker protection: {}", if config.privacy.block_trackers { "ON" } else { "OFF" });

    // Create GTK application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    let config_clone = config.clone();
    app.connect_activate(move |app| {
        build_ui(app, &config_clone);
    });

    // Run the application
    let args: Vec<String> = std::env::args().collect();
    app.run_with_args(&args);
}

fn build_ui(app: &Application, config: &Config) {
    let window = BrowserWindow::new(app, config);
    window.present();
    info!("Marshall browser window created");
}
