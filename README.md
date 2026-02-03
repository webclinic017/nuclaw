# NuClaw - Personal Claude Assistant

A Rust implementation of NanoClaw - a personal Claude assistant that runs securely in isolated containers.

## Why Rust?

This project is a complete rewrite of [NanoClaw](https://github.com/gavrielc/nanoclaw) in Rust for:
- **Performance** - Faster startup and lower memory usage
- **Safety** - Memory safety and thread safety guarantees
- **Concurrency** - Better async handling for I/O operations

## Features

- **WhatsApp I/O** - Message Claude from your phone
- **Isolated group context** - Each group has its own memory and filesystem
- **Container isolation** - Agents run in Docker or Apple Container
- **Scheduled tasks** - Recurring jobs with cron expressions
- **Mount allowlist** - Secure additional mount validation

## Architecture

```
WhatsApp → SQLite → Scheduler → Container (Claude Agent SDK) → Response
```

### Core Components

- `src/main.rs` - Application entry point
- `src/whatsapp.rs` - WhatsApp Web connection
- `src/container_runner.rs` - Container management
- `src/task_scheduler.rs` - Scheduled task execution
- `src/db.rs` - SQLite database operations
- `src/mount_security.rs` - Mount validation

## Requirements

- Rust 1.70+
- Docker or Apple Container
- Node.js (for agent execution)
- Claude Code subscription

## Quick Start

```bash
# Build
cargo build --release

# Run with WhatsApp authentication
cargo run -- auth

# Start the service
cargo run
```

## Configuration

Environment variables:
- `ASSISTANT_NAME` - Trigger word (default: "Andy")
- `LOG_LEVEL` - Logging level (debug, info, warn, error)
- `CONTAINER_IMAGE` - Container image name
- `CONTAINER_TIMEOUT` - Agent execution timeout in ms
- `TZ` - Timezone for scheduled tasks

## Mount Allowlist

Additional mounts are configured in `~/.config/nuclaw/mount-allowlist.json`:

```json
{
  "allowedRoots": [
    {
      "path": "~/projects",
      "allowReadWrite": true,
      "description": "Development projects"
    }
  ],
  "blockedPatterns": ["password", "secret"],
  "nonMainReadOnly": true
}
```

## Development

```bash
# Run with debug logging
LOG_LEVEL=debug cargo run

# Run tests
cargo test

# Check code
cargo clippy
```

## License

MIT
