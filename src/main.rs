//! NuClaw - Personal Claude Assistant in Rust
//!
//! A Rust implementation of NanoClaw project structure.

mod config;
mod db;
mod error;
mod types;
mod utils;
mod whatsapp;

pub use config::{ensure_directories, project_root, store_dir};
pub use error::{NuClawError, Result};
pub use types::{ContainerInput, ContainerOutput, NewMessage, RegisteredGroup, RouterState, Session};

use structopt::StructOpt;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(short, long, default_value = "Andy")]
    name: String,

    #[structopt(short, long, default_value = "info")]
    log_level: String,

    #[structopt(long)]
    auth: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Starting NuClaw v1.0.0");
    info!("This is a Rust port of NanoClaw");

    ensure_directories().map_err(|e| crate::error::NuClawError::FileSystem {
        message: e.to_string()
    })?;

    // Initialize database
    let _db = db::Database::new().map_err(|e| crate::error::NuClawError::Database {
        message: e.to_string()
    })?;
    info!("Database initialized successfully");

    // Placeholder for full implementation
    info!("Full implementation pending");

    Ok(())
}
