// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Browser UI components

mod window;
mod toolbar;
mod urlbar;
mod tabbar;
mod statusbar;
mod dialogs;
mod theme;

pub use window::BrowserWindow;
pub use toolbar::Toolbar;
pub use urlbar::UrlBar;
pub use tabbar::TabBar;
pub use statusbar::StatusBar;
pub use theme::Theme;
