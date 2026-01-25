// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Database Module
//! SQLite-based local storage for Marshall

use rusqlite::{Connection, Result as SqlResult};
use std::path::PathBuf;
use parking_lot::Mutex;
use std::sync::Arc;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub path: PathBuf,
    pub enable_wal: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("marshall");
        path.push("data.db");
        
        Self {
            path,
            enable_wal: true,
        }
    }
}

/// Main database manager
pub struct Database {
    conn: Arc<Mutex<Connection>>,
    #[allow(dead_code)]
    config: DatabaseConfig,
}

impl Database {
    pub fn new(config: DatabaseConfig) -> SqlResult<Self> {
        if let Some(parent) = config.path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        let conn = Connection::open(&config.path)?;

        if config.enable_wal {
            conn.execute_batch("PRAGMA journal_mode=WAL;")?;
        }

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
            config,
        };

        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> SqlResult<()> {
        let conn = self.conn.lock();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS employees (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT UNIQUE,
                phone TEXT,
                role TEXT,
                status TEXT DEFAULT 'Active',
                hourly_rate REAL DEFAULT 0,
                payment_method TEXT,
                created_at TEXT,
                updated_at TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS time_entries (
                id TEXT PRIMARY KEY,
                employee_id TEXT NOT NULL,
                project_id TEXT,
                clock_in TEXT NOT NULL,
                clock_out TEXT,
                break_minutes INTEGER DEFAULT 0,
                notes TEXT,
                status TEXT DEFAULT 'Pending'
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                client TEXT,
                status TEXT DEFAULT 'Active',
                priority TEXT DEFAULT 'Medium',
                budget REAL,
                actual_cost REAL DEFAULT 0,
                start_date TEXT,
                due_date TEXT,
                completed_date TEXT,
                created_at TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS payouts (
                id TEXT PRIMARY KEY,
                employee_id TEXT NOT NULL,
                project_id TEXT,
                amount REAL NOT NULL,
                description TEXT,
                status TEXT DEFAULT 'Pending',
                approved_by TEXT,
                approved_at TEXT,
                paid_at TEXT,
                created_at TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS contacts (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                phone_primary TEXT,
                phone_secondary TEXT,
                email TEXT,
                company TEXT,
                notes TEXT,
                favorite INTEGER DEFAULT 0,
                created_at TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS call_history (
                id TEXT PRIMARY KEY,
                contact_id TEXT,
                phone_number TEXT NOT NULL,
                direction TEXT NOT NULL,
                duration_seconds INTEGER,
                status TEXT,
                notes TEXT,
                started_at TEXT,
                ended_at TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS osint_cache (
                domain TEXT PRIMARY KEY,
                whois_data TEXT,
                ports_data TEXT,
                vuln_data TEXT,
                cached_at TEXT,
                expires_at TEXT
            )",
            [],
        )?;

        Ok(())
    }

    pub fn conn(&self) -> &Arc<Mutex<Connection>> {
        &self.conn
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new(DatabaseConfig::default()).expect("Failed to initialize database")
    }
}
