// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! NullSec theme for Marshall browser

use crate::config::Config;

/// Theme generator
pub struct Theme;

impl Theme {
    /// Generate CSS for the browser
    pub fn generate_css(config: &Config) -> String {
        if config.appearance.dark_mode {
            Self::dark_theme()
        } else {
            Self::light_theme()
        }
    }

    fn dark_theme() -> String {
        r#"
/* ═══════════════════════════════════════════════════════════════
   MARSHALL BROWSER - NULLSEC DARK THEME
   bad-antics | 2026
   ═══════════════════════════════════════════════════════════════ */

/* Color Variables */
@define-color bg_color #0d0d0d;
@define-color bg_secondary #1a1a1a;
@define-color bg_tertiary #252525;
@define-color fg_color #e0e0e0;
@define-color fg_secondary #808080;
@define-color accent_color #ff0040;
@define-color accent_secondary #00ff88;
@define-color border_color #333333;
@define-color success_color #00ff88;
@define-color warning_color #ffaa00;
@define-color error_color #ff4444;

/* Main Window */
window {
    background-color: @bg_color;
    color: @fg_color;
}

.marshall-main {
    background-color: @bg_color;
}

/* Toolbar */
.marshall-toolbar {
    background: linear-gradient(to bottom, @bg_secondary, @bg_color);
    border-bottom: 1px solid @border_color;
    padding: 6px 8px;
}

.toolbar-btn {
    background: transparent;
    border: none;
    border-radius: 4px;
    padding: 6px 8px;
    color: @fg_color;
    min-width: 32px;
    min-height: 32px;
}

.toolbar-btn:hover {
    background-color: @bg_tertiary;
}

.toolbar-btn:active {
    background-color: @accent_color;
}

.nav-btn {
    margin: 0 2px;
}

/* Shield Button */
.shield-btn.shield-active {
    color: @success_color;
}

.shield-btn.shield-inactive {
    color: @error_color;
}

/* URL Bar */
.marshall-urlbar {
    background-color: @bg_tertiary;
    border: 1px solid @border_color;
    border-radius: 20px;
    padding: 4px 12px;
    margin: 0 8px;
}

.marshall-urlbar:focus-within {
    border-color: @accent_color;
    box-shadow: 0 0 0 2px alpha(@accent_color, 0.2);
}

.url-entry {
    background: transparent;
    border: none;
    color: @fg_color;
    font-family: "JetBrains Mono", monospace;
    font-size: 13px;
}

.url-entry:focus {
    outline: none;
}

.security-icon {
    margin-right: 8px;
    color: @fg_secondary;
}

.security-icon.secure {
    color: @success_color;
}

.security-icon.insecure {
    color: @error_color;
}

/* Tab Bar */
.marshall-tabbar {
    background-color: @bg_secondary;
    border-bottom: 1px solid @border_color;
    padding: 4px 4px 0 4px;
}

.marshall-notebook {
    background: transparent;
}

.marshall-notebook tab {
    background-color: @bg_tertiary;
    border: 1px solid @border_color;
    border-bottom: none;
    border-radius: 8px 8px 0 0;
    padding: 6px 12px;
    margin: 0 2px;
    color: @fg_secondary;
}

.marshall-notebook tab:checked {
    background-color: @bg_color;
    color: @fg_color;
    border-bottom: 2px solid @accent_color;
}

.marshall-notebook tab:hover:not(:checked) {
    background-color: alpha(@accent_color, 0.1);
}

.marshall-tab {
    padding: 4px 8px;
}

.tab-favicon {
    margin-right: 6px;
}

.tab-label {
    font-size: 12px;
}

.tab-close-btn {
    background: transparent;
    border: none;
    border-radius: 50%;
    padding: 2px;
    margin-left: 6px;
    opacity: 0.5;
    min-width: 16px;
    min-height: 16px;
}

.tab-close-btn:hover {
    background-color: @error_color;
    opacity: 1;
}

.new-tab-btn {
    background: transparent;
    border: none;
    border-radius: 4px;
    padding: 6px;
    margin: 4px;
    color: @fg_secondary;
}

.new-tab-btn:hover {
    background-color: @bg_tertiary;
    color: @accent_color;
}

/* Status Bar */
.marshall-statusbar {
    background-color: @bg_secondary;
    border-top: 1px solid @border_color;
    padding: 4px 12px;
    font-size: 11px;
}

.status-text {
    color: @fg_secondary;
}

.status-progress {
    min-width: 100px;
    min-height: 4px;
}

.status-progress trough {
    background-color: @bg_tertiary;
    border-radius: 2px;
}

.status-progress progress {
    background-color: @accent_color;
    border-radius: 2px;
}

.privacy-indicator {
    color: @success_color;
    font-weight: bold;
    padding: 2px 8px;
    background-color: alpha(@success_color, 0.1);
    border-radius: 4px;
}

.blocked-counter {
    color: @accent_secondary;
    padding: 2px 8px;
}

/* WebView */
webview {
    background-color: @bg_color;
}

/* Scrollbars */
scrollbar {
    background-color: @bg_secondary;
}

scrollbar slider {
    background-color: @bg_tertiary;
    border-radius: 4px;
    min-width: 8px;
    min-height: 8px;
}

scrollbar slider:hover {
    background-color: @fg_secondary;
}

/* Context Menu */
menu {
    background-color: @bg_secondary;
    border: 1px solid @border_color;
    border-radius: 8px;
    padding: 4px;
}

menuitem {
    padding: 8px 12px;
    border-radius: 4px;
}

menuitem:hover {
    background-color: @accent_color;
}

/* Dialogs */
dialog {
    background-color: @bg_color;
}

dialog headerbar {
    background-color: @bg_secondary;
    border-bottom: 1px solid @border_color;
}
"#.to_string()
    }

    fn light_theme() -> String {
        r#"
/* MARSHALL BROWSER - LIGHT THEME */
/* bad-antics | 2026 */

window {
    background-color: #ffffff;
    color: #1a1a1a;
}

.marshall-toolbar {
    background-color: #f5f5f5;
    border-bottom: 1px solid #e0e0e0;
}

.marshall-urlbar {
    background-color: #ffffff;
    border: 1px solid #d0d0d0;
}

.marshall-statusbar {
    background-color: #f5f5f5;
    border-top: 1px solid #e0e0e0;
}
"#.to_string()
    }
}
