// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Dr. Marshall - Advanced AI Assistant with Grok-level Intelligence
//! Features multi-provider LLM support, animated avatar, TTS/STT, and integrated tools

pub mod avatar;
pub mod speech;
pub mod conversation;
pub mod ai_engine;
pub mod ai_chat;

use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub use avatar::*;
pub use speech::*;
pub use conversation::*;
pub use ai_engine::*;
pub use ai_chat::*;

/// Assistant personality and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantConfig {
    pub name: String,
    pub voice_id: String,
    pub avatar_style: AvatarStyle,
    pub greeting: String,
    pub personality: Personality,
    pub wake_word: Option<String>,
    pub auto_listen: bool,
    pub tts_enabled: bool,
    pub stt_enabled: bool,
    pub ai_config: AIModelConfig,
}

impl Default for AssistantConfig {
    fn default() -> Self {
        Self {
            name: "Dr. Marshall".to_string(),
            voice_id: "nullsec-voice".to_string(),
            avatar_style: AvatarStyle::CyberPunk,
            greeting: "Welcome to Marshall Command Center. I'm Dr. Marshall, your Chief Intelligence Officer. How can I assist you today?".to_string(),
            personality: Personality::Professional,
            wake_word: Some("Hey Marshall".to_string()),
            auto_listen: false,
            tts_enabled: true,
            stt_enabled: true,
            ai_config: AIModelConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AvatarStyle {
    CyberPunk,
    Professional,
    Hacker,
    Minimal,
    Custom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Personality {
    Professional,
    Friendly,
    Technical,
    Concise,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssistantState {
    Idle,
    Listening,
    Processing,
    Speaking,
    Error,
}

/// Main Dr. Marshall Assistant controller
pub struct DrMarshall {
    pub config: AssistantConfig,
    pub state: Arc<RwLock<AssistantState>>,
    pub avatar: Avatar,
    pub speech: SpeechEngine,
    pub conversation: ConversationManager,
    pub ai_engine: AIEngine,
    pub chat_panel: Option<AIChatPanel>,
}

impl DrMarshall {
    pub fn new(config: AssistantConfig) -> Self {
        let state = Arc::new(RwLock::new(AssistantState::Idle));
        let ai_engine = AIEngine::new(config.ai_config.clone());
        
        Self {
            avatar: Avatar::new(config.avatar_style),
            speech: SpeechEngine::new(config.tts_enabled, config.stt_enabled),
            conversation: ConversationManager::new(),
            ai_engine,
            chat_panel: None,
            config,
            state,
        }
    }

    /// Initialize with chat panel for GUI
    pub fn with_chat_panel(mut self) -> Self {
        self.chat_panel = Some(AIChatPanel::new());
        self
    }

    pub fn greet(&self) {
        let greeting = &self.config.greeting;
        self.speak(greeting);
        self.avatar.animate(AvatarAnimation::Wave);
    }

    pub fn speak(&self, text: &str) {
        *self.state.write() = AssistantState::Speaking;
        self.avatar.animate(AvatarAnimation::Speaking);
        self.speech.speak(text);
        *self.state.write() = AssistantState::Idle;
        self.avatar.animate(AvatarAnimation::Idle);
    }

    pub fn listen(&self) -> Option<String> {
        *self.state.write() = AssistantState::Listening;
        self.avatar.animate(AvatarAnimation::Listening);
        let result = self.speech.listen();
        *self.state.write() = AssistantState::Idle;
        self.avatar.animate(AvatarAnimation::Idle);
        result
    }

    /// Process user input with AI
    pub fn chat(&mut self, input: &str) -> Result<String, AIError> {
        *self.state.write() = AssistantState::Processing;
        self.avatar.animate(AvatarAnimation::Thinking);
        
        let response = self.ai_engine.chat(input);
        
        *self.state.write() = AssistantState::Idle;
        self.avatar.animate(AvatarAnimation::Idle);
        
        response
    }

    /// Process command with fallback to pattern matching
    pub fn process_command(&mut self, input: &str) -> AssistantResponse {
        *self.state.write() = AssistantState::Processing;
        self.avatar.animate(AvatarAnimation::Thinking);
        
        // Try AI first
        let response = match self.ai_engine.chat(input) {
            Ok(ai_response) => {
                // Parse for actions from AI response
                let action = self.parse_action_from_response(&ai_response);
                AssistantResponse::new(&ai_response, action)
            }
            Err(_) => {
                // Fallback to pattern matching
                self.conversation.process(input)
            }
        };
        
        *self.state.write() = AssistantState::Idle;
        response
    }

    /// Parse potential actions from AI response
    fn parse_action_from_response(&self, response: &str) -> Option<AssistantAction> {
        let lower = response.to_lowercase();
        
        // Check for navigation hints
        if lower.contains("navigating to") || lower.contains("opening") {
            if let Some(url) = self.extract_url(response) {
                return Some(AssistantAction::Navigate(url));
            }
        }
        
        // Check for search hints
        if lower.contains("searching for") || lower.contains("search results") {
            if let Some(query) = self.extract_search_query(response) {
                return Some(AssistantAction::Search(query));
            }
        }
        
        // Check for OSINT hints
        if lower.contains("running osint") || lower.contains("reconnaissance") {
            if let Some(target) = self.extract_osint_target(response) {
                return Some(AssistantAction::RunOSINT(target));
            }
        }
        
        // Check for workforce hints
        if lower.contains("workforce") || lower.contains("employee") || lower.contains("timecard") {
            return Some(AssistantAction::ShowWorkforce);
        }
        
        // Check for VoIP hints
        if lower.contains("calling") || lower.contains("dialing") {
            if let Some(contact) = self.extract_contact(response) {
                return Some(AssistantAction::Call(contact));
            }
        }
        
        None
    }

    fn extract_url(&self, text: &str) -> Option<String> {
        let url_regex = regex::Regex::new(r"https?://[^\s]+").ok()?;
        url_regex.find(text).map(|m| m.as_str().to_string())
    }

    fn extract_search_query(&self, text: &str) -> Option<String> {
        let regex = regex::Regex::new(r#"(?i)search(?:ing)?\s+(?:for\s+)?['"]?([^'"]+)['"]?"#).ok()?;
        regex.captures(text).and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
    }

    fn extract_osint_target(&self, text: &str) -> Option<String> {
        let regex = regex::Regex::new(r#"(?i)(?:osint|recon|reconnaissance)\s+(?:on\s+)?['"]?([^'"]+)['"]?"#).ok()?;
        regex.captures(text).and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
    }

    fn extract_contact(&self, text: &str) -> Option<String> {
        let regex = regex::Regex::new(r#"(?i)(?:calling|dialing)\s+['"]?([^'"]+)['"]?"#).ok()?;
        regex.captures(text).and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
    }

    pub fn state(&self) -> AssistantState {
        *self.state.read()
    }

    /// Get the chat panel widget
    pub fn get_chat_widget(&self) -> Option<&gtk::Box> {
        self.chat_panel.as_ref().map(|p| p.widget())
    }

    /// Check if local AI is available
    pub fn check_local_ai(&self) -> bool {
        self.ai_engine.check_local_ai()
    }

    /// List available local models
    pub fn list_local_models(&self) -> Vec<String> {
        self.ai_engine.list_ollama_models()
    }

    /// Set AI provider
    pub fn set_ai_provider(&mut self, provider: AIProvider, api_key: Option<String>) {
        let mut config = self.ai_engine.get_config();
        config.provider = provider;
        config.model = provider.default_model().to_string();
        config.api_key = api_key;
        self.ai_engine.set_config(config);
    }

    /// Set AI model
    pub fn set_ai_model(&mut self, model: &str) {
        let mut config = self.ai_engine.get_config();
        config.model = model.to_string();
        self.ai_engine.set_config(config);
    }
}

// Legacy Assistant type alias for backward compatibility
pub type Assistant = DrMarshall;

/// Response from the assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantResponse {
    pub id: String,
    pub text: String,
    pub action: Option<AssistantAction>,
    pub timestamp: DateTime<Utc>,
    pub confidence: f32,
}

impl AssistantResponse {
    pub fn new(text: &str, action: Option<AssistantAction>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            text: text.to_string(),
            action,
            timestamp: Utc::now(),
            confidence: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssistantAction {
    Navigate(String),
    Search(String),
    Call(String),
    CreateTask(String),
    ShowDashboard,
    ShowWorkforce,
    ShowSearch,
    ShowVoIP,
    RunOSINT(String),
    Custom(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dr_marshall_creation() {
        let config = AssistantConfig::default();
        let marshall = DrMarshall::new(config);
        assert_eq!(marshall.state(), AssistantState::Idle);
    }

    #[test]
    fn test_assistant_config() {
        let config = AssistantConfig::default();
        assert_eq!(config.name, "Dr. Marshall");
    }
}
