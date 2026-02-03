//! Core types for NuClaw

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredGroup {
    pub name: String,
    pub folder: String,
    pub trigger: String,
    pub added_at: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Session(HashMap<String, String>);

impl Session {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: String,
    pub group_folder: String,
    pub chat_jid: String,
    pub prompt: String,
    pub schedule_type: String,
    pub schedule_value: String,
    pub context_mode: String,
    pub next_run: Option<String>,
    pub last_run: Option<String>,
    pub last_result: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRunLog {
    pub task_id: String,
    pub run_at: String,
    pub duration_ms: i64,
    pub status: String,
    pub result: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMessage {
    pub id: String,
    pub chat_jid: String,
    pub sender: String,
    pub sender_name: String,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatInfo {
    pub jid: String,
    pub name: String,
    pub last_message_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInput {
    pub prompt: String,
    pub session_id: Option<String>,
    pub group_folder: String,
    pub chat_jid: String,
    pub is_main: bool,
    pub is_scheduled_task: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerOutput {
    pub status: String,
    pub result: Option<String>,
    pub new_session_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RouterState {
    pub last_timestamp: String,
    pub last_agent_timestamp: HashMap<String, String>,
}
