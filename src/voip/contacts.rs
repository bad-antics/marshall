// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Contact management for VoIP

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub phone_numbers: Vec<PhoneNumber>,
    pub email: Option<String>,
    pub company: Option<String>,
    pub notes: Option<String>,
    pub avatar_url: Option<String>,
    pub is_favorite: bool,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub number: String,
    pub label: PhoneLabel,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PhoneLabel {
    Mobile,
    Work,
    Home,
    Other,
}

impl Contact {
    pub fn new(name: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            phone_numbers: Vec::new(),
            email: None,
            company: None,
            notes: None,
            avatar_url: None,
            is_favorite: false,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_phone(&mut self, number: &str, label: PhoneLabel, is_primary: bool) {
        self.phone_numbers.push(PhoneNumber {
            number: number.to_string(),
            label,
            is_primary,
        });
        self.updated_at = Utc::now();
    }

    pub fn primary_number(&self) -> Option<&str> {
        self.phone_numbers
            .iter()
            .find(|p| p.is_primary)
            .or_else(|| self.phone_numbers.first())
            .map(|p| p.number.as_str())
    }
}

/// Contact manager
pub struct ContactManager {
    contacts: HashMap<String, Contact>,
}

impl ContactManager {
    pub fn new() -> Self {
        Self {
            contacts: HashMap::new(),
        }
    }

    pub fn add(&mut self, contact: Contact) -> String {
        let id = contact.id.clone();
        self.contacts.insert(id.clone(), contact);
        id
    }

    pub fn remove(&mut self, id: &str) -> Option<Contact> {
        self.contacts.remove(id)
    }

    pub fn get(&self, id: &str) -> Option<&Contact> {
        self.contacts.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Contact> {
        self.contacts.get_mut(id)
    }

    pub fn find_by_number(&self, number: &str) -> Option<&Contact> {
        let normalized = normalize_number(number);
        self.contacts.values().find(|c| {
            c.phone_numbers
                .iter()
                .any(|p| normalize_number(&p.number) == normalized)
        })
    }

    pub fn search(&self, query: &str) -> Vec<&Contact> {
        let query_lower = query.to_lowercase();
        self.contacts
            .values()
            .filter(|c| {
                c.name.to_lowercase().contains(&query_lower)
                    || c.phone_numbers.iter().any(|p| p.number.contains(query))
                    || c.company.as_ref().map_or(false, |co| co.to_lowercase().contains(&query_lower))
                    || c.email.as_ref().map_or(false, |e| e.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn all(&self) -> Vec<&Contact> {
        self.contacts.values().collect()
    }

    pub fn favorites(&self) -> Vec<&Contact> {
        self.contacts
            .values()
            .filter(|c| c.is_favorite)
            .collect()
    }

    pub fn by_tag(&self, tag: &str) -> Vec<&Contact> {
        self.contacts
            .values()
            .filter(|c| c.tags.iter().any(|t| t == tag))
            .collect()
    }

    pub fn count(&self) -> usize {
        self.contacts.len()
    }
}

impl Default for ContactManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Normalize phone number for comparison
fn normalize_number(number: &str) -> String {
    number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect()
}
