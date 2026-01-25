// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! URL/Address bar widget

use gtk::prelude::*;
use gtk::{Box as GtkBox, Entry, Image, Orientation};

/// URL bar with security indicator
#[derive(Clone)]
pub struct UrlBar {
    container: GtkBox,
    entry: Entry,
    security_icon: Image,
}

impl UrlBar {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 4);
        container.style_context().add_class("marshall-urlbar");
        container.set_hexpand(true);

        // Security indicator
        let security_icon = Image::from_icon_name(Some("channel-insecure-symbolic"), gtk::IconSize::Button);
        security_icon.style_context().add_class("security-icon");

        // URL entry
        let entry = Entry::new();
        entry.set_placeholder_text(Some("Search or enter URL"));
        entry.set_hexpand(true);
        entry.style_context().add_class("url-entry");

        container.pack_start(&security_icon, false, false, 4);
        container.pack_start(&entry, true, true, 0);

        Self {
            container,
            entry,
            security_icon,
        }
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    pub fn set_url(&self, url: &str) {
        self.entry.set_text(url);
        self.update_security_indicator(url);
    }

    pub fn get_url(&self) -> String {
        self.entry.text().to_string()
    }

    fn update_security_indicator(&self, url: &str) {
        let ctx = self.security_icon.style_context();
        if url.starts_with("https://") {
            self.security_icon.set_from_icon_name(Some("channel-secure-symbolic"), gtk::IconSize::Button);
            ctx.remove_class("insecure");
            ctx.add_class("secure");
        } else if url.starts_with("http://") {
            self.security_icon.set_from_icon_name(Some("channel-insecure-symbolic"), gtk::IconSize::Button);
            ctx.remove_class("secure");
            ctx.add_class("insecure");
        } else {
            self.security_icon.set_from_icon_name(Some("applications-internet-symbolic"), gtk::IconSize::Button);
            ctx.remove_class("secure");
            ctx.remove_class("insecure");
        }
    }

    pub fn focus(&self) {
        self.entry.grab_focus();
    }

    pub fn select_all(&self) {
        self.entry.select_region(0, -1);
    }
}

impl Default for UrlBar {
    fn default() -> Self {
        Self::new()
    }
}
