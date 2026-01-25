// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! AI Chat Interface with GTK4 UI components
//! Provides the Dr. Marshall chat interface

use gtk4::prelude::*;
use gtk4::{
    Box, Button, Entry, Label, ListBox, ListBoxRow, Orientation,
    ScrolledWindow, TextView, TextBuffer, PolicyType, CssProvider,
    StyleContext, TextTag, WrapMode, Align,
};
use std::sync::Arc;
use parking_lot::RwLock;
use chrono::Local;

use super::ai_engine::{AIEngine, AIModelConfig, AIProvider, AIError};

/// Chat message for display
#[derive(Debug, Clone)]
pub struct ChatDisplayMessage {
    pub is_user: bool,
    pub content: String,
    pub timestamp: String,
}

/// AI Chat Panel Widget
pub struct AIChatPanel {
    pub container: Box,
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

        // Main container
        let container = Box::new(Orientation::Vertical, 0);
        container.set_css_classes(&["ai-chat-panel"]);

        // Header
        let header = Self::create_header();
        container.append(&header);

        // Message area
        let scroll = ScrolledWindow::new();
        scroll.set_vexpand(true);
        scroll.set_policy(PolicyType::Never, PolicyType::Automatic);

        let message_list = ListBox::new();
        message_list.set_css_classes(&["ai-message-list"]);
        message_list.set_selection_mode(gtk4::SelectionMode::None);
        scroll.set_child(Some(&message_list));
        container.append(&scroll);

        // Status label
        let status_label = Label::new(Some("Dr. Marshall ready"));
        status_label.set_css_classes(&["ai-status"]);
        status_label.set_halign(Align::Start);
        status_label.set_margin_start(12);
        status_label.set_margin_bottom(4);
        container.append(&status_label);

        // Input area
        let input_box = Box::new(Orientation::Horizontal, 8);
        input_box.set_margin_start(12); input_box.set_margin_end(12); input_box.set_margin_top(12); input_box.set_margin_bottom(12);

        let input_entry = Entry::new();
        input_entry.set_hexpand(true);
        input_entry.set_placeholder_text(Some("Ask Dr. Marshall anything..."));
        input_entry.set_css_classes(&["ai-input"]);

        let send_button = Button::with_label("Send");
        send_button.set_css_classes(&["ai-send-btn", "suggested-action"]);

        let clear_button = Button::with_label("Clear");
        clear_button.set_css_classes(&["ai-clear-btn"]);

        let settings_button = Button::with_label("⚙");
        settings_button.set_css_classes(&["ai-settings-btn"]);

        input_box.append(&input_entry);
        input_box.append(&send_button);
        input_box.append(&clear_button);
        input_box.append(&settings_button);
        container.append(&input_box);

        // Apply CSS
        Self::apply_styles(&container);

        let panel = Self {
            container,
            message_list,
            input_entry,
            send_button,
            clear_button,
            settings_button,
            status_label,
            engine,
            messages,
        };

        panel.setup_signals();
        panel.add_welcome_message();
        panel
    }

    fn create_header() -> Box {
        let header = Box::new(Orientation::Horizontal, 12);
        header.set_css_classes(&["ai-header"]);
        header.set_margin_start(12); header.set_margin_end(12); header.set_margin_top(12); header.set_margin_bottom(12);

        // Avatar
        let avatar = Label::new(Some("🤖"));
        avatar.set_css_classes(&["ai-avatar"]);

        // Title
        let title_box = Box::new(Orientation::Vertical, 4);
        let title = Label::new(Some("Dr. Marshall"));
        title.set_css_classes(&["ai-title"]);
        title.set_halign(Align::Start);

        let subtitle = Label::new(Some("NullSec Chief Intelligence Officer"));
        subtitle.set_css_classes(&["ai-subtitle"]);
        subtitle.set_halign(Align::Start);

        title_box.append(&title);
        title_box.append(&subtitle);

        header.append(&avatar);
        header.append(&title_box);

        header
    }

    fn apply_styles(widget: &Box) {
        let css = r#"
            .ai-chat-panel {
                background: linear-gradient(180deg, #1a1a2e 0%, #0f0f1a 100%);
                border-radius: 12px;
            }
            .ai-header {
                background: rgba(0, 255, 136, 0.1);
                border-radius: 12px 12px 0 0;
                padding: 16px;
            }
            .ai-avatar {
                font-size: 48px;
                padding: 8px;
            }
            .ai-title {
                font-size: 20px;
                font-weight: bold;
                color: #00ff88;
            }
            .ai-subtitle {
                font-size: 12px;
                color: #888;
            }
            .ai-message-list {
                background: transparent;
            }
            .ai-message {
                padding: 12px;
                margin: 8px 12px;
                border-radius: 12px;
            }
            .ai-message-user {
                background: linear-gradient(135deg, #00ff88 0%, #00cc6a 100%);
                color: #000;
                margin-left: 60px;
            }
            .ai-message-assistant {
                background: rgba(255, 255, 255, 0.1);
                color: #fff;
                margin-right: 60px;
            }
            .ai-message-content {
                font-size: 14px;
                line-height: 1.5;
            }
            .ai-message-time {
                font-size: 10px;
                color: rgba(255, 255, 255, 0.5);
                margin-top: 4px;
            }
            .ai-status {
                font-size: 12px;
                color: #00ff88;
                font-style: italic;
            }
            .ai-input {
                background: rgba(255, 255, 255, 0.1);
                border: 1px solid rgba(0, 255, 136, 0.3);
                border-radius: 8px;
                color: #fff;
                padding: 12px;
            }
            .ai-input:focus {
                border-color: #00ff88;
            }
            .ai-send-btn {
                background: linear-gradient(135deg, #00ff88 0%, #00cc6a 100%);
                color: #000;
                font-weight: bold;
                border-radius: 8px;
                padding: 8px 16px;
            }
            .ai-clear-btn {
                background: rgba(255, 255, 255, 0.1);
                color: #888;
                border-radius: 8px;
            }
            .ai-settings-btn {
                background: rgba(255, 255, 255, 0.1);
                color: #888;
                border-radius: 8px;
                padding: 8px;
            }
        "#;

        let provider = CssProvider::new();
        provider.load_from_data(css);

        if let Some(display) = gtk4::gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn setup_signals(&self) {
        let engine = self.engine.clone();
        let message_list = self.message_list.clone();
        let input_entry = self.input_entry.clone();
        let status_label = self.status_label.clone();
        let messages = self.messages.clone();

        // Send on button click
        let engine_clone = engine.clone();
        let message_list_clone = message_list.clone();
        let input_clone = input_entry.clone();
        let status_clone = status_label.clone();
        let messages_clone = messages.clone();

        self.send_button.connect_clicked(move |_| {
            let text = input_clone.text().to_string();
            if !text.is_empty() {
                Self::send_message(
                    &text,
                    &engine_clone,
                    &message_list_clone,
                    &status_clone,
                    &messages_clone,
                );
                input_clone.set_text("");
            }
        });

        // Send on Enter key
        let engine_clone = engine.clone();
        let message_list_clone = message_list.clone();
        let status_clone = status_label.clone();
        let messages_clone = messages.clone();

        self.input_entry.connect_activate(move |entry| {
            let text = entry.text().to_string();
            if !text.is_empty() {
                Self::send_message(
                    &text,
                    &engine_clone,
                    &message_list_clone,
                    &status_clone,
                    &messages_clone,
                );
                entry.set_text("");
            }
        });

        // Clear history
        let engine_clone = engine.clone();
        let message_list_clone = message_list.clone();
        let messages_clone = messages.clone();

        self.clear_button.connect_clicked(move |_| {
            engine_clone.write().clear_history();
            messages_clone.write().clear();
            while let Some(row) = message_list_clone.row_at_index(0) {
                message_list_clone.remove(&row);
            }
        });
    }

    fn add_welcome_message(&self) {
        let welcome = ChatDisplayMessage {
            is_user: false,
            content: "Welcome to Marshall Command Center. I'm Dr. Marshall, your Chief Intelligence Officer.\n\nI can help you with:\n• **OSINT & Security** - Domain recon, port scanning, vulnerability assessment\n• **Workforce Management** - Time tracking, projects, scheduling\n• **VoIP Communications** - Calls, contacts, voicemail\n• **Privacy-Enhanced Browsing** - Search, bookmarks, ad blocking\n\nHow can I assist you today?".to_string(),
            timestamp: Local::now().format("%H:%M").to_string(),
        };

        self.messages.write().push(welcome.clone());
        Self::add_message_to_list(&self.message_list, &welcome);
    }

    fn send_message(
        text: &str,
        engine: &Arc<RwLock<AIEngine>>,
        message_list: &ListBox,
        status_label: &Label,
        messages: &Arc<RwLock<Vec<ChatDisplayMessage>>>,
    ) {
        // Add user message
        let user_msg = ChatDisplayMessage {
            is_user: true,
            content: text.to_string(),
            timestamp: Local::now().format("%H:%M").to_string(),
        };
        messages.write().push(user_msg.clone());
        Self::add_message_to_list(message_list, &user_msg);

        // Update status
        status_label.set_text("Dr. Marshall is thinking...");

        // Get AI response
        let response = engine.write().chat(text);

        match response {
            Ok(response_text) => {
                let assistant_msg = ChatDisplayMessage {
                    is_user: false,
                    content: response_text,
                    timestamp: Local::now().format("%H:%M").to_string(),
                };
                messages.write().push(assistant_msg.clone());
                Self::add_message_to_list(message_list, &assistant_msg);
                status_label.set_text("Dr. Marshall ready");
            }
            Err(e) => {
                let error_msg = ChatDisplayMessage {
                    is_user: false,
                    content: format!("⚠️ Error: {}\n\nTip: Make sure Ollama is running for local AI, or configure an API key for cloud providers.", e),
                    timestamp: Local::now().format("%H:%M").to_string(),
                };
                messages.write().push(error_msg.clone());
                Self::add_message_to_list(message_list, &error_msg);
                status_label.set_text("Error - check AI configuration");
            }
        }
    }

    fn add_message_to_list(list: &ListBox, msg: &ChatDisplayMessage) {
        let row = ListBoxRow::new();
        row.set_selectable(false);
        row.set_activatable(false);

        let message_box = Box::new(Orientation::Vertical, 4);
        if msg.is_user {
            message_box.set_css_classes(&["ai-message", "ai-message-user"]);
        } else {
            message_box.set_css_classes(&["ai-message", "ai-message-assistant"]);
        }

        let content = Label::new(Some(&msg.content));
        content.set_css_classes(&["ai-message-content"]);
        content.set_wrap(true);
        content.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
        content.set_xalign(0.0);
        content.set_use_markup(true);
        content.set_selectable(true);

        let time = Label::new(Some(&msg.timestamp));
        time.set_css_classes(&["ai-message-time"]);
        time.set_halign(if msg.is_user { Align::End } else { Align::Start });

        message_box.append(&content);
        message_box.append(&time);
        row.set_child(Some(&message_box));

        list.append(&row);

        // Scroll to bottom
        if let Some(adj) = list.parent()
            .and_then(|p| p.downcast::<ScrolledWindow>().ok())
            .map(|sw| sw.vadjustment())
        {
            adj.set_value(adj.upper());
        }
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }

    /// Configure AI provider
    pub fn set_provider(&self, provider: AIProvider, api_key: Option<String>) {
        let mut config = self.engine.read().get_config();
        config.provider = provider;
        config.model = provider.default_model().to_string();
        config.api_key = api_key;
        self.engine.write().set_config(config);
    }

    /// Check if local AI is available
    pub fn check_local_ai(&self) -> bool {
        self.engine.read().check_local_ai()
    }

    /// List available Ollama models
    pub fn list_local_models(&self) -> Vec<String> {
        self.engine.read().list_ollama_models()
    }
}

impl Default for AIChatPanel {
    fn default() -> Self {
        Self::new()
    }
}

/// AI Settings Dialog
pub struct AISettingsDialog {
    pub provider_combo: gtk4::ComboBoxText,
    pub model_entry: Entry,
    pub api_key_entry: Entry,
    pub temperature_scale: gtk4::Scale,
    pub max_tokens_spin: gtk4::SpinButton,
}

impl AISettingsDialog {
    pub fn new() -> Self {
        let provider_combo = gtk4::ComboBoxText::new();
        provider_combo.append(Some("ollama"), "Ollama (Local)");
        provider_combo.append(Some("openai"), "OpenAI (GPT-4)");
        provider_combo.append(Some("anthropic"), "Anthropic (Claude)");
        provider_combo.append(Some("groq"), "Groq (Llama 3.1)");
        provider_combo.append(Some("xai"), "xAI (Grok)");
        provider_combo.append(Some("openrouter"), "OpenRouter");
        provider_combo.append(Some("together"), "Together AI");
        provider_combo.append(Some("mistral"), "Mistral AI");
        provider_combo.set_active_id(Some("ollama"));

        let model_entry = Entry::new();
        model_entry.set_text("llama3.1:8b");
        model_entry.set_placeholder_text(Some("Model name"));

        let api_key_entry = Entry::new();
        api_key_entry.set_visibility(false);
        api_key_entry.set_placeholder_text(Some("API Key (for cloud providers)"));

        let temperature_scale = gtk4::Scale::with_range(
            Orientation::Horizontal,
            0.0,
            2.0,
            0.1,
        );
        temperature_scale.set_value(0.7);

        let max_tokens_spin = gtk4::SpinButton::with_range(256.0, 32768.0, 256.0);
        max_tokens_spin.set_value(4096.0);

        Self {
            provider_combo,
            model_entry,
            api_key_entry,
            temperature_scale,
            max_tokens_spin,
        }
    }

    pub fn get_config(&self) -> AIModelConfig {
        let provider = match self.provider_combo.active_id().as_deref() {
            Some("openai") => AIProvider::OpenAI,
            Some("anthropic") => AIProvider::Anthropic,
            Some("groq") => AIProvider::Groq,
            Some("xai") => AIProvider::XAI,
            Some("openrouter") => AIProvider::OpenRouter,
            Some("together") => AIProvider::Together,
            Some("mistral") => AIProvider::Mistral,
            _ => AIProvider::Ollama,
        };

        let api_key = {
            let text = self.api_key_entry.text();
            if text.is_empty() { None } else { Some(text.to_string()) }
        };

        AIModelConfig {
            provider,
            model: self.model_entry.text().to_string(),
            api_key,
            endpoint: None,
            temperature: self.temperature_scale.value() as f32,
            max_tokens: self.max_tokens_spin.value() as u32,
            top_p: 0.95,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            stream: false,
        }
    }
}

impl Default for AISettingsDialog {
    fn default() -> Self {
        Self::new()
    }
}
