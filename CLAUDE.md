# Challenges Server

> **IMPORTANT:** For general project context, read this file and linked docs.
> Only explore source files when actively implementing, planning, or debugging.

## Overview

A self-hostable Rust/Axum HTTP API server for ham radio challenge tracking. Enables operators to track progress toward awards (DXCC, WAS, POTA milestones) with leaderboards and time-limited competitions. The official FullDuplex challenges server is one deployment of this codebase.

## Quick Reference

| Area | Description | Details |
|------|-------------|---------|
| Architecture | Server structure, database, auth flow | [docs/architecture.md](docs/architecture.md) |
| API | Public and admin endpoints | [docs/api.md](docs/api.md) |
| Challenges | Challenge types, scoring, tiers | [docs/features/challenges.md](docs/features/challenges.md) |
| Leaderboards | Ranking, snapshots, queries | [docs/features/leaderboards.md](docs/features/leaderboards.md) |
| Auth | Device tokens, callsign verification | [docs/features/auth.md](docs/features/auth.md) |

## Code Standards

- **Maximum file size: 1000 lines.** Refactor when approaching this limit.
- Use `thiserror` for error types with `IntoResponse` impl
- All database queries via `sqlx` with compile-time checking
- Handlers return `Result<Json<T>, AppError>`
- Configuration via environment variables only

## Building and Running

**NEVER build, run tests, or run the server yourself. Always prompt the user to do so.**

When you need to verify changes compile or tests pass, ask the user to run the appropriate command and report back the results.

### Commands

```bash
# Development
cargo build                    # Build the project
cargo test                     # Run tests
cargo run                      # Run server (requires DATABASE_URL)

# Database
sqlx database create           # Create database
sqlx migrate run               # Run migrations

# Docker
docker compose up -d           # Start Postgres + server
docker compose down            # Stop services
```

### Environment Variables

```bash
DATABASE_URL=postgres://user:pass@localhost:5432/challenges  # Required
ADMIN_TOKEN=your-secret-token                                 # Required
PORT=8080                                                     # Optional, default 8080
BASE_URL=https://challenges.example.com                       # Optional
RUST_LOG=info                                                 # Optional
```

## Project Structure

```
challenges-server/
├── migrations/           # SQL migrations
├── src/
│   ├── main.rs          # Entry point
│   ├── config.rs        # Env var parsing
│   ├── error.rs         # AppError type
│   ├── auth/            # Token middleware
│   ├── db/              # Database queries
│   ├── models/          # Data structures
│   ├── handlers/        # HTTP handlers
│   ├── scoring/         # Score calculation
│   └── middleware/      # Rate limiting, admin auth
└── docs/
    ├── architecture.md
    ├── api.md
    └── features/
```
