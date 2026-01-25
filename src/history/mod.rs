// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Browsing history management

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

use crate::config::Config;

const MAX_HISTORY_ENTRIES: usize = 10000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub visited_at: DateTime<Utc>,
    pub visit_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoryStore {
    pub entries: VecDeque<HistoryEntry>,
}

pub struct HistoryManager {
    store: HistoryStore,
    file_path: PathBuf,
    enabled: bool,
}

impl HistoryManager {
    pub fn new(enabled: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = Config::data_dir()?.join("history.json");
        
        let store = if enabled && file_path.exists() {
            let content = fs::read_to_string(&file_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HistoryStore::default()
        };
        
        Ok(Self { store, file_path, enabled })
    }

    pub fn add_entry(&mut self, url: &str, title: &str) {
        if !self.enabled {
            return;
        }

        // Check if URL already exists
        if let Some(entry) = self.store.entries.iter_mut().find(|e| e.url == url) {
            entry.visit_count += 1;
            entry.visited_at = Utc::now();
            entry.title = title.to_string();
            return;
        }

        let entry = HistoryEntry {
            url: url.to_string(),
            title: title.to_string(),
            visited_at: Utc::now(),
            visit_count: 1,
        };

        self.store.entries.push_front(entry);

        // Trim if over limit
        while self.store.entries.len() > MAX_HISTORY_ENTRIES {
            self.store.entries.pop_back();
        }

        self.save().ok();
    }

    pub fn get_recent(&self, limit: usize) -> Vec<&HistoryEntry> {
        self.store.entries.iter().take(limit).collect()
    }

    pub fn search(&self, query: &str) -> Vec<&HistoryEntry> {
        let query = query.to_lowercase();
        self.store.entries.iter()
            .filter(|e| {
                e.url.to_lowercase().contains(&query) ||
                e.title.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn clear(&mut self) {
        self.store.entries.clear();
        self.save().ok();
    }

    pub fn remove_entry(&mut self, url: &str) {
        self.store.entries.retain(|e| e.url != url);
        self.save().ok();
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.enabled {
            return Ok(());
        }
        
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string(&self.store)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.clear();
        }
    }
}
