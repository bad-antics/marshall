// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! AI Chat Interface with GTK3 UI components
//! Provides the Dr. Marshall chat interface

use gtk::prelude::*;
use gtk::{
    Box as GtkBox, Button, Entry, Label, ListBox, ListBoxRow, Orientation,
    ScrolledWindow, PolicyType, CssProvider, StyleContext,
};
use std::sync::Arc;
use parking_lot::RwLock;
use chrono::Local;

use super::ai_engine::{AIEngine, AIModelConfig, AIProvider};

/// Chat message for display
#[derive(Debug, Clone)]
pub struct ChatDisplayMessage {
    pub is_user: bool,
    pub content: String,
    pub timestamp: String,
}

/// AI Chat Panel Widget
pub struct AIChatPanel {
    pub container: GtkBox,
    pub message_list: ListBox,
    pub input_entry: Entry,
    pub send_button: Button,
    pub clear_button: Button,
    pub settings_button: Button,
    pub status_label: Label,
    pub engine: Arc<RwLock<AIEngine>>,
    pub messages: Arc<RwLock<Vec<ChatDisplayMessage>>>,
}

impl AIChatPanel {
    pub fn new() -> Self {
        let engine = Arc::new(RwLock::new(AIEngine::default()));
        let messages = Arc::new(RwLock::new(Vec::new()));

        let container = GtkBox::new(Orientation::Vertical, 0);
        container.style_context().add_class("ai-chat-panel");

        let header = Self::create_header();
        container.pack_start(&header, false, false, 0);

        let scroll = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        scroll.set_vexpand(true);
        scroll.set_policy(PolicyType::Never, PolicyType::Automatic);

        let message_list = ListBox::new();
        message_list.style_context().add_class("ai-message-list");
        message_list.set_selection_mode(gtk::SelectionMode::None);
        scroll.add(&message_list);
        container.pack_start(&scroll, true, true, 0);

        let status_label = Label::new(Some("Dr. Marshall ready"));
        status_label.style_context().add_class("ai-status");
        status_label.set_halign(gtk::Align::Start);
        status_label.set_margin_start(12);
        status_label.set_margin_bottom(4);
        container.pack_start(&status_label, false, false, 0);

        let input_box = GtkBox::new(Orientation::Horizontal, 8);
        input_box.set_margin_start(12);
        input_box.set_margin_end(12);
        input_box.set_margin_top(12);
        input_box.set_margin_bottom(12);

        let input_entry = Entry::new();
        input_entry.set_hexpand(true);
        input_entry.set_placeholder_text(Some("Ask Dr. Marshall anything..."));
        input_entry.style_context().add_class("ai-input");

        let send_button = Button::with_label("Send");
        send_button.style_context().add_class("ai-send-btn");

        let clear_button = Button::with_label("Clear");
        clear_button.style_context().add_class("ai-clear-btn");

        let settings_button = Button::with_label("⚙");
        settings_button.style_context().add_class("ai-settings-btn");

        input_box.pack_start(&input_entry, true, true, 0);
        input_box.pack_start(&send_button, false, false, 0);
        input_box.pack_start(&clear_button, false, false, 0);
        input_box.pack_start(&settings_button, false, false, 0);
        container.pack_start(&input_box, false, false, 0);

        Self::apply_styles();

        let panel = Self {
            container, message_list, input_entry, send_button, clear_button, settings_button, status_label, engine, messages,
        };

        panel.setup_signals();
        panel.add_welcome_message();
        panel
    }

    fn create_header() -> GtkBox {
        let header = GtkBox::new(Orientation::Horizontal, 12);
        header.style_context().add_class("ai-header");
        header.set_margin_start(12); header.set_margin_end(12); header.set_margin_top(12); header.set_margin_bottom(12);

        let avatar = Label::new(Some("🤖"));
        avatar.style_context().add_class("ai-avatar");

        let title_box = GtkBox::new(Orientation::Vertical, 4);
        let title = Label::new(Some("Dr. Marshall"));
        title.style_context().add_class("ai-title");
        title.set_halign(gtk::Align::Start);

        let subtitle = Label::new(Some("NullSec Chief Intelligence Officer"));
        subtitle.style_context().add_class("ai-subtitle");
        subtitle.set_halign(gtk::Align::Start);

        title_box.pack_start(&title, false, false, 0);
        title_box.pack_start(&subtitle, false, false, 0);
        header.pack_start(&avatar, false, false, 0);
        header.pack_start(&title_box, true, true, 0);
        header
    }

    fn apply_styles() {
        let css = r#"
            .ai-chat-panel { background: #1a1a2e; }
            .ai-header { background: rgba(0, 255, 136, 0.1); padding: 16px; }
            .ai-title { font-size: 20px; font-weight: bold; color: #00ff88; }
            .ai-subtitle { font-size: 12px; color: #888; }
            .ai-status { font-size: 12px; color: #00ff88; font-style: italic; }
            .ai-send-btn { background: #00ff88; color: #000; font-weight: bold; }
        "#;
        let provider = CssProvider::new();
        provider.load_from_data(css.as_bytes()).ok();
        if let Some(screen) = gdk::Screen::default() {
            StyleContext::add_provider_for_screen(&screen, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        }
    }

    fn setup_signals(&self) {
        let engine = self.engine.clone();
        let message_list = self.message_list.clone();
        let input_entry = self.input_entry.clone();
        let status_label = self.status_label.clone();
        let messages = self.messages.clone();

        let e = engine.clone(); let m = message_list.clone(); let i = input_entry.clone(); let s = status_label.clone(); let ms = messages.clone();
        self.send_button.connect_clicked(move |_| {
            let text = i.text().to_string();
            if !text.is_empty() { Self::send_message(&text, &e, &m, &s, &ms); i.set_text(""); }
        });

        let e = engine.clone(); let m = message_list.clone(); let s = status_label.clone(); let ms = messages.clone();
        self.input_entry.connect_activate(move |entry| {
            let text = entry.text().to_string();
            if !text.is_empty() { Self::send_message(&text, &e, &m, &s, &ms); entry.set_text(""); }
        });

        let e = engine.clone(); let m = message_list.clone(); let ms = messages.clone();
        self.clear_button.connect_clicked(move |_| {
            e.write().clear_history(); ms.write().clear();
            for child in m.children() { m.remove(&child); }
        });
    }

    fn add_welcome_message(&self) {
        let welcome = ChatDisplayMessage {
            is_user: false,
            content: "Welcome to Marshall Command Center. I'm Dr. Marshall, your Chief Intelligence Officer.\n\nI can help with OSINT, security research, and more.".to_string(),
            timestamp: Local::now().format("%H:%M").to_string(),
        };
        self.messages.write().push(welcome.clone());
        Self::add_message_to_list(&self.message_list, &welcome);
    }

    fn send_message(text: &str, engine: &Arc<RwLock<AIEngine>>, message_list: &ListBox, status_label: &Label, messages: &Arc<RwLock<Vec<ChatDisplayMessage>>>) {
        let user_msg = ChatDisplayMessage { is_user: true, content: text.to_string(), timestamp: Local::now().format("%H:%M").to_string() };
        messages.write().push(user_msg.clone());
        Self::add_message_to_list(message_list, &user_msg);
        status_label.set_text("Dr. Marshall is thinking...");

        match engine.write().chat(text) {
            Ok(response_text) => {
                let msg = ChatDisplayMessage { is_user: false, content: response_text, timestamp: Local::now().format("%H:%M").to_string() };
                messages.write().push(msg.clone());
                Self::add_message_to_list(message_list, &msg);
                status_label.set_text("Dr. Marshall ready");
            }
            Err(e) => {
                let msg = ChatDisplayMessage { is_user: false, content: format!("Error: {}", e), timestamp: Local::now().format("%H:%M").to_string() };
                messages.write().push(msg.clone());
                Self::add_message_to_list(message_list, &msg);
                status_label.set_text("Error - check AI configuration");
            }
        }
    }

    fn add_message_to_list(list: &ListBox, msg: &ChatDisplayMessage) {
        let row = ListBoxRow::new();
        row.set_selectable(false);
        let message_box = GtkBox::new(Orientation::Vertical, 4);
        message_box.style_context().add_class(if msg.is_user { "ai-message-user" } else { "ai-message-assistant" });

        let content = Label::new(Some(&msg.content));
        content.set_line_wrap(true); content.set_xalign(0.0); content.set_selectable(true);
        let time = Label::new(Some(&msg.timestamp));
        time.set_halign(if msg.is_user { gtk::Align::End } else { gtk::Align::Start });

        message_box.pack_start(&content, false, false, 0);
        message_box.pack_start(&time, false, false, 0);
        row.add(&message_box);
        list.add(&row);
        row.show_all();
    }

    pub fn widget(&self) -> &GtkBox { &self.container }
    pub fn set_provider(&self, provider: AIProvider, api_key: Option<String>) {
        let mut config = self.engine.read().get_config();
        config.provider = provider; config.model = provider.default_model().to_string(); config.api_key = api_key;
        self.engine.write().set_config(config);
    }
    pub fn check_local_ai(&self) -> bool { self.engine.read().check_local_ai() }
    pub fn list_local_models(&self) -> Vec<String> { self.engine.read().list_ollama_models() }
}

impl Default for AIChatPanel { fn default() -> Self { Self::new() } }

pub struct AISettingsDialog {
    pub provider_combo: gtk::ComboBoxText, pub model_entry: Entry, pub api_key_entry: Entry,
    pub temperature_scale: gtk::Scale, pub max_tokens_spin: gtk::SpinButton,
}

impl AISettingsDialog {
    pub fn new() -> Self {
        let provider_combo = gtk::ComboBoxText::new();
        provider_combo.append(Some("ollama"), "Ollama (Local)");
        provider_combo.append(Some("openai"), "OpenAI (GPT-4)");
        provider_combo.append(Some("anthropic"), "Anthropic (Claude)");
        provider_combo.set_active_id(Some("ollama"));

        let model_entry = Entry::new(); model_entry.set_text("llama3.1:8b");
        let api_key_entry = Entry::new(); api_key_entry.set_visibility(false);
        let temperature_scale = gtk::Scale::with_range(Orientation::Horizontal, 0.0, 2.0, 0.1);
        temperature_scale.set_value(0.7);
        let max_tokens_spin = gtk::SpinButton::with_range(256.0, 32768.0, 256.0);
        max_tokens_spin.set_value(4096.0);

        Self { provider_combo, model_entry, api_key_entry, temperature_scale, max_tokens_spin }
    }

    pub fn get_config(&self) -> AIModelConfig {
        let provider = match self.provider_combo.active_id().as_deref() {
            Some("openai") => AIProvider::OpenAI, Some("anthropic") => AIProvider::Anthropic, _ => AIProvider::Ollama,
        };
        let api_key = { let text = self.api_key_entry.text(); if text.is_empty() { None } else { Some(text.to_string()) } };
        AIModelConfig {
            provider, model: self.model_entry.text().to_string(), api_key, endpoint: None,
            temperature: self.temperature_scale.value() as f32, max_tokens: self.max_tokens_spin.value() as u32,
            top_p: 0.95, frequency_penalty: 0.0, presence_penalty: 0.0, stream: false,
        }
    }
}

impl Default for AISettingsDialog { fn default() -> Self { Self::new() } }
