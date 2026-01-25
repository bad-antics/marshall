// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Main browser window implementation

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Orientation, CssProvider, StyleContext};
use webkit2gtk::{WebView, WebViewExt, WebContext, WebContextExt};
use gdk::Screen;
use tracing::info;

use crate::config::Config;
use crate::tabs::TabManager;
use super::{Toolbar, TabBar, StatusBar, Theme};

const WINDOW_WIDTH: i32 = 1400;
const WINDOW_HEIGHT: i32 = 900;

/// Main browser window
pub struct BrowserWindow;

impl BrowserWindow {
    pub fn new(app: &Application, config: &Config) -> ApplicationWindow {
        // Apply CSS theme
        Self::apply_theme(config);

        // Create main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Marshall - NullSec Browser")
            .default_width(WINDOW_WIDTH)
            .default_height(WINDOW_HEIGHT)
            .build();

        // Main vertical container
        let main_box = GtkBox::new(Orientation::Vertical, 0);
        main_box.style_context().add_class("marshall-main");

        // Create toolbar
        let toolbar = Toolbar::new(config);
        main_box.pack_start(toolbar.container(), false, false, 0);

        // Create tab bar
        let tab_bar = TabBar::new();
        main_box.pack_start(tab_bar.container(), false, false, 0);

        // Create WebKit context with privacy settings
        let web_context = Self::create_private_context(config);
        
        // Create initial web view
        let webview = Self::create_webview(&web_context, config);
        webview.set_vexpand(true);
        webview.set_hexpand(true);
        
        // Load homepage
        webview.load_uri(&config.general.homepage);
        
        main_box.pack_start(&webview, true, true, 0);

        // Create status bar
        let status_bar = StatusBar::new(config);
        main_box.pack_start(status_bar.container(), false, false, 0);

        window.add(&main_box);
        window.show_all();
        
        // Connect webview signals
        Self::connect_webview_signals(&webview, &toolbar, &status_bar);

        info!("Browser window initialized with privacy settings");
        
        window
    }

    fn apply_theme(config: &Config) {
        let css = Theme::generate_css(config);
        
        let provider = CssProvider::new();
        provider.load_from_data(css.as_bytes()).ok();

        if let Some(screen) = Screen::default() {
            StyleContext::add_provider_for_screen(
                &screen,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn create_private_context(config: &Config) -> WebContext {
        let context = WebContext::default().unwrap();

        // Configure cookie policy
        if let Some(cookie_manager) = context.cookie_manager() {
            use webkit2gtk::CookieAcceptPolicy;
            use webkit2gtk::CookieManagerExt;
            
            let policy = if config.privacy.block_third_party_cookies {
                CookieAcceptPolicy::NoThirdParty
            } else {
                CookieAcceptPolicy::Always
            };
            cookie_manager.set_accept_policy(policy);
        }

        context
    }

    fn create_webview(context: &WebContext, config: &Config) -> WebView {
        use webkit2gtk::SettingsExt;
        
        let webview = WebView::with_context(context);
        
        // Get settings
        let settings: webkit2gtk::Settings = WebViewExt::settings(&webview).unwrap();
        
        // JavaScript
        settings.set_enable_javascript(config.general.enable_javascript);
        
        // Privacy settings - disable features that can fingerprint
        if config.privacy.block_fingerprinting {
            settings.set_enable_webgl(false);
            settings.set_enable_webaudio(false);
        } else {
            settings.set_enable_webgl(config.general.enable_webgl);
        }
        
        // User agent
        if let Some(ref ua) = config.privacy.user_agent {
            settings.set_user_agent(Some(ua));
        } else if config.privacy.strict_mode {
            settings.set_user_agent(Some(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
            ));
        }
        
        // Developer features
        #[cfg(feature = "developer")]
        {
            settings.set_enable_developer_extras(true);
        }
        
        // Appearance
        settings.set_default_font_family(&config.appearance.font_family);
        settings.set_default_font_size(config.appearance.font_size);
        
        // Security
        settings.set_enable_dns_prefetching(false);
        settings.set_enable_page_cache(false);

        // Set zoom level
        webview.set_zoom_level(config.appearance.zoom_level);

        webview
    }

    fn connect_webview_signals(webview: &WebView, toolbar: &Toolbar, status_bar: &StatusBar) {
        // URL changed
        let toolbar_clone = toolbar.clone();
        webview.connect_uri_notify(move |wv| {
            if let Some(uri) = wv.uri() {
                toolbar_clone.set_url(&uri);
            }
        });

        // Loading progress
        let status_bar_clone = status_bar.clone();
        webview.connect_estimated_load_progress_notify(move |wv| {
            let progress = wv.estimated_load_progress();
            status_bar_clone.set_progress(progress);
        });

        // Load finished
        let status_bar_clone2 = status_bar.clone();
        webview.connect_load_changed(move |_wv, event| {
            use webkit2gtk::LoadEvent;
            match event {
                LoadEvent::Started => {
                    status_bar_clone2.set_status("Loading...");
                }
                LoadEvent::Committed => {
                    status_bar_clone2.set_status("Receiving data...");
                }
                LoadEvent::Finished => {
                    status_bar_clone2.set_status("Done");
                    status_bar_clone2.set_progress(0.0);
                }
                _ => {}
            }
        });
    }
}
