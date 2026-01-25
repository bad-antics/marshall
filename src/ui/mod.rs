// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Marshall UI Module
//! GTK3 user interface components

mod window;
mod toolbar;
mod tabbar;
mod urlbar;
mod statusbar;
mod theme;
mod dialogs;
pub mod dashboard;
pub mod search_results;

pub use window::BrowserWindow;
pub use toolbar::Toolbar;
pub use tabbar::TabBar;
pub use urlbar::UrlBar;
pub use statusbar::StatusBar;
pub use theme::Theme;
pub use dashboard::*;
pub use search_results::*;

use gtk::prelude::*;
use gtk::CssProvider;

/// Initialize all UI styles
pub fn init_styles() {
    let provider = CssProvider::new();
    let css = format!(
        "{}\n{}\n{}",
        DASHBOARD_CSS,
        SEARCH_RESULTS_CSS,
        GLOBAL_CSS
    );
    provider.load_from_data(css.as_bytes()).expect("Failed to load CSS");
    
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Failed to get default screen"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Global CSS styles
pub const GLOBAL_CSS: &str = r#"
/* Marshall Global Styles */

* {
    font-family: 'Inter', 'Segoe UI', system-ui, sans-serif;
}

window {
    background: #0a0a14;
}

entry {
    background: #1a1a2e;
    border: 1px solid #333;
    border-radius: 4px;
    color: #fff;
    padding: 8px 12px;
    caret-color: #00ff9f;
}

entry:focus {
    border-color: #00ff9f;
}

button {
    background: linear-gradient(135deg, #1a1a2e 0%, #252540 100%);
    border: 1px solid #444;
    border-radius: 4px;
    color: #fff;
    padding: 8px 16px;
}

button:hover {
    background: #2d2d4a;
    border-color: #00ff9f;
}

button:active {
    background: #00ff9f;
    color: #000;
}

label {
    color: #ccc;
}

frame {
    background: rgba(26, 26, 46, 0.5);
    border: 1px solid #333;
    border-radius: 6px;
}

frame > label {
    color: #00ff9f;
    font-weight: bold;
    padding: 0 5px;
}
"#;

pub mod homepage;
pub use homepage::*;
