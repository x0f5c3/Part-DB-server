# Manual Installation

This guide covers installing Part-DB manually without Docker. This is useful for development or custom deployment scenarios.

## Prerequisites

### Install Rust

Install Rust using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Verify the installation:

```bash
rustc --version
cargo --version
```

### Install Node.js

Install Node.js 18+ (using nvm is recommended):

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20
nvm use 20
```

### Set Up Database

#### PostgreSQL (Recommended)

```bash
# Ubuntu/Debian
sudo apt install postgresql postgresql-contrib

# Start PostgreSQL
sudo systemctl start postgresql

# Create database and user
sudo -u postgres psql
CREATE DATABASE partdb;
CREATE USER partdb WITH PASSWORD 'your-secure-password';
GRANT ALL PRIVILEGES ON DATABASE partdb TO partdb;
\q
```

#### SQLite (Simple Setup)

SQLite requires no setup - just specify a file path in the configuration.

## Installation Steps

### 1. Clone the Repository

```bash
git clone https://github.com/x0f5c3/Part-DB-server.git
cd Part-DB-server
```

### 2. Configure Environment

Create a `.env.local` file:

```bash
cp .env .env.local
```

Edit `.env.local`:

```env
# Database
DATABASE_URL=postgres://partdb:your-secure-password@localhost/partdb

# Or for SQLite:
# DATABASE_URL=sqlite:./data/partdb.db

# Server
HOST=127.0.0.1
PORT=3000

# Connection pool
DB_MAX_CONNECTIONS=10
DB_CONNECT_TIMEOUT=30
```

### 3. Build the Backend

```bash
# Development build
cargo build

# Production build (optimized)
cargo build --release
```

### 4. Build the Frontend

```bash
cd frontend
npm install
npm run build
cd ..
```

### 5. Run Migrations

If you're using a database that requires migrations:

```bash
# This will be handled automatically on first run
cargo run
```

### 6. Start the Application

Development mode:

```bash
# Terminal 1: Backend
cargo run

# Terminal 2: Frontend
cd frontend
npm run dev
```

Production mode:

```bash
# Backend
./target/release/backend &

# Frontend
cd frontend
npm start
```

## Directory Structure After Installation

```
Part-DB-server/
├── backend/           # Rust backend source
│   ├── src/
│   └── target/        # Build output
├── frontend/          # Next.js frontend source
│   ├── src/
│   ├── .next/         # Build output
│   └── node_modules/
├── .env.local         # Your configuration
└── data/              # SQLite database (if used)
```

## Accessing Part-DB

After starting both backend and frontend:

- **Backend API**: http://localhost:3000
- **Frontend UI**: http://localhost:3001

## Troubleshooting

### Database Connection Issues

Verify your `DATABASE_URL` is correct:

```bash
# Test PostgreSQL connection
psql $DATABASE_URL -c "SELECT 1"
```

### Build Failures

Ensure all dependencies are installed:

```bash
# Rust dependencies
rustup update
cargo update

# Node dependencies
cd frontend
rm -rf node_modules
npm install
```

### Port Already in Use

Change the port in `.env.local`:

```env
PORT=3001
```

See [Configuration](./configuration.md) for all available options.
