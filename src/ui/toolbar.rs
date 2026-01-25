// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Navigation toolbar

use gtk::prelude::*;
use gtk::{Box as GtkBox, Button, Orientation};

use crate::config::Config;
use super::UrlBar;

/// Navigation toolbar
#[derive(Clone)]
pub struct Toolbar {
    container: GtkBox,
    back_btn: Button,
    forward_btn: Button,
    reload_btn: Button,
    home_btn: Button,
    url_bar: UrlBar,
    menu_btn: Button,
    shield_btn: Button,
}

impl Toolbar {
    pub fn new(_config: &Config) -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 4);
        container.style_context().add_class("marshall-toolbar");
        container.set_margin_start(8);
        container.set_margin_end(8);
        container.set_margin_top(4);
        container.set_margin_bottom(4);

        // Navigation buttons
        let back_btn = Button::from_icon_name(Some("go-previous-symbolic"), gtk::IconSize::Button);
        back_btn.set_tooltip_text(Some("Back (Alt+Left)"));
        back_btn.style_context().add_class("toolbar-btn");

        let forward_btn = Button::from_icon_name(Some("go-next-symbolic"), gtk::IconSize::Button);
        forward_btn.set_tooltip_text(Some("Forward (Alt+Right)"));
        forward_btn.style_context().add_class("toolbar-btn");

        let reload_btn = Button::from_icon_name(Some("view-refresh-symbolic"), gtk::IconSize::Button);
        reload_btn.set_tooltip_text(Some("Reload (Ctrl+R)"));
        reload_btn.style_context().add_class("toolbar-btn");

        let home_btn = Button::from_icon_name(Some("go-home-symbolic"), gtk::IconSize::Button);
        home_btn.set_tooltip_text(Some("Home"));
        home_btn.style_context().add_class("toolbar-btn");

        // URL bar
        let url_bar = UrlBar::new();
        
        // Shield button (privacy indicator)
        let shield_btn = Button::from_icon_name(Some("security-high-symbolic"), gtk::IconSize::Button);
        shield_btn.set_tooltip_text(Some("Privacy Protection: ON"));
        shield_btn.style_context().add_class("shield-btn");
        shield_btn.style_context().add_class("shield-active");

        // Menu button
        let menu_btn = Button::from_icon_name(Some("open-menu-symbolic"), gtk::IconSize::Button);
        menu_btn.set_tooltip_text(Some("Menu"));
        menu_btn.style_context().add_class("menu-btn");

        // Pack widgets
        container.pack_start(&back_btn, false, false, 2);
        container.pack_start(&forward_btn, false, false, 2);
        container.pack_start(&reload_btn, false, false, 2);
        container.pack_start(&home_btn, false, false, 2);
        container.pack_start(url_bar.container(), true, true, 8);
        container.pack_start(&shield_btn, false, false, 2);
        container.pack_start(&menu_btn, false, false, 2);

        Self {
            container,
            back_btn,
            forward_btn,
            reload_btn,
            home_btn,
            url_bar,
            menu_btn,
            shield_btn,
        }
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    pub fn set_url(&self, url: &str) {
        self.url_bar.set_url(url);
    }

    pub fn get_url(&self) -> String {
        self.url_bar.get_url()
    }

    pub fn set_can_go_back(&self, can: bool) {
        self.back_btn.set_sensitive(can);
    }

    pub fn set_can_go_forward(&self, can: bool) {
        self.forward_btn.set_sensitive(can);
    }

    pub fn set_shield_active(&self, active: bool) {
        let ctx = self.shield_btn.style_context();
        if active {
            ctx.add_class("shield-active");
            ctx.remove_class("shield-inactive");
            self.shield_btn.set_tooltip_text(Some("Privacy Protection: ON"));
        } else {
            ctx.remove_class("shield-active");
            ctx.add_class("shield-inactive");
            self.shield_btn.set_tooltip_text(Some("Privacy Protection: OFF"));
        }
    }
}
