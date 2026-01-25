// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Status bar widget

use gtk::prelude::*;
use gtk::{Box as GtkBox, Label, ProgressBar, Orientation};

use crate::config::Config;

/// Status bar at bottom of window
#[derive(Clone)]
pub struct StatusBar {
    container: GtkBox,
    status_label: Label,
    progress_bar: ProgressBar,
    privacy_label: Label,
    blocked_label: Label,
}

impl StatusBar {
    pub fn new(config: &Config) -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 8);
        container.style_context().add_class("marshall-statusbar");
        container.set_margin_start(8);
        container.set_margin_end(8);
        container.set_margin_top(2);
        container.set_margin_bottom(2);

        // Status text
        let status_label = Label::new(Some("Ready"));
        status_label.style_context().add_class("status-text");
        status_label.set_hexpand(true);
        status_label.set_halign(gtk::Align::Start);

        // Progress bar (hidden by default)
        let progress_bar = ProgressBar::new();
        progress_bar.style_context().add_class("status-progress");
        progress_bar.set_visible(false);

        // Privacy indicator
        let privacy_mode = if config.privacy.strict_mode { "STRICT" } else { "Standard" };
        let privacy_label = Label::new(Some(&format!("🔒 {}", privacy_mode)));
        privacy_label.style_context().add_class("privacy-indicator");

        // Blocked count
        let blocked_label = Label::new(Some("🛡️ 0 blocked"));
        blocked_label.style_context().add_class("blocked-counter");

        container.pack_start(&status_label, true, true, 0);
        container.pack_start(&progress_bar, false, false, 4);
        container.pack_start(&blocked_label, false, false, 4);
        container.pack_start(&privacy_label, false, false, 4);

        Self {
            container,
            status_label,
            progress_bar,
            privacy_label,
            blocked_label,
        }
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    pub fn set_status(&self, status: &str) {
        self.status_label.set_text(status);
    }

    pub fn set_progress(&self, progress: f64) {
        if progress > 0.0 && progress < 1.0 {
            self.progress_bar.set_visible(true);
            self.progress_bar.set_fraction(progress);
        } else {
            self.progress_bar.set_visible(false);
        }
    }

    pub fn set_blocked_count(&self, count: u64) {
        self.blocked_label.set_text(&format!("🛡️ {} blocked", count));
    }

    pub fn set_privacy_mode(&self, strict: bool) {
        let mode = if strict { "STRICT" } else { "Standard" };
        self.privacy_label.set_text(&format!("🔒 {}", mode));
    }
}
