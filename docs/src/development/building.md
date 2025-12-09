# Building

This page documents how to build Part-DB for development and production.

## Development Build

### Backend

```bash
# Debug build (faster compile, slower runtime)
cargo build

# Run directly
cargo run
```

### Frontend

```bash
cd frontend

# Development server with hot reload
npm run dev
```

## Production Build

### Backend

```bash
# Release build (optimized)
cargo build --release

# The binary is in target/release/
./target/release/backend
```

### Frontend

```bash
cd frontend

# Production build
npm run build

# Start production server
npm start
```

## Build Artifacts

### Backend

```
target/
├── debug/
│   └── backend           # Debug binary
└── release/
    └── backend           # Release binary
```

### Frontend

```
frontend/
├── .next/                # Next.js build output
│   ├── static/          # Static assets
│   └── server/          # Server-side code
└── out/                  # Static export (if used)
```

## Docker Build

### Using Docker Compose

```bash
docker-compose build
docker-compose up
```

### Building Individual Images

```bash
# Backend
docker build -t partdb-backend .

# Frontend
docker build -t partdb-frontend ./frontend
```

### Multi-stage Build

The Dockerfile uses multi-stage builds for smaller images:

```dockerfile
# Build stage
FROM rust:1.75 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/backend /usr/local/bin/
CMD ["backend"]
```

## Build Configuration

### Cargo Profile

Configure build profiles in `Cargo.toml`:

```toml
[profile.release]
lto = true
codegen-units = 1
opt-level = 3
```

### Next.js Configuration

Configure Next.js in `next.config.js`:

```javascript
module.exports = {
  output: 'standalone',  // For Docker builds
  experimental: {
    optimizeCss: true,
  },
};
```

## Build Optimization

### Rust

- **LTO (Link-Time Optimization)** - Smaller, faster binaries
- **Single codegen unit** - Better optimization, slower compile
- **Strip symbols** - Smaller binary size

```toml
[profile.release]
lto = "thin"
strip = true
```

### Frontend

- **Tree shaking** - Remove unused code (automatic with Next.js)
- **Minification** - Compress JavaScript/CSS
- **Image optimization** - Next.js automatic image optimization

## Build Times

Approximate build times:

| Build | Backend | Frontend |
|-------|---------|----------|
| Debug | ~30s | ~5s |
| Release | ~2min | ~30s |
| Docker | ~5min | ~2min |

### Speeding Up Builds

#### Rust

```bash
# Use sccache for caching
cargo install sccache
export RUSTC_WRAPPER=sccache

# Use mold linker (Linux)
sudo apt install mold
RUSTFLAGS="-C link-arg=-fuse-ld=mold" cargo build
```

#### Frontend

```bash
# Use npm cache
npm ci  # Clean install from lock file

# Turbo mode (if using Turborepo)
npx turbo build
```

## CI Build

GitHub Actions build workflow:

```yaml
- name: Build Backend
  run: cargo build --release

- name: Build Frontend
  working-directory: frontend
  run: |
    npm ci
    npm run build
```

See [GitHub Actions](./github-actions.md) for complete CI/CD configuration.
