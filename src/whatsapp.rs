//! WhatsApp integration for NuClaw

use crate::config::{data_dir, store_dir};
use crate::error::Result;
use crate::types::{RegisteredGroup, RouterState, Session};
use crate::utils::json::{load_json, save_json};
use std::collections::HashMap;
use std::process::Command;
use tracing::debug;

pub fn load_router_state() -> RouterState {
    let state_path = data_dir().join("router_state.json");
    load_json(&state_path, RouterState { last_timestamp: String::new(), last_agent_timestamp: HashMap::new() })
}

pub fn load_registered_groups() -> HashMap<String, RegisteredGroup> {
    let path = data_dir().join("registered_groups.json");
    load_json(&path, HashMap::new())
}

pub fn load_sessions() -> Session {
    let path = data_dir().join("sessions.json");
    load_json(&path, Session::new())
}

pub fn ensure_container_system_running() -> Result<()> {
    let output = Command::new("container").args(&["system", "status"]).output();

    match output {
        Ok(_) => { debug!("Container system already running"); Ok(()) }
        Err(_) => {
            debug!("Starting container system...");
            let output = Command::new("container").args(&["system", "start"]).output();
            match output {
                Ok(_) => { debug!("Container system started"); Ok(()) }
                Err(e) => Err(crate::error::NuClawError::Container {
                    message: format!("Failed to start container system: {}", e)
                })
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum AuthState {
    WaitingForScan,
    Connected,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct AuthResult {
    pub state: AuthState,
    pub error: Option<String>,
}

pub async fn start_auth_flow() -> AuthResult {
    let auth_path = store_dir().join("auth");
    std::fs::create_dir_all(&auth_path).ok();

    AuthResult {
        state: AuthState::Error("Not implemented".to_string()),
        error: Some("WhatsApp Web authentication pending implementation".to_string()),
    }
}
