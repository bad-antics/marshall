// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Audio handling for VoIP calls

use std::sync::Arc;
use parking_lot::RwLock;

/// Audio device information
#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub is_input: bool,
    pub is_default: bool,
}

/// Audio settings for calls
#[derive(Debug, Clone)]
pub struct AudioSettings {
    pub input_device: Option<String>,
    pub output_device: Option<String>,
    pub input_volume: f32,
    pub output_volume: f32,
    pub echo_cancellation: bool,
    pub noise_suppression: bool,
    pub auto_gain_control: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            input_device: None,
            output_device: None,
            input_volume: 1.0,
            output_volume: 1.0,
            echo_cancellation: true,
            noise_suppression: true,
            auto_gain_control: true,
        }
    }
}

/// Audio engine for VoIP
pub struct AudioEngine {
    pub settings: Arc<RwLock<AudioSettings>>,
    pub is_active: Arc<RwLock<bool>>,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            settings: Arc::new(RwLock::new(AudioSettings::default())),
            is_active: Arc::new(RwLock::new(false)),
        }
    }

    pub fn list_devices(&self) -> Vec<AudioDevice> {
        // In real implementation, enumerate audio devices
        vec![
            AudioDevice {
                id: "default_input".to_string(),
                name: "Default Microphone".to_string(),
                is_input: true,
                is_default: true,
            },
            AudioDevice {
                id: "default_output".to_string(),
                name: "Default Speakers".to_string(),
                is_input: false,
                is_default: true,
            },
        ]
    }

    pub fn start(&self) {
        *self.is_active.write() = true;
        tracing::info!("Audio engine started");
    }

    pub fn stop(&self) {
        *self.is_active.write() = false;
        tracing::info!("Audio engine stopped");
    }

    pub fn set_input_volume(&self, volume: f32) {
        self.settings.write().input_volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_output_volume(&self, volume: f32) {
        self.settings.write().output_volume = volume.clamp(0.0, 1.0);
    }

    pub fn is_active(&self) -> bool {
        *self.is_active.read()
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}
