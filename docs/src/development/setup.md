# Development Setup

This guide covers setting up a development environment for Part-DB.

## Prerequisites

- **Git** - Version control
- **Rust 1.70+** - Backend development
- **Node.js 18+** - Frontend development
- **PostgreSQL 14+** - Database (or SQLite for simple setups)
- **VS Code** or another editor with Rust/TypeScript support

## Clone the Repository

```bash
git clone https://github.com/x0f5c3/Part-DB-server.git
cd Part-DB-server
```

## Backend Setup

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Install Development Tools

```bash
# Format checker
rustup component add rustfmt

# Linter
rustup component add clippy
```

### Configure Database

Create a `.env.local` file:

```env
DATABASE_URL=postgres://partdb:password@localhost/partdb_dev
HOST=127.0.0.1
PORT=3000
```

Or use SQLite for simpler setup:

```env
DATABASE_URL=sqlite:./dev.db
HOST=127.0.0.1
PORT=3000
```

### Run the Backend

```bash
# Development mode (with auto-reload via cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Or without auto-reload
cargo run
```

The backend will be available at http://localhost:3000

## Frontend Setup

### Install Dependencies

```bash
cd frontend
npm install
```

### Run the Frontend

```bash
npm run dev
```

The frontend will be available at http://localhost:3001

## Editor Setup

### VS Code Extensions

Recommended extensions for development:

- **rust-analyzer** - Rust language support
- **ESLint** - JavaScript/TypeScript linting
- **Prettier** - Code formatting
- **Tailwind CSS IntelliSense** - Tailwind autocomplete
- **Error Lens** - Inline error display

### VS Code Settings

Add to `.vscode/settings.json`:

```json
{
  "[rust]": {
    "editor.formatOnSave": true
  },
  "[typescript]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[typescriptreact]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "rust-analyzer.checkOnSave.command": "clippy"
}
```

## Development Workflow

### 1. Make Changes

Edit the code in `backend/` or `frontend/`.

### 2. Format Code

```bash
# Rust
cargo fmt

# Frontend
cd frontend && npm run lint
```

### 3. Run Tests

```bash
# Rust
cargo test

# Frontend
cd frontend && npm test
```

### 4. Check for Issues

```bash
# Rust linting
cargo clippy

# TypeScript checking
cd frontend && npx tsc --noEmit
```

### 5. Commit Changes

```bash
git add .
git commit -m "Description of changes"
git push
```

## Project Structure

```
Part-DB-server/
├── backend/           # Rust backend
│   ├── src/          # Source code
│   └── tests/        # Integration tests
├── frontend/         # Next.js frontend
│   └── src/          # Source code
├── docs/             # Documentation (mdbook)
└── .github/          # CI/CD workflows
```

## Environment Variables

### Backend

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | Database connection string | Required |
| `HOST` | Server bind address | `0.0.0.0` |
| `PORT` | Server port | `3000` |
| `JWT_SECRET` | JWT signing secret | Auto-generated |

### Frontend

| Variable | Description | Default |
|----------|-------------|---------|
| `NEXT_PUBLIC_API_URL` | Backend API URL | (proxy to localhost:3000) |

## Database Setup

### PostgreSQL

```bash
# Create database
createdb partdb_dev

# Set up user
psql -c "CREATE USER partdb WITH PASSWORD 'password';"
psql -c "GRANT ALL ON DATABASE partdb_dev TO partdb;"
```

### SQLite

No setup required - just specify a file path in `DATABASE_URL`.

## Troubleshooting

### Rust Compilation Errors

```bash
# Clean and rebuild
cargo clean
cargo build
```

### Node.js Issues

```bash
# Clean and reinstall
rm -rf node_modules
npm install
```

### Database Connection Issues

```bash
# Test PostgreSQL connection
psql $DATABASE_URL -c "SELECT 1"
```
