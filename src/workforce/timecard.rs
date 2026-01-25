// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Time card and clock in/out management

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration, Datelike, Weekday};
use uuid::Uuid;

/// Time entry for an employee's work session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: String,
    pub employee_id: String,
    pub project_id: Option<String>,
    pub clock_in: DateTime<Utc>,
    pub clock_out: Option<DateTime<Utc>>,
    pub break_minutes: i32,
    pub hourly_rate: f64,
    pub notes: Option<String>,
    pub status: TimeEntryStatus,
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TimeEntryStatus {
    Active,      // Currently clocked in
    Completed,   // Clocked out, pending approval
    Approved,    // Approved for payment
    Rejected,    // Rejected, needs review
    Paid,        // Already paid out
}

impl TimeEntry {
    pub fn new(employee_id: &str, project_id: Option<&str>, hourly_rate: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            employee_id: employee_id.to_string(),
            project_id: project_id.map(|s| s.to_string()),
            clock_in: Utc::now(),
            clock_out: None,
            break_minutes: 0,
            hourly_rate,
            notes: None,
            status: TimeEntryStatus::Active,
            approved_by: None,
            approved_at: None,
        }
    }

    /// Calculate hours worked (excluding breaks)
    pub fn hours_worked(&self) -> f64 {
        let end = self.clock_out.unwrap_or_else(Utc::now);
        let duration = end - self.clock_in;
        let total_minutes = duration.num_minutes() as f64;
        let work_minutes = total_minutes - self.break_minutes as f64;
        (work_minutes / 60.0).max(0.0)
    }

    /// Calculate earnings for this entry
    pub fn earnings(&self) -> f64 {
        self.hours_worked() * self.hourly_rate
    }

    /// Format duration as HH:MM
    pub fn duration_string(&self) -> String {
        let hours = self.hours_worked();
        let h = hours as i32;
        let m = ((hours - h as f64) * 60.0) as i32;
        format!("{:02}:{:02}", h, m)
    }

    /// Check if this entry is from today
    pub fn is_today(&self) -> bool {
        let today = Utc::now().date_naive();
        self.clock_in.date_naive() == today
    }

    /// Check if this entry is from this week
    pub fn is_this_week(&self) -> bool {
        let now = Utc::now();
        let entry_date = self.clock_in.date_naive();
        let today = now.date_naive();
        
        // Get start of week (Monday)
        let days_since_monday = today.weekday().num_days_from_monday();
        let week_start = today - Duration::days(days_since_monday as i64);
        let week_end = week_start + Duration::days(6);
        
        entry_date >= week_start && entry_date <= week_end
    }
}

/// Time card manager
pub struct TimeCardManager {
    entries: HashMap<String, TimeEntry>,
    active_sessions: HashMap<String, String>, // employee_id -> entry_id
}

impl TimeCardManager {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            active_sessions: HashMap::new(),
        }
    }

    /// Clock in an employee
    pub fn clock_in(&mut self, employee_id: &str, project_id: Option<&str>, hourly_rate: f64) -> String {
        let entry = TimeEntry::new(employee_id, project_id, hourly_rate);
        let entry_id = entry.id.clone();
        
        self.active_sessions.insert(employee_id.to_string(), entry_id.clone());
        self.entries.insert(entry_id.clone(), entry);
        
        entry_id
    }

    /// Clock out an employee
    pub fn clock_out(&mut self, employee_id: &str, notes: Option<&str>) -> Result<f64, String> {
        let entry_id = self.active_sessions.remove(employee_id)
            .ok_or("Employee is not clocked in")?;
        
        let entry = self.entries.get_mut(&entry_id)
            .ok_or("Time entry not found")?;
        
        entry.clock_out = Some(Utc::now());
        entry.status = TimeEntryStatus::Completed;
        if let Some(n) = notes {
            entry.notes = Some(n.to_string());
        }
        
        Ok(entry.hours_worked())
    }

    /// Check if employee is currently clocked in
    pub fn is_clocked_in(&self, employee_id: &str) -> bool {
        self.active_sessions.contains_key(employee_id)
    }

    /// Get current session for employee
    pub fn get_active_session(&self, employee_id: &str) -> Option<&TimeEntry> {
        self.active_sessions
            .get(employee_id)
            .and_then(|id| self.entries.get(id))
    }

    /// Get all entries for an employee
    pub fn entries_for_employee(&self, employee_id: &str) -> Vec<&TimeEntry> {
        self.entries
            .values()
            .filter(|e| e.employee_id == employee_id)
            .collect()
    }

    /// Get all entries for a project
    pub fn entries_for_project(&self, project_id: &str) -> Vec<&TimeEntry> {
        self.entries
            .values()
            .filter(|e| e.project_id.as_ref().map(|p| p == project_id).unwrap_or(false))
            .collect()
    }

    /// Total hours for an employee on a project
    pub fn total_hours_for_project(&self, employee_id: &str, project_id: &str) -> f64 {
        self.entries
            .values()
            .filter(|e| {
                e.employee_id == employee_id
                    && e.project_id.as_ref().map(|p| p == project_id).unwrap_or(false)
            })
            .map(|e| e.hours_worked())
            .sum()
    }

    /// Get hourly rate for employee (from most recent entry)
    pub fn get_rate_for_employee(&self, employee_id: &str) -> Option<f64> {
        self.entries
            .values()
            .filter(|e| e.employee_id == employee_id)
            .max_by_key(|e| e.clock_in)
            .map(|e| e.hourly_rate)
    }

    /// Count of currently clocked in employees
    pub fn currently_clocked_in_count(&self) -> usize {
        self.active_sessions.len()
    }

    /// Total hours worked today (all employees)
    pub fn total_hours_today(&self) -> f64 {
        self.entries
            .values()
            .filter(|e| e.is_today())
            .map(|e| e.hours_worked())
            .sum()
    }

    /// Total hours worked this week (all employees)
    pub fn total_hours_this_week(&self) -> f64 {
        self.entries
            .values()
            .filter(|e| e.is_this_week())
            .map(|e| e.hours_worked())
            .sum()
    }

    /// Approve a time entry
    pub fn approve(&mut self, entry_id: &str, approver_id: &str) -> bool {
        if let Some(entry) = self.entries.get_mut(entry_id) {
            if entry.status == TimeEntryStatus::Completed {
                entry.status = TimeEntryStatus::Approved;
                entry.approved_by = Some(approver_id.to_string());
                entry.approved_at = Some(Utc::now());
                return true;
            }
        }
        false
    }

    /// Reject a time entry
    pub fn reject(&mut self, entry_id: &str, reason: &str) -> bool {
        if let Some(entry) = self.entries.get_mut(entry_id) {
            if entry.status == TimeEntryStatus::Completed {
                entry.status = TimeEntryStatus::Rejected;
                entry.notes = Some(format!("Rejected: {}", reason));
                return true;
            }
        }
        false
    }

    /// Get all entries
    pub fn all(&self) -> Vec<&TimeEntry> {
        self.entries.values().collect()
    }

    /// Get pending entries (completed but not approved)
    pub fn pending(&self) -> Vec<&TimeEntry> {
        self.entries
            .values()
            .filter(|e| e.status == TimeEntryStatus::Completed)
            .collect()
    }
}

impl Default for TimeCardManager {
    fn default() -> Self {
        Self::new()
    }
}
