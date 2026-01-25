# Challenges Server

A self-hostable Rust/Axum HTTP API server for ham radio challenge tracking. Enables operators to track progress toward awards (DXCC, WAS, POTA milestones) with leaderboards and time-limited competitions.

## Features

- **Challenge Management**: Create and manage collection, cumulative, and time-bounded challenges
- **Progress Tracking**: Track participant progress toward challenge goals
- **Leaderboards**: Real-time leaderboards with configurable scoring methods
- **Tiered Awards**: Define progression tiers with badge rewards
- **Device Authentication**: Secure device token-based authentication
- **Admin Interface**: Web-based configurator for challenge management

## Quick Start

### Using Docker Compose (Recommended)

```bash
# Start the server and database
docker compose up -d

# The server will be available at http://localhost:8081
# Default admin token: dev-admin-token
```

### Manual Setup

#### Prerequisites

- Rust 1.75+
- PostgreSQL 16+
- Node.js 20+ (for building the web frontend)

#### Build and Run

```bash
# Build the frontend
cd web && npm ci && npm run build && cd ..

# Set up environment
export DATABASE_URL=postgres://user:pass@localhost:5432/challenges
export ADMIN_TOKEN=your-secret-token

# Run database migrations
sqlx database create
sqlx migrate run

# Build and run the server
cargo build --release
./target/release/challenges-server
```

## Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `ADMIN_TOKEN` | Token for admin API access | Required |
| `PORT` | Server port | `8080` |
| `BASE_URL` | Public URL for invite links | Optional |
| `RUST_LOG` | Log level | `info` |

## API Overview

### Public Endpoints

- `GET /v1/challenges` - List all challenges
- `GET /v1/challenges/:id` - Get challenge details
- `POST /v1/challenges/:id/join` - Join a challenge
- `GET /v1/challenges/:id/leaderboard` - Get leaderboard

### Authenticated Endpoints

- `POST /v1/challenges/:id/progress` - Report progress
- `GET /v1/challenges/:id/progress` - Get own progress
- `DELETE /v1/challenges/:id/leave` - Leave a challenge

### Admin Endpoints

- `POST /v1/admin/challenges` - Create challenge
- `PUT /v1/admin/challenges/:id` - Update challenge
- `DELETE /v1/admin/challenges/:id` - Delete challenge
- `POST /v1/admin/challenges/:id/badges` - Upload badge
- `POST /v1/admin/challenges/:id/invites` - Generate invite

## Challenge Types

### Collection
Work a set of specific items (e.g., all 50 US states for WAS, 100+ DXCC entities).

### Cumulative
Reach a target value (e.g., 1000 QSOs, 500 miles per watt).

### Time-Bounded
Complete objectives within a time period (e.g., contest-style events).

## Development

```bash
# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Build frontend in development mode
cd web && npm run dev
```

## License

MIT License - see [LICENSE](LICENSE) for details.
