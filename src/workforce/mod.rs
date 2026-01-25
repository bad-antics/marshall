// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Workforce Control Center - Employee, TimeCard, Project, and Payout Management
//! Personal call center workforce tracking system

pub mod employee;
pub mod timecard;
pub mod project;
pub mod payout;

use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

pub use employee::*;
pub use timecard::*;
pub use project::*;
pub use payout::*;

/// Workforce configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkforceConfig {
    pub company_name: String,
    pub timezone: String,
    pub currency: String,
    pub default_hourly_rate: f64,
    pub overtime_multiplier: f64,
    pub overtime_threshold_hours: f64,
    pub require_project_for_clock: bool,
    pub auto_clock_out_hours: f64,
    pub payout_schedule: PayoutSchedule,
}

impl Default for WorkforceConfig {
    fn default() -> Self {
        Self {
            company_name: "NullSec Operations".to_string(),
            timezone: "UTC".to_string(),
            currency: "USD".to_string(),
            default_hourly_rate: 25.0,
            overtime_multiplier: 1.5,
            overtime_threshold_hours: 40.0,
            require_project_for_clock: true,
            auto_clock_out_hours: 12.0,
            payout_schedule: PayoutSchedule::Weekly,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PayoutSchedule {
    Daily,
    Weekly,
    BiWeekly,
    Monthly,
    OnCompletion,
}

/// Main Workforce Control Center
pub struct WorkforceCenter {
    pub config: WorkforceConfig,
    pub employees: Arc<RwLock<EmployeeManager>>,
    pub timecards: Arc<RwLock<TimeCardManager>>,
    pub projects: Arc<RwLock<ProjectManager>>,
    pub payouts: Arc<RwLock<PayoutManager>>,
}

impl WorkforceCenter {
    pub fn new(config: WorkforceConfig) -> Self {
        Self {
            config,
            employees: Arc::new(RwLock::new(EmployeeManager::new())),
            timecards: Arc::new(RwLock::new(TimeCardManager::new())),
            projects: Arc::new(RwLock::new(ProjectManager::new())),
            payouts: Arc::new(RwLock::new(PayoutManager::new())),
        }
    }

    /// Get workforce dashboard summary
    pub fn get_dashboard(&self) -> WorkforceDashboard {
        let employees = self.employees.read();
        let timecards = self.timecards.read();
        let projects = self.projects.read();
        let payouts = self.payouts.read();

        WorkforceDashboard {
            total_employees: employees.count(),
            active_employees: employees.active_count(),
            clocked_in_count: timecards.currently_clocked_in_count(),
            active_projects: projects.active_count(),
            completed_projects: projects.completed_count(),
            pending_payouts: payouts.pending_count(),
            total_pending_amount: payouts.total_pending_amount(),
            hours_today: timecards.total_hours_today(),
            hours_this_week: timecards.total_hours_this_week(),
        }
    }

    /// Clock in an employee
    pub fn clock_in(&self, employee_id: &str, project_id: Option<&str>) -> Result<String, String> {
        let employees = self.employees.read();
        
        // Verify employee exists and is active
        let employee = employees.get(employee_id)
            .ok_or("Employee not found")?;
        
        if employee.status != EmployeeStatus::Active {
            return Err("Employee is not active".to_string());
        }

        // Check if already clocked in
        if self.timecards.read().is_clocked_in(employee_id) {
            return Err("Employee is already clocked in".to_string());
        }

        // Verify project if required
        if self.config.require_project_for_clock {
            let proj_id = project_id.ok_or("Project ID required for clock in")?;
            let projects = self.projects.read();
            let project = projects.get(proj_id)
                .ok_or("Project not found")?;
            
            if project.status != ProjectStatus::InProgress && project.status != ProjectStatus::Open {
                return Err("Project is not active".to_string());
            }
        }

        // Create time entry
        let entry_id = self.timecards.write().clock_in(
            employee_id,
            project_id,
            employee.hourly_rate,
        );

        tracing::info!("Employee {} clocked in", employee_id);
        Ok(entry_id)
    }

    /// Clock out an employee
    pub fn clock_out(&self, employee_id: &str, notes: Option<&str>) -> Result<f64, String> {
        let hours = self.timecards.write().clock_out(employee_id, notes)?;
        tracing::info!("Employee {} clocked out, worked {:.2} hours", employee_id, hours);
        Ok(hours)
    }

    /// Assign employee to project
    pub fn assign_to_project(&self, employee_id: &str, project_id: &str) -> Result<(), String> {
        // Verify both exist
        let employees = self.employees.read();
        let _employee = employees.get(employee_id)
            .ok_or("Employee not found")?;

        self.projects.write().assign_worker(project_id, employee_id)?;
        tracing::info!("Assigned employee {} to project {}", employee_id, project_id);
        Ok(())
    }

    /// Mark project as complete and trigger payout
    pub fn complete_project(&self, project_id: &str) -> Result<Vec<String>, String> {
        let mut projects = self.projects.write();
        let project = projects.get_mut(project_id)
            .ok_or("Project not found")?;
        
        if project.status == ProjectStatus::Completed {
            return Err("Project already completed".to_string());
        }

        project.status = ProjectStatus::Completed;
        project.completed_at = Some(Utc::now());

        // Calculate payouts for all workers
        let mut payout_ids = Vec::new();
        let timecards = self.timecards.read();
        let mut payouts = self.payouts.write();

        for worker_id in &project.assigned_workers {
            let hours = timecards.total_hours_for_project(worker_id, project_id);
            let rate = timecards.get_rate_for_employee(worker_id)
                .unwrap_or(self.config.default_hourly_rate);
            
            if hours > 0.0 {
                let payout_id = payouts.create_payout(
                    worker_id,
                    hours * rate,
                    &format!("Project completion: {}", project.name),
                    Some(project_id),
                );
                payout_ids.push(payout_id);
            }
        }

        tracing::info!("Project {} completed, {} payouts created", project_id, payout_ids.len());
        Ok(payout_ids)
    }
}

/// Dashboard summary data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkforceDashboard {
    pub total_employees: usize,
    pub active_employees: usize,
    pub clocked_in_count: usize,
    pub active_projects: usize,
    pub completed_projects: usize,
    pub pending_payouts: usize,
    pub total_pending_amount: f64,
    pub hours_today: f64,
    pub hours_this_week: f64,
}
