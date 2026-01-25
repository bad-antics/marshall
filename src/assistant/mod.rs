// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Interactive AI Assistant with Talking Head Avatar
//! Provides TTS, STT, and animated avatar for user interaction

pub mod avatar;
pub mod speech;
pub mod conversation;

use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub use avatar::*;
pub use speech::*;
pub use conversation::*;

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
}

impl Default for AssistantConfig {
    fn default() -> Self {
        Self {
            name: "Marshall".to_string(),
            voice_id: "nullsec-voice".to_string(),
            avatar_style: AvatarStyle::CyberPunk,
            greeting: "Welcome to Marshall Command Center. How can I assist you today?".to_string(),
            personality: Personality::Professional,
            wake_word: Some("Hey Marshall".to_string()),
            auto_listen: false,
            tts_enabled: true,
            stt_enabled: true,
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

/// Main Assistant controller
pub struct Assistant {
    pub config: AssistantConfig,
    pub state: Arc<RwLock<AssistantState>>,
    pub avatar: Avatar,
    pub speech: SpeechEngine,
    pub conversation: ConversationManager,
}

impl Assistant {
    pub fn new(config: AssistantConfig) -> Self {
        let state = Arc::new(RwLock::new(AssistantState::Idle));
        
        Self {
            avatar: Avatar::new(config.avatar_style),
            speech: SpeechEngine::new(config.tts_enabled, config.stt_enabled),
            conversation: ConversationManager::new(),
            config,
            state,
        }
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

    pub fn process_command(&self, input: &str) -> AssistantResponse {
        *self.state.write() = AssistantState::Processing;
        self.avatar.animate(AvatarAnimation::Thinking);
        
        let response = self.conversation.process(input);
        
        *self.state.write() = AssistantState::Idle;
        response
    }

    pub fn state(&self) -> AssistantState {
        *self.state.read()
    }
}

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
