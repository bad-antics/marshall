// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Project management and tracking

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Project information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub client: Option<String>,
    pub status: ProjectStatus,
    pub priority: ProjectPriority,
    pub budget_hours: Option<f64>,
    pub budget_amount: Option<f64>,
    pub hourly_rate: Option<f64>,
    pub assigned_workers: Vec<String>,
    pub tasks: Vec<Task>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub due_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ProjectStatus {
    Draft,
    Open,
    InProgress,
    OnHold,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProjectPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Urgent = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub assigned_to: Option<String>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: f64,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Review,
    Completed,
    Blocked,
}

impl Project {
    pub fn new(name: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: None,
            client: None,
            status: ProjectStatus::Draft,
            priority: ProjectPriority::Normal,
            budget_hours: None,
            budget_amount: None,
            hourly_rate: None,
            assigned_workers: Vec::new(),
            tasks: Vec::new(),
            tags: Vec::new(),
            created_at: now,
            started_at: None,
            due_at: None,
            completed_at: None,
            created_by: None,
        }
    }

    pub fn add_task(&mut self, name: &str) -> String {
        let task = Task {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: None,
            status: TaskStatus::Todo,
            assigned_to: None,
            estimated_hours: None,
            actual_hours: 0.0,
            created_at: Utc::now(),
            completed_at: None,
        };
        let id = task.id.clone();
        self.tasks.push(task);
        id
    }

    pub fn progress_percent(&self) -> f64 {
        if self.tasks.is_empty() {
            return 0.0;
        }
        let completed = self.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
        (completed as f64 / self.tasks.len() as f64) * 100.0
    }

    pub fn total_estimated_hours(&self) -> f64 {
        self.tasks.iter().filter_map(|t| t.estimated_hours).sum()
    }

    pub fn total_actual_hours(&self) -> f64 {
        self.tasks.iter().map(|t| t.actual_hours).sum()
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_at {
            return Utc::now() > due && self.status != ProjectStatus::Completed;
        }
        false
    }

    pub fn is_over_budget(&self) -> bool {
        if let Some(budget) = self.budget_hours {
            return self.total_actual_hours() > budget;
        }
        false
    }
}

impl Task {
    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(Utc::now());
    }
}

/// Project manager
pub struct ProjectManager {
    projects: HashMap<String, Project>,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
        }
    }

    pub fn add(&mut self, project: Project) -> String {
        let id = project.id.clone();
        self.projects.insert(id.clone(), project);
        id
    }

    pub fn remove(&mut self, id: &str) -> Option<Project> {
        self.projects.remove(id)
    }

    pub fn get(&self, id: &str) -> Option<&Project> {
        self.projects.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Project> {
        self.projects.get_mut(id)
    }

    pub fn assign_worker(&mut self, project_id: &str, worker_id: &str) -> Result<(), String> {
        let project = self.projects.get_mut(project_id)
            .ok_or("Project not found")?;
        
        if !project.assigned_workers.contains(&worker_id.to_string()) {
            project.assigned_workers.push(worker_id.to_string());
        }
        Ok(())
    }

    pub fn remove_worker(&mut self, project_id: &str, worker_id: &str) -> bool {
        if let Some(project) = self.projects.get_mut(project_id) {
            if let Some(pos) = project.assigned_workers.iter().position(|w| w == worker_id) {
                project.assigned_workers.remove(pos);
                return true;
            }
        }
        false
    }

    pub fn search(&self, query: &str) -> Vec<&Project> {
        let query_lower = query.to_lowercase();
        self.projects
            .values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower)
                    || p.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower))
                    || p.client.as_ref().map_or(false, |c| c.to_lowercase().contains(&query_lower))
                    || p.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn all(&self) -> Vec<&Project> {
        self.projects.values().collect()
    }

    pub fn by_status(&self, status: ProjectStatus) -> Vec<&Project> {
        self.projects
            .values()
            .filter(|p| p.status == status)
            .collect()
    }

    pub fn for_worker(&self, worker_id: &str) -> Vec<&Project> {
        self.projects
            .values()
            .filter(|p| p.assigned_workers.contains(&worker_id.to_string()))
            .collect()
    }

    pub fn active_count(&self) -> usize {
        self.projects
            .values()
            .filter(|p| p.status == ProjectStatus::InProgress || p.status == ProjectStatus::Open)
            .count()
    }

    pub fn completed_count(&self) -> usize {
        self.projects
            .values()
            .filter(|p| p.status == ProjectStatus::Completed)
            .count()
    }

    pub fn overdue(&self) -> Vec<&Project> {
        self.projects
            .values()
            .filter(|p| p.is_overdue())
            .collect()
    }

    pub fn set_status(&mut self, id: &str, status: ProjectStatus) -> bool {
        if let Some(project) = self.projects.get_mut(id) {
            project.status = status;
            if status == ProjectStatus::InProgress && project.started_at.is_none() {
                project.started_at = Some(Utc::now());
            }
            if status == ProjectStatus::Completed {
                project.completed_at = Some(Utc::now());
            }
            true
        } else {
            false
        }
    }
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}
