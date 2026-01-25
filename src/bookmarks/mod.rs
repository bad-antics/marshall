// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Bookmark management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub title: String,
    pub url: String,
    pub favicon: Option<String>,
    pub folder_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkFolder {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkStore {
    pub bookmarks: HashMap<String, Bookmark>,
    pub folders: HashMap<String, BookmarkFolder>,
}

impl Default for BookmarkStore {
    fn default() -> Self {
        let mut folders = HashMap::new();
        
        // Create default folders
        let toolbar = BookmarkFolder {
            id: "toolbar".to_string(),
            name: "Bookmarks Toolbar".to_string(),
            parent_id: None,
            created_at: Utc::now(),
        };
        
        let other = BookmarkFolder {
            id: "other".to_string(),
            name: "Other Bookmarks".to_string(),
            parent_id: None,
            created_at: Utc::now(),
        };
        
        folders.insert("toolbar".to_string(), toolbar);
        folders.insert("other".to_string(), other);
        
        Self {
            bookmarks: HashMap::new(),
            folders,
        }
    }
}

pub struct BookmarkManager {
    store: BookmarkStore,
    file_path: PathBuf,
}

impl BookmarkManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = Config::data_dir()?.join("bookmarks.json");
        
        let store = if file_path.exists() {
            let content = fs::read_to_string(&file_path)?;
            serde_json::from_str(&content)?
        } else {
            BookmarkStore::default()
        };
        
        Ok(Self { store, file_path })
    }

    pub fn add_bookmark(&mut self, title: &str, url: &str, folder_id: Option<&str>) -> String {
        let bookmark = Bookmark {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            url: url.to_string(),
            favicon: None,
            folder_id: folder_id.map(|s| s.to_string()),
            created_at: Utc::now(),
            tags: Vec::new(),
        };
        
        let id = bookmark.id.clone();
        self.store.bookmarks.insert(id.clone(), bookmark);
        self.save().ok();
        id
    }

    pub fn remove_bookmark(&mut self, id: &str) -> bool {
        let removed = self.store.bookmarks.remove(id).is_some();
        if removed {
            self.save().ok();
        }
        removed
    }

    pub fn get_bookmark(&self, id: &str) -> Option<&Bookmark> {
        self.store.bookmarks.get(id)
    }

    pub fn get_all_bookmarks(&self) -> Vec<&Bookmark> {
        self.store.bookmarks.values().collect()
    }

    pub fn get_bookmarks_in_folder(&self, folder_id: &str) -> Vec<&Bookmark> {
        self.store.bookmarks.values()
            .filter(|b| b.folder_id.as_deref() == Some(folder_id))
            .collect()
    }

    pub fn search_bookmarks(&self, query: &str) -> Vec<&Bookmark> {
        let query = query.to_lowercase();
        self.store.bookmarks.values()
            .filter(|b| {
                b.title.to_lowercase().contains(&query) ||
                b.url.to_lowercase().contains(&query) ||
                b.tags.iter().any(|t| t.to_lowercase().contains(&query))
            })
            .collect()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(&self.store)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }
}
