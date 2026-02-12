// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Main browser window implementation

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Orientation, CssProvider, StyleContext, Paned};
use webkit2gtk::{WebView, WebViewExt, WebContext, WebContextExt, LoadEvent, PolicyDecisionType, NavigationPolicyDecision, PolicyDecisionExt, NavigationPolicyDecisionExt, URIRequestExt};
use gdk::Screen;
use tracing::info;
use std::cell::RefCell;
use std::rc::Rc;

use crate::config::Config;
use crate::assistant::ai_chat::AIChatPanel;
use super::{Toolbar, TabBar, StatusBar, Theme, homepage};

const WINDOW_WIDTH: i32 = 1400;
const WINDOW_HEIGHT: i32 = 900;

/// Internal navigation history for marshall:// pages
/// WebKit's load_html doesn't create history entries, so we track them ourselves
#[derive(Clone)]
struct InternalHistory {
    entries: Vec<String>,
    current: i32,  // -1 means no internal history
}

impl InternalHistory {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            current: -1,
        }
    }
    
    fn push(&mut self, uri: &str) {
        // If we're not at the end, truncate forward history
        if self.current >= 0 && (self.current as usize) < self.entries.len() - 1 {
            self.entries.truncate((self.current + 1) as usize);
        }
        
        // Don't add duplicate consecutive entries
        if self.entries.last().map(|s| s.as_str()) != Some(uri) {
            self.entries.push(uri.to_string());
        }
        self.current = (self.entries.len() as i32) - 1;
    }
    
    fn can_go_back(&self) -> bool {
        self.current > 0
    }
    
    fn can_go_forward(&self) -> bool {
        self.current >= 0 && (self.current as usize) < self.entries.len() - 1
    }
    
    fn go_back(&mut self) -> Option<String> {
        if self.can_go_back() {
            self.current -= 1;
            Some(self.entries[self.current as usize].clone())
        } else {
            None
        }
    }
    
    fn go_forward(&mut self) -> Option<String> {
        if self.can_go_forward() {
            self.current += 1;
            Some(self.entries[self.current as usize].clone())
        } else {
            None
        }
    }
}

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

        // Create Dr. Marshall AI sidebar in a horizontal pane
        let paned = Paned::new(Orientation::Horizontal);
        paned.set_wide_handle(true);
        paned.pack1(&webview, true, false);

        // Build AI Chat Panel and wrap in a frame
        let ai_panel = AIChatPanel::new();
        let ai_sidebar = ai_panel.widget().clone();
        ai_sidebar.set_width_request(380);
        ai_sidebar.set_no_show_all(true); // hidden by default
        ai_sidebar.set_visible(false);
        paned.pack2(&ai_sidebar, false, false);

        main_box.pack_start(&paned, true, true, 0);

        // Create status bar
        let status_bar = StatusBar::new(config);
        main_box.pack_start(status_bar.container(), false, false, 0);

        window.add(&main_box);
        
        // Create internal history tracker for marshall:// pages
        let history = Rc::new(RefCell::new(InternalHistory::new()));
        
        // Connect all signals BEFORE loading homepage
        Self::connect_toolbar_signals(&toolbar, &webview, config, history.clone());
        Self::connect_webview_signals(&webview, &toolbar, &status_bar, history.clone());

        // Connect AI sidebar toggle
        let ai_sidebar_toggle = ai_sidebar.clone();
        toolbar.connect_ai(move || {
            let visible = ai_sidebar_toggle.is_visible();
            ai_sidebar_toggle.set_visible(!visible);
            if !visible {
                ai_sidebar_toggle.show_all();
            }
        });
        
        // Now load Marshall branded homepage (history is added in handle_internal_url)
        Self::handle_internal_url(&webview, "marshall://home", &history, &toolbar);
        
        window.show_all();

        info!("Browser window initialized with Marshall homepage and OSINT injection");
        
        window
    }

    /// Load the Marshall branded homepage (legacy, kept for reference)
    #[allow(dead_code)]
    fn load_marshall_home(webview: &WebView) {
        let homepage_html = homepage::generate_homepage();
        // Use None as base URI to avoid triggering decide_policy infinite loops
        webview.load_html(&homepage_html, None);
    }
    
    /// Handle internal marshall:// URLs with history tracking
    fn handle_internal_url(webview: &WebView, uri: &str, history: &Rc<RefCell<InternalHistory>>, toolbar: &Toolbar) {
        // Add to internal history
        history.borrow_mut().push(uri);
        
        // Update toolbar back/forward buttons
        let h = history.borrow();
        toolbar.set_can_go_back(h.can_go_back());
        toolbar.set_can_go_forward(h.can_go_forward());
        drop(h);
        
        // Load the appropriate page
        Self::load_internal_page(webview, uri);
    }
    
    /// Load internal page content without history tracking (for back/forward navigation)
    fn load_internal_page(webview: &WebView, uri: &str) {
        // Use None as base URI to avoid triggering decide_policy
        match uri {
            "marshall://home" | "marshall://home/" => {
                let html = homepage::generate_homepage();
                webview.load_html(&html, None);
            }
            "marshall://menu" | "marshall://menu/" => {
                let html = homepage::generate_menu_page();
                webview.load_html(&html, None);
            }
            "marshall://settings" | "marshall://settings/" => {
                let html = homepage::generate_settings_page();
                webview.load_html(&html, None);
            }
            "marshall://privacy" | "marshall://privacy/" => {
                let html = homepage::generate_privacy_page();
                webview.load_html(&html, None);
            }
            "marshall://assistant" | "marshall://assistant/" => {
                let html = homepage::generate_assistant_page();
                webview.load_html(&html, None);
            }
            "marshall://workforce" | "marshall://workforce/" => {
                let html = homepage::generate_workforce_page();
                webview.load_html(&html, None);
            }
            "marshall://voip" | "marshall://voip/" => {
                let html = homepage::generate_voip_page();
                webview.load_html(&html, None);
            }
            "marshall://osint" | "marshall://osint/" => {
                let html = homepage::generate_osint_page();
                webview.load_html(&html, None);
            }
            _ if uri.starts_with("marshall://osint/") => {
                let domain = uri.strip_prefix("marshall://osint/").unwrap_or("").trim_end_matches('/');
                if !domain.is_empty() {
                    let decoded = urlencoding::decode(domain).unwrap_or_default();
                    let html = homepage::generate_osint_results(&decoded);
                    webview.load_html(&html, None);
                } else {
                    let html = homepage::generate_osint_page();
                    webview.load_html(&html, None);
                }
            }
            _ if uri.starts_with("marshall://link/") => {
                let html = homepage::generate_homepage();
                webview.load_html(&html, None);
            }
            _ => {
                let html = homepage::generate_homepage();
                webview.load_html(&html, None);
            }
        }
    }

    /// Inject Marshall userscript into the page
    fn inject_marshall_script(webview: &WebView) {
        let userscript = homepage::generate_userscript();
        webview.run_javascript(&userscript, None::<&gio::Cancellable>, |_result| {
            // Script injection complete
        });
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
        
        // JavaScript - MUST be enabled for userscript injection
        settings.set_enable_javascript(true);
        
        // Privacy settings - disable features that can fingerprint
        if config.privacy.block_fingerprinting {
            settings.set_enable_webgl(false);
            settings.set_enable_webaudio(false);
        } else {
            settings.set_enable_webgl(config.general.enable_webgl);
        }
        
        // User agent - use Marshall branded user agent
        settings.set_user_agent(Some(
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Marshall/2.0 Chrome/120.0.0.0 Safari/537.36"
        ));
        
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

    fn connect_toolbar_signals(toolbar: &Toolbar, webview: &WebView, _config: &Config, history: Rc<RefCell<InternalHistory>>) {
        // Back button - handle both internal and external navigation
        let wv = webview.clone();
        let history_back = history.clone();
        let toolbar_back = toolbar.clone();
        toolbar.connect_back(move || {
            // First try internal history (for marshall:// pages)
            let uri = history_back.borrow_mut().go_back();
            if let Some(uri) = uri {
                // Update buttons
                let h = history_back.borrow();
                toolbar_back.set_can_go_back(h.can_go_back());
                toolbar_back.set_can_go_forward(h.can_go_forward());
                drop(h);
                // Load page without adding to history
                Self::load_internal_page(&wv, &uri);
            } else {
                // Try WebKit's history (for external pages)
                wv.go_back();
            }
        });

        // Forward button - handle both internal and external navigation
        let wv = webview.clone();
        let history_fwd = history.clone();
        let toolbar_fwd = toolbar.clone();
        toolbar.connect_forward(move || {
            // First try internal history
            let uri = history_fwd.borrow_mut().go_forward();
            if let Some(uri) = uri {
                // Update buttons
                let h = history_fwd.borrow();
                toolbar_fwd.set_can_go_back(h.can_go_back());
                toolbar_fwd.set_can_go_forward(h.can_go_forward());
                drop(h);
                // Load page without adding to history
                Self::load_internal_page(&wv, &uri);
            } else {
                // Try WebKit's history
                wv.go_forward();
            }
        });

        // Reload button
        let wv = webview.clone();
        toolbar.connect_reload(move || {
            wv.reload();
        });

        // Home button - load Marshall homepage with history
        let wv = webview.clone();
        let history_home = history.clone();
        let toolbar_home = toolbar.clone();
        toolbar.connect_home(move || {
            Self::handle_internal_url(&wv, "marshall://home", &history_home, &toolbar_home);
        });

        // Menu button - show menu page with history
        let wv_menu = webview.clone();
        let history_menu = history.clone();
        let toolbar_menu = toolbar.clone();
        toolbar.connect_menu(move || {
            Self::handle_internal_url(&wv_menu, "marshall://menu", &history_menu, &toolbar_menu);
        });

        // URL bar - navigate on enter
        let wv = webview.clone();
        let history_nav = history.clone();
        let toolbar_nav = toolbar.clone();
        toolbar.connect_navigate(move |url| {
            // Check for marshall:// internal URLs
            if url.starts_with("marshall://") || url == "marshall:home" || url.is_empty() {
                if url.is_empty() || url == "marshall:home" {
                    Self::handle_internal_url(&wv, "marshall://home", &history_nav, &toolbar_nav);
                } else {
                    Self::handle_internal_url(&wv, url, &history_nav, &toolbar_nav);
                }
                return;
            }
            
            let url = if url.starts_with("http://") || url.starts_with("https://") {
                url.to_string()
            } else if url.contains('.') && !url.contains(' ') {
                format!("https://{}", url)
            } else {
                // Search using DuckDuckGo with dark mode params
                format!("https://duckduckgo.com/?q={}&kae=d&k1=-1&kaj=m&kam=osm&kp=-2", urlencoding::encode(url))
            };
            wv.load_uri(&url);
        });
    }

    fn connect_webview_signals(webview: &WebView, toolbar: &Toolbar, status_bar: &StatusBar, history: Rc<RefCell<InternalHistory>>) {
        // Intercept navigation to marshall:// URLs from link clicks/JS
        let history_policy = history.clone();
        let toolbar_policy = toolbar.clone();
        webview.connect_decide_policy(move |wv, decision, decision_type| {
            if decision_type == PolicyDecisionType::NavigationAction {
                if let Ok(nav_decision) = decision.clone().downcast::<NavigationPolicyDecision>() {
                    if let Some(request) = nav_decision.request() {
                        if let Some(uri) = request.uri() {
                            let uri_str = uri.as_str();
                            // Only intercept marshall:// URLs
                            if uri_str.starts_with("marshall://") {
                                // Check if we're already on this page (avoid loops)
                                let current_uri = wv.uri().map(|u| u.to_string()).unwrap_or_default();
                                if current_uri.starts_with("marshall://") && current_uri.trim_end_matches('/') == uri_str.trim_end_matches('/') {
                                    // Already on this page, just ignore to prevent loop
                                    decision.ignore();
                                    return true;
                                }
                                
                                decision.ignore();
                                let wv_clone = wv.clone();
                                let uri_owned = uri_str.to_string();
                                let history_clone = history_policy.clone();
                                let toolbar_clone = toolbar_policy.clone();
                                glib::idle_add_local_once(move || {
                                    Self::handle_internal_url(&wv_clone, &uri_owned, &history_clone, &toolbar_clone);
                                });
                                return true;
                            }
                        }
                    }
                }
            }
            false // Let WebKit handle other decisions
        });

        // URL changed - show marshall://home for internal homepage
        let toolbar_clone = toolbar.clone();
        webview.connect_uri_notify(move |wv| {
            if let Some(uri) = wv.uri() {
                // Show marshall://home for internal pages and blank pages
                if uri.starts_with("marshall://") || uri == "about:blank" || uri.is_empty() {
                    toolbar_clone.set_url("marshall://home");
                } else {
                    toolbar_clone.set_url(&uri);
                }
            }
        });

        // Title changed
        webview.connect_title_notify(move |wv| {
            if let Some(title) = wv.title() {
                let _ = title;
            }
        });

        // Can go back/forward changed - WebKit history for external pages
        // Note: Internal marshall:// history is managed separately
        let toolbar_clone = toolbar.clone();
        let history_wb = history.clone();
        webview.connect_notify_local(Some("can-go-back"), move |wv, _| {
            // Combine WebKit's history with our internal history
            let internal_can_back = history_wb.borrow().can_go_back();
            toolbar_clone.set_can_go_back(wv.can_go_back() || internal_can_back);
        });

        let toolbar_clone = toolbar.clone();
        let history_wf = history.clone();
        webview.connect_notify_local(Some("can-go-forward"), move |wv, _| {
            let internal_can_fwd = history_wf.borrow().can_go_forward();
            toolbar_clone.set_can_go_forward(wv.can_go_forward() || internal_can_fwd);
        });

        // Loading progress
        let status_bar_clone = status_bar.clone();
        webview.connect_estimated_load_progress_notify(move |wv| {
            let progress = wv.estimated_load_progress();
            status_bar_clone.set_progress(progress);
        });

        // Load finished - inject Marshall script on DuckDuckGo
        let status_bar_clone2 = status_bar.clone();
        webview.connect_load_changed(move |wv, event| {
            match event {
                LoadEvent::Started => {
                    status_bar_clone2.set_status("Loading...");
                }
                LoadEvent::Committed => {
                    if let Some(uri) = wv.uri() {
                        if uri.contains("duckduckgo.com") || uri.contains("duck.ai") {
                            // Inject early CSS to hide branding immediately
                            Self::inject_marshall_script(wv);
                            status_bar_clone2.set_status("🔍 Dr Marshall Active");
                        } else {
                            status_bar_clone2.set_status("Receiving data...");
                        }
                    }
                }
                LoadEvent::Finished => {
                    if let Some(uri) = wv.uri() {
                        if uri.contains("duckduckgo.com") || uri.contains("duck.ai") {
                            // Inject again after page fully loads
                            Self::inject_marshall_script(wv);
                            status_bar_clone2.set_status("✓ Dr Marshall Complete");
                        } else {
                            status_bar_clone2.set_status("Done");
                        }
                    }
                    status_bar_clone2.set_progress(0.0);
                }
                _ => {}
            }
        });
    }
}
