// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Employee management

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Employee information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub role: EmployeeRole,
    pub status: EmployeeStatus,
    pub hourly_rate: f64,
    pub skills: Vec<String>,
    pub notes: Option<String>,
    pub avatar_url: Option<String>,
    pub payment_method: PaymentMethod,
    pub payment_info: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_active: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EmployeeStatus {
    Active,
    Inactive,
    OnLeave,
    Terminated,
    Pending,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EmployeeRole {
    Worker,
    TeamLead,
    Manager,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    BankTransfer,
    PayPal,
    Crypto,
    Check,
    Cash,
    Other(String),
}

impl Employee {
    pub fn new(name: &str, email: &str, hourly_rate: f64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            email: email.to_string(),
            phone: None,
            role: EmployeeRole::Worker,
            status: EmployeeStatus::Active,
            hourly_rate,
            skills: Vec::new(),
            notes: None,
            avatar_url: None,
            payment_method: PaymentMethod::BankTransfer,
            payment_info: None,
            created_at: now,
            updated_at: now,
            last_active: None,
        }
    }

    pub fn with_role(mut self, role: EmployeeRole) -> Self {
        self.role = role;
        self
    }

    pub fn with_skills(mut self, skills: Vec<String>) -> Self {
        self.skills = skills;
        self
    }

    pub fn update_activity(&mut self) {
        self.last_active = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}

/// Employee manager
pub struct EmployeeManager {
    employees: HashMap<String, Employee>,
}

impl EmployeeManager {
    pub fn new() -> Self {
        Self {
            employees: HashMap::new(),
        }
    }

    pub fn add(&mut self, employee: Employee) -> String {
        let id = employee.id.clone();
        self.employees.insert(id.clone(), employee);
        id
    }

    pub fn remove(&mut self, id: &str) -> Option<Employee> {
        self.employees.remove(id)
    }

    pub fn get(&self, id: &str) -> Option<&Employee> {
        self.employees.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Employee> {
        self.employees.get_mut(id)
    }

    pub fn search(&self, query: &str) -> Vec<&Employee> {
        let query_lower = query.to_lowercase();
        self.employees
            .values()
            .filter(|e| {
                e.name.to_lowercase().contains(&query_lower)
                    || e.email.to_lowercase().contains(&query_lower)
                    || e.skills.iter().any(|s| s.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn all(&self) -> Vec<&Employee> {
        self.employees.values().collect()
    }

    pub fn by_status(&self, status: EmployeeStatus) -> Vec<&Employee> {
        self.employees
            .values()
            .filter(|e| e.status == status)
            .collect()
    }

    pub fn by_role(&self, role: EmployeeRole) -> Vec<&Employee> {
        self.employees
            .values()
            .filter(|e| e.role == role)
            .collect()
    }

    pub fn count(&self) -> usize {
        self.employees.len()
    }

    pub fn active_count(&self) -> usize {
        self.employees
            .values()
            .filter(|e| e.status == EmployeeStatus::Active)
            .count()
    }

    pub fn set_status(&mut self, id: &str, status: EmployeeStatus) -> bool {
        if let Some(emp) = self.employees.get_mut(id) {
            emp.status = status;
            emp.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    pub fn update_rate(&mut self, id: &str, rate: f64) -> bool {
        if let Some(emp) = self.employees.get_mut(id) {
            emp.hourly_rate = rate;
            emp.updated_at = Utc::now();
            true
        } else {
            false
        }
    }
}

impl Default for EmployeeManager {
    fn default() -> Self {
        Self::new()
    }
}
