// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Call queue and worker assignment for call center operations

use std::collections::VecDeque;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Queued call waiting to be handled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedCall {
    pub id: String,
    pub caller_number: String,
    pub caller_name: Option<String>,
    pub queue_name: String,
    pub queued_at: DateTime<Utc>,
    pub priority: CallPriority,
    pub reason: Option<String>,
    pub assigned_worker: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CallPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Urgent = 3,
}

impl QueuedCall {
    pub fn new(caller_number: &str, queue_name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            caller_number: caller_number.to_string(),
            caller_name: None,
            queue_name: queue_name.to_string(),
            queued_at: Utc::now(),
            priority: CallPriority::Normal,
            reason: None,
            assigned_worker: None,
        }
    }

    pub fn wait_time_seconds(&self) -> i64 {
        (Utc::now() - self.queued_at).num_seconds()
    }
}

/// Call queue for call center operations
pub struct CallQueue {
    pub name: String,
    pub calls: Arc<RwLock<VecDeque<QueuedCall>>>,
    pub max_size: usize,
    pub max_wait_time: i64, // seconds
}

impl CallQueue {
    pub fn new(name: &str, max_size: usize) -> Self {
        Self {
            name: name.to_string(),
            calls: Arc::new(RwLock::new(VecDeque::new())),
            max_size,
            max_wait_time: 300, // 5 minutes default
        }
    }

    pub fn enqueue(&self, call: QueuedCall) -> Result<(), String> {
        let mut calls = self.calls.write();
        if calls.len() >= self.max_size {
            return Err("Queue is full".to_string());
        }
        
        // Insert by priority
        let pos = calls
            .iter()
            .position(|c| c.priority < call.priority)
            .unwrap_or(calls.len());
        calls.insert(pos, call);
        Ok(())
    }

    pub fn dequeue(&self) -> Option<QueuedCall> {
        self.calls.write().pop_front()
    }

    pub fn peek(&self) -> Option<QueuedCall> {
        self.calls.read().front().cloned()
    }

    pub fn remove(&self, call_id: &str) -> Option<QueuedCall> {
        let mut calls = self.calls.write();
        if let Some(pos) = calls.iter().position(|c| c.id == call_id) {
            calls.remove(pos)
        } else {
            None
        }
    }

    pub fn assign_to_worker(&self, call_id: &str, worker_id: &str) -> bool {
        let mut calls = self.calls.write();
        if let Some(call) = calls.iter_mut().find(|c| c.id == call_id) {
            call.assigned_worker = Some(worker_id.to_string());
            true
        } else {
            false
        }
    }

    pub fn size(&self) -> usize {
        self.calls.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.calls.read().is_empty()
    }

    pub fn average_wait_time(&self) -> i64 {
        let calls = self.calls.read();
        if calls.is_empty() {
            return 0;
        }
        let total: i64 = calls.iter().map(|c| c.wait_time_seconds()).sum();
        total / calls.len() as i64
    }

    pub fn get_all(&self) -> Vec<QueuedCall> {
        self.calls.read().iter().cloned().collect()
    }
}
