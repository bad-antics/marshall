// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Payout management for worker compensation

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Payout record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payout {
    pub id: String,
    pub employee_id: String,
    pub amount: f64,
    pub currency: String,
    pub description: String,
    pub project_id: Option<String>,
    pub status: PayoutStatus,
    pub payment_method: Option<String>,
    pub transaction_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
    pub paid_at: Option<DateTime<Utc>>,
    pub approved_by: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PayoutStatus {
    Pending,
    Approved,
    Processing,
    Paid,
    Failed,
    Cancelled,
}

impl Payout {
    pub fn new(employee_id: &str, amount: f64, description: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            employee_id: employee_id.to_string(),
            amount,
            currency: "USD".to_string(),
            description: description.to_string(),
            project_id: None,
            status: PayoutStatus::Pending,
            payment_method: None,
            transaction_id: None,
            created_at: Utc::now(),
            approved_at: None,
            paid_at: None,
            approved_by: None,
            notes: None,
        }
    }

    pub fn formatted_amount(&self) -> String {
        format!("{:.2} {}", self.amount, self.currency)
    }
}

/// Payout manager
pub struct PayoutManager {
    payouts: HashMap<String, Payout>,
}

impl PayoutManager {
    pub fn new() -> Self {
        Self {
            payouts: HashMap::new(),
        }
    }

    pub fn create_payout(
        &mut self,
        employee_id: &str,
        amount: f64,
        description: &str,
        project_id: Option<&str>,
    ) -> String {
        let mut payout = Payout::new(employee_id, amount, description);
        payout.project_id = project_id.map(|s| s.to_string());
        
        let id = payout.id.clone();
        self.payouts.insert(id.clone(), payout);
        id
    }

    pub fn get(&self, id: &str) -> Option<&Payout> {
        self.payouts.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Payout> {
        self.payouts.get_mut(id)
    }

    pub fn approve(&mut self, payout_id: &str, approver_id: &str) -> Result<(), String> {
        let payout = self.payouts.get_mut(payout_id)
            .ok_or("Payout not found")?;
        
        if payout.status != PayoutStatus::Pending {
            return Err("Payout is not pending".to_string());
        }

        payout.status = PayoutStatus::Approved;
        payout.approved_at = Some(Utc::now());
        payout.approved_by = Some(approver_id.to_string());
        Ok(())
    }

    pub fn mark_paid(&mut self, payout_id: &str, transaction_id: Option<&str>) -> Result<(), String> {
        let payout = self.payouts.get_mut(payout_id)
            .ok_or("Payout not found")?;
        
        if payout.status != PayoutStatus::Approved && payout.status != PayoutStatus::Processing {
            return Err("Payout must be approved first".to_string());
        }

        payout.status = PayoutStatus::Paid;
        payout.paid_at = Some(Utc::now());
        payout.transaction_id = transaction_id.map(|s| s.to_string());
        Ok(())
    }

    pub fn cancel(&mut self, payout_id: &str, reason: &str) -> Result<(), String> {
        let payout = self.payouts.get_mut(payout_id)
            .ok_or("Payout not found")?;
        
        if payout.status == PayoutStatus::Paid {
            return Err("Cannot cancel a paid payout".to_string());
        }

        payout.status = PayoutStatus::Cancelled;
        payout.notes = Some(format!("Cancelled: {}", reason));
        Ok(())
    }

    pub fn for_employee(&self, employee_id: &str) -> Vec<&Payout> {
        self.payouts
            .values()
            .filter(|p| p.employee_id == employee_id)
            .collect()
    }

    pub fn for_project(&self, project_id: &str) -> Vec<&Payout> {
        self.payouts
            .values()
            .filter(|p| p.project_id.as_ref().map(|id| id == project_id).unwrap_or(false))
            .collect()
    }

    pub fn by_status(&self, status: PayoutStatus) -> Vec<&Payout> {
        self.payouts
            .values()
            .filter(|p| p.status == status)
            .collect()
    }

    pub fn pending(&self) -> Vec<&Payout> {
        self.by_status(PayoutStatus::Pending)
    }

    pub fn pending_count(&self) -> usize {
        self.payouts
            .values()
            .filter(|p| p.status == PayoutStatus::Pending)
            .count()
    }

    pub fn total_pending_amount(&self) -> f64 {
        self.payouts
            .values()
            .filter(|p| p.status == PayoutStatus::Pending || p.status == PayoutStatus::Approved)
            .map(|p| p.amount)
            .sum()
    }

    pub fn total_paid(&self, employee_id: &str) -> f64 {
        self.payouts
            .values()
            .filter(|p| p.employee_id == employee_id && p.status == PayoutStatus::Paid)
            .map(|p| p.amount)
            .sum()
    }

    pub fn all(&self) -> Vec<&Payout> {
        self.payouts.values().collect()
    }
}

impl Default for PayoutManager {
    fn default() -> Self {
        Self::new()
    }
}
