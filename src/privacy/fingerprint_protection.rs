// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Fingerprint protection

use rand::Rng;

/// Fingerprint protection to prevent browser identification
pub struct FingerprintProtection {
    enabled: bool,
}

impl FingerprintProtection {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Get a randomized canvas noise value
    pub fn canvas_noise(&self) -> f64 {
        if self.enabled {
            let mut rng = rand::thread_rng();
            rng.gen_range(-0.0001..0.0001)
        } else {
            0.0
        }
    }

    /// Get a spoofed timezone offset
    pub fn timezone_offset(&self) -> i32 {
        if self.enabled {
            0 // UTC
        } else {
            // Return actual timezone
            chrono::Local::now().offset().local_minus_utc() / 60
        }
    }

    /// Get spoofed screen dimensions
    pub fn screen_dimensions(&self) -> (u32, u32) {
        if self.enabled {
            // Return common resolution
            (1920, 1080)
        } else {
            // Would return actual dimensions
            (1920, 1080)
        }
    }

    /// Get spoofed hardware concurrency
    pub fn hardware_concurrency(&self) -> u32 {
        if self.enabled {
            4 // Common value
        } else {
            num_cpus::get() as u32
        }
    }

    /// Get spoofed device memory
    pub fn device_memory(&self) -> u32 {
        if self.enabled {
            8 // Common value (GB)
        } else {
            8 // Would detect actual
        }
    }
}
