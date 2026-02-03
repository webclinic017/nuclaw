//! Database for NuClaw

use crate::config::store_dir;
use crate::types::{ChatInfo, NewMessage, ScheduledTask, TaskRunLog};
use rusqlite::{Connection, Result as SqlResult};
use std::sync::Mutex;

pub struct Database {
    connection: Mutex<Connection>,
}

impl Database {
    pub fn new() -> SqlResult<Self> {
        let db_path = store_dir().join("nuclaw.db");
        let connection = Connection::open(&db_path)?;

        // Create tables
        connection.execute(
            "CREATE TABLE IF NOT EXISTS chats (
                jid TEXT PRIMARY KEY,
                name TEXT,
                last_message_time TEXT
            )",
            [],
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id TEXT,
                chat_jid TEXT,
                sender TEXT,
                sender_name TEXT,
                content TEXT,
                timestamp TEXT,
                is_from_me INTEGER DEFAULT 0,
                PRIMARY KEY (id, chat_jid)
            )",
            [],
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS scheduled_tasks (
                id TEXT PRIMARY KEY,
                group_folder TEXT NOT NULL,
                chat_jid TEXT NOT NULL,
                prompt TEXT NOT NULL,
                schedule_type TEXT NOT NULL,
                schedule_value TEXT NOT NULL,
                next_run TEXT,
                last_run TEXT,
                last_result TEXT,
                status TEXT DEFAULT 'active',
                created_at TEXT NOT NULL,
                context_mode TEXT DEFAULT 'isolated'
            )",
            [],
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS task_run_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                task_id TEXT NOT NULL,
                run_at TEXT NOT NULL,
                duration_ms INTEGER NOT NULL,
                status TEXT NOT NULL,
                result TEXT,
                error TEXT
            )",
            [],
        )?;

        Ok(Database {
            connection: Mutex::new(connection),
        })
    }
}
