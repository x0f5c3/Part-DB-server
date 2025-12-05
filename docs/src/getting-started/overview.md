# Overview

Part-DB is a modern inventory management system for electronic components. This section will help you get started with installing and configuring Part-DB.

## System Requirements

### Backend Requirements

- **Rust 1.70+** - Install via [rustup](https://rustup.rs/)
- **PostgreSQL 14+** or **MySQL 5.7+** or **SQLite 3** for the database

### Frontend Requirements

- **Node.js 18+** with npm
- Modern web browser

### Optional

- **Docker** for containerized deployment
- **Reverse proxy** (nginx, Caddy, etc.) for production use

## Quick Start

The fastest way to get Part-DB running is with Docker:

```bash
docker-compose up -d
```

Or for a manual installation:

1. Clone the repository
2. Set up the database
3. Build the backend: `cargo build --release`
4. Build the frontend: `cd frontend && npm install && npm run build`
5. Configure environment variables
6. Start the application

See the detailed [Installation Guide](./installation.md) for step-by-step instructions.

## Next Steps

After installation:

1. Configure Part-DB via environment variables
2. Create your first user account
3. Start adding categories and parts
4. Explore the [API documentation](../api/overview.md) for integration options
