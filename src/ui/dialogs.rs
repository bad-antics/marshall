// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Dialogs (settings, about, etc.)

use gtk::prelude::*;
use gtk::{Dialog, Label, Box as GtkBox, Orientation, Window, ResponseType, ButtonsType, MessageType};

/// About dialog
pub fn show_about_dialog<W: IsA<Window>>(parent: &W) {
    let dialog = gtk::AboutDialog::builder()
        .transient_for(parent)
        .modal(true)
        .program_name("Marshall")
        .version(env!("CARGO_PKG_VERSION"))
        .comments("NullSec Privacy Browser\nSecure. Private. Untraceable.")
        .website("https://github.com/bad-antics/marshall")
        .website_label("GitHub Repository")
        .authors(vec!["bad-antics".to_string()])
        .license_type(gtk::License::MitX11)
        .logo_icon_name("web-browser")
        .build();

    dialog.run();
    dialog.close();
}

/// Settings dialog stub
pub fn show_settings_dialog<W: IsA<Window>>(parent: &W) {
    let dialog = gtk::MessageDialog::new(
        Some(parent),
        gtk::DialogFlags::MODAL,
        MessageType::Info,
        ButtonsType::Ok,
        "Settings coming soon...",
    );
    dialog.set_title("Marshall Settings");
    dialog.run();
    dialog.close();
}
