// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Text-to-Speech and Speech-to-Text engine

use std::sync::Arc;
use parking_lot::RwLock;
use crossbeam_channel::{Sender, Receiver, bounded};

/// Speech engine state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpeechState {
    Idle,
    Speaking,
    Listening,
    Processing,
}

/// TTS Voice configuration
#[derive(Debug, Clone)]
pub struct VoiceConfig {
    pub voice_id: String,
    pub rate: f32,
    pub pitch: f32,
    pub volume: f32,
    pub language: String,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            voice_id: "default".to_string(),
            rate: 1.0,
            pitch: 1.0,
            volume: 0.8,
            language: "en-US".to_string(),
        }
    }
}

/// Speech recognition result
#[derive(Debug, Clone)]
pub struct SpeechResult {
    pub text: String,
    pub confidence: f32,
    pub alternatives: Vec<String>,
    pub is_final: bool,
}

/// Speech engine for TTS and STT
pub struct SpeechEngine {
    pub tts_enabled: bool,
    pub stt_enabled: bool,
    pub voice_config: VoiceConfig,
    pub state: Arc<RwLock<SpeechState>>,
    command_tx: Sender<SpeechCommand>,
    result_rx: Receiver<SpeechResult>,
}

enum SpeechCommand {
    Speak(String),
    StopSpeaking,
    StartListening,
    StopListening,
}

impl SpeechEngine {
    pub fn new(tts_enabled: bool, stt_enabled: bool) -> Self {
        let (command_tx, _command_rx) = bounded(32);
        let (_result_tx, result_rx) = bounded(32);
        
        Self {
            tts_enabled,
            stt_enabled,
            voice_config: VoiceConfig::default(),
            state: Arc::new(RwLock::new(SpeechState::Idle)),
            command_tx,
            result_rx,
        }
    }

    pub fn speak(&self, text: &str) {
        if !self.tts_enabled {
            return;
        }
        
        *self.state.write() = SpeechState::Speaking;
        let _ = self.command_tx.send(SpeechCommand::Speak(text.to_string()));
        
        // In a real implementation, this would use a TTS library
        // For now, we'll simulate the speech
        tracing::info!("TTS: {}", text);
        
        *self.state.write() = SpeechState::Idle;
    }

    pub fn stop_speaking(&self) {
        let _ = self.command_tx.send(SpeechCommand::StopSpeaking);
        *self.state.write() = SpeechState::Idle;
    }

    pub fn listen(&self) -> Option<String> {
        if !self.stt_enabled {
            return None;
        }
        
        *self.state.write() = SpeechState::Listening;
        let _ = self.command_tx.send(SpeechCommand::StartListening);
        
        // In a real implementation, this would use a STT library
        // For now, return None (would be filled by actual speech recognition)
        
        if let Ok(result) = self.result_rx.try_recv() {
            *self.state.write() = SpeechState::Idle;
            return Some(result.text);
        }
        
        *self.state.write() = SpeechState::Idle;
        None
    }

    pub fn stop_listening(&self) {
        let _ = self.command_tx.send(SpeechCommand::StopListening);
        *self.state.write() = SpeechState::Idle;
    }

    pub fn set_voice(&mut self, config: VoiceConfig) {
        self.voice_config = config;
    }

    pub fn state(&self) -> SpeechState {
        *self.state.read()
    }

    pub fn is_speaking(&self) -> bool {
        *self.state.read() == SpeechState::Speaking
    }

    pub fn is_listening(&self) -> bool {
        *self.state.read() == SpeechState::Listening
    }
}

/// Phoneme for lip-sync
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Phoneme {
    Silent,
    A,
    E,
    I,
    O,
    U,
    M,
    B,
    P,
    F,
    V,
    TH,
    L,
    R,
    S,
    SH,
    K,
    G,
    T,
    D,
    N,
}

impl Phoneme {
    /// Convert text to phonemes for lip-sync (simplified)
    pub fn from_char(c: char) -> Self {
        match c.to_ascii_lowercase() {
            'a' => Phoneme::A,
            'e' => Phoneme::E,
            'i' => Phoneme::I,
            'o' => Phoneme::O,
            'u' => Phoneme::U,
            'm' => Phoneme::M,
            'b' | 'p' => Phoneme::B,
            'f' | 'v' => Phoneme::F,
            'l' => Phoneme::L,
            'r' => Phoneme::R,
            's' | 'z' => Phoneme::S,
            'k' | 'c' | 'g' => Phoneme::K,
            't' | 'd' => Phoneme::T,
            'n' => Phoneme::N,
            ' ' | '.' | ',' | '!' | '?' => Phoneme::Silent,
            _ => Phoneme::Silent,
        }
    }
}
