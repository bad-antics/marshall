// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! URL/Address bar widget with Marshall branding

use gtk::prelude::*;
use gtk::{Box as GtkBox, Entry, Image, Orientation};
use std::cell::RefCell;
use std::rc::Rc;
use glib::clone;
use std::time::{SystemTime, UNIX_EPOCH};

/// URL bar with security indicator - shows "Marshall" by default, real URL on focus
#[derive(Clone)]
pub struct UrlBar {
    container: GtkBox,
    entry: Entry,
    security_icon: Image,
    real_url: Rc<RefCell<String>>,
    is_focused: Rc<RefCell<bool>>,
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
        
        // Show Marshall branding by default
        entry.set_text("🔒 Marshall - Private Browsing");

        container.pack_start(&security_icon, false, false, 4);
        container.pack_start(&entry, true, true, 0);
        
        let real_url = Rc::new(RefCell::new(String::from("marshall://home")));
        let is_focused = Rc::new(RefCell::new(false));
        
        // Show real URL when focused, Marshall branding when not
        let real_url_clone = real_url.clone();
        let is_focused_clone = is_focused.clone();
        entry.connect_focus_in_event(clone!(@weak entry => @default-return gtk::Inhibit(false), move |_, _| {
            *is_focused_clone.borrow_mut() = true;
            let url = real_url_clone.borrow().clone();
            entry.set_text(&url);
            entry.select_region(0, -1);
            gtk::Inhibit(false)
        }));
        
        let real_url_clone = real_url.clone();
        let is_focused_clone = is_focused.clone();
        entry.connect_focus_out_event(clone!(@weak entry => @default-return gtk::Inhibit(false), move |_, _| {
            *is_focused_clone.borrow_mut() = false;
            // Show Marshall branding unless it's an internal marshall:// URL
            let url = real_url_clone.borrow();
            if url.starts_with("marshall://") {
                entry.set_text(&format!("🔒 Marshall - {}", url.replace("marshall://", "")));
            } else {
                entry.set_text("🔒 Marshall - Private Browsing");
            }
            gtk::Inhibit(false)
        }));
        
        // Intercept copy to scramble URLs
        let real_url_clone = real_url.clone();
        entry.connect_copy_clipboard(move |e| {
            let url = real_url_clone.borrow().clone();
            let scrambled = Self::scramble_url(&url);
            let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
            clipboard.set_text(&scrambled);
            e.stop_signal_emission_by_name("copy-clipboard");
        });

        Self {
            container,
            entry,
            security_icon,
            real_url,
            is_focused,
        }
    }
    
    /// Scramble URL for privacy when copying
    fn scramble_url(_url: &str) -> String {
        // Generate a pseudo-random ID based on time
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        
        let id = format!("{:x}", now);
        format!("marshall://link/{}", &id[..16.min(id.len())])
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    pub fn set_url(&self, url: &str) {
        // Store the real URL
        *self.real_url.borrow_mut() = url.to_string();
        
        // Only show real URL if focused, otherwise show Marshall branding
        if *self.is_focused.borrow() {
            self.entry.set_text(url);
        } else if url.starts_with("marshall://") {
            self.entry.set_text(&format!("🔒 Marshall - {}", url.replace("marshall://", "")));
        } else {
            self.entry.set_text("🔒 Marshall - Private Browsing");
        }
        self.update_security_indicator(url);
    }

    pub fn get_url(&self) -> String {
        // Return real URL for navigation, not display text
        self.real_url.borrow().clone()
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

    /// Connect to the activate signal (Enter pressed)
    pub fn connect_activate<F: Fn(&str) + 'static>(&self, f: F) {
        let entry = self.entry.clone();
        self.entry.connect_activate(move |_| {
            let text = entry.text();
            f(&text);
        });
    }
}

impl Default for UrlBar {
    fn default() -> Self {
        Self::new()
    }
}
