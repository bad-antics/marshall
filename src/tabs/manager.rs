// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Tab manager

use std::collections::HashMap;
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

/// Individual tab data
#[derive(Clone)]
pub struct TabData {
    pub id: String,
    pub title: String,
    pub url: String,
    pub favicon: Option<String>,
    pub is_loading: bool,
    pub is_private: bool,
}

impl TabData {
    pub fn new(url: &str, is_private: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: "New Tab".to_string(),
            url: url.to_string(),
            favicon: None,
            is_loading: false,
            is_private,
        }
    }
}

/// Tab manager
pub struct TabManager {
    tabs: Arc<RwLock<HashMap<String, TabData>>>,
    tab_order: Arc<RwLock<Vec<String>>>,
    active_tab: Arc<RwLock<Option<String>>>,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: Arc::new(RwLock::new(HashMap::new())),
            tab_order: Arc::new(RwLock::new(Vec::new())),
            active_tab: Arc::new(RwLock::new(None)),
        }
    }

    pub fn create_tab(&self, url: &str, is_private: bool) -> String {
        let tab = TabData::new(url, is_private);
        let id = tab.id.clone();
        
        self.tabs.write().insert(id.clone(), tab);
        self.tab_order.write().push(id.clone());
        *self.active_tab.write() = Some(id.clone());
        
        id
    }

    pub fn close_tab(&self, id: &str) -> bool {
        if self.tabs.write().remove(id).is_some() {
            let mut order = self.tab_order.write();
            if let Some(pos) = order.iter().position(|x| x == id) {
                order.remove(pos);
                
                // Update active tab if needed
                let mut active = self.active_tab.write();
                if active.as_ref() == Some(&id.to_string()) {
                    *active = order.get(pos.saturating_sub(1)).cloned()
                        .or_else(|| order.first().cloned());
                }
            }
            true
        } else {
            false
        }
    }

    pub fn get_tab(&self, id: &str) -> Option<TabData> {
        self.tabs.read().get(id).cloned()
    }

    pub fn get_active_tab(&self) -> Option<TabData> {
        let active_id = self.active_tab.read().clone()?;
        self.get_tab(&active_id)
    }

    pub fn set_active_tab(&self, id: &str) {
        if self.tabs.read().contains_key(id) {
            *self.active_tab.write() = Some(id.to_string());
        }
    }

    pub fn update_tab_title(&self, id: &str, title: &str) {
        if let Some(tab) = self.tabs.write().get_mut(id) {
            tab.title = title.to_string();
        }
    }

    pub fn update_tab_url(&self, id: &str, url: &str) {
        if let Some(tab) = self.tabs.write().get_mut(id) {
            tab.url = url.to_string();
        }
    }

    pub fn tab_count(&self) -> usize {
        self.tabs.read().len()
    }

    pub fn get_all_tabs(&self) -> Vec<TabData> {
        let tabs = self.tabs.read();
        let order = self.tab_order.read();
        order.iter()
            .filter_map(|id| tabs.get(id).cloned())
            .collect()
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}
