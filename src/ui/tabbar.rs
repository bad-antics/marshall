// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Tab bar widget

use gtk::prelude::*;
use gtk::{Box as GtkBox, Button, Label, Notebook, Orientation, Image};

/// Tab bar with tabs
#[derive(Clone)]
pub struct TabBar {
    container: GtkBox,
    notebook: Notebook,
    new_tab_btn: Button,
}

impl TabBar {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 0);
        container.style_context().add_class("marshall-tabbar");

        let notebook = Notebook::new();
        notebook.set_scrollable(true);
        notebook.set_show_border(false);
        notebook.style_context().add_class("marshall-notebook");
        notebook.set_hexpand(true);

        let new_tab_btn = Button::from_icon_name(Some("list-add-symbolic"), gtk::IconSize::Button);
        new_tab_btn.set_tooltip_text(Some("New Tab (Ctrl+T)"));
        new_tab_btn.style_context().add_class("new-tab-btn");

        container.pack_start(&notebook, true, true, 0);
        container.pack_start(&new_tab_btn, false, false, 4);

        Self {
            container,
            notebook,
            new_tab_btn,
        }
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    pub fn notebook(&self) -> &Notebook {
        &self.notebook
    }
}

impl Default for TabBar {
    fn default() -> Self {
        Self::new()
    }
}

/// Individual tab widget
pub struct Tab {
    container: GtkBox,
    label: Label,
    close_btn: Button,
    favicon: Image,
}

impl Tab {
    pub fn new(title: &str) -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 4);
        container.style_context().add_class("marshall-tab");

        let favicon = Image::from_icon_name(Some("applications-internet-symbolic"), gtk::IconSize::Menu);
        favicon.style_context().add_class("tab-favicon");

        let label = Label::new(Some(title));
        label.style_context().add_class("tab-label");
        label.set_ellipsize(gtk::pango::EllipsizeMode::End);
        label.set_max_width_chars(20);

        let close_btn = Button::from_icon_name(Some("window-close-symbolic"), gtk::IconSize::Menu);
        close_btn.style_context().add_class("tab-close-btn");

        container.pack_start(&favicon, false, false, 0);
        container.pack_start(&label, true, true, 0);
        container.pack_start(&close_btn, false, false, 0);

        Self {
            container,
            label,
            close_btn,
            favicon,
        }
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    pub fn set_title(&self, title: &str) {
        self.label.set_text(title);
    }
}
