# Project Structure

Part-DB uses a modern monorepo structure with separate backend and frontend applications.

## Directory Layout

```
Part-DB-server/
├── backend/                 # Rust backend application
│   ├── Cargo.toml          # Rust dependencies
│   ├── src/
│   │   ├── main.rs         # Entry point & server setup
│   │   ├── lib.rs          # Library exports
│   │   ├── auth.rs         # Authentication logic
│   │   ├── db.rs           # Database connection pool
│   │   ├── error.rs        # Error types & handling
│   │   ├── models.rs       # Data models & DTOs
│   │   ├── openapi.rs      # OpenAPI documentation
│   │   └── routes.rs       # API route handlers
│   └── tests/
│       └── api_tests.rs    # Integration tests
│
├── frontend/               # React/Next.js frontend
│   ├── package.json        # Node.js dependencies
│   ├── next.config.js      # Next.js configuration
│   ├── tailwind.config.ts  # Tailwind CSS configuration
│   ├── tsconfig.json       # TypeScript configuration
│   └── src/
│       ├── app/            # Next.js App Router pages
│       │   ├── page.tsx    # Homepage
│       │   ├── layout.tsx  # Root layout
│       │   ├── globals.css # Global styles
│       │   ├── parts/      # Parts pages
│       │   ├── categories/ # Categories pages
│       │   └── storage/    # Storage pages
│       ├── components/
│       │   └── ui/         # shadcn/ui components
│       ├── lib/
│       │   └── utils.ts    # Utility functions
│       ├── store/          # State management
│       └── types/
│           └── api.ts      # API type definitions
│
├── docs/                   # Documentation (mdbook)
│   ├── book.toml          # mdbook configuration
│   └── src/               # Markdown source files
│
├── .github/
│   └── workflows/         # GitHub Actions CI/CD
│
├── Cargo.toml             # Workspace Cargo configuration
├── Cargo.lock             # Rust dependency lock file
├── package.json           # Root package.json
├── docker-compose.yml     # Docker Compose configuration
└── Dockerfile             # Docker build configuration
```

## Technology Stack

### Backend

| Component | Technology | Purpose |
|-----------|------------|---------|
| Framework | Axum 0.8 | HTTP server and routing |
| Database | SQLX 0.8 | Async database access |
| Runtime | Tokio | Async runtime |
| Serialization | Serde | JSON serialization |
| Auth | JWT | Token-based authentication |
| Docs | utoipa | OpenAPI specification |

### Frontend

| Component | Technology | Purpose |
|-----------|------------|---------|
| Framework | Next.js 14 | React framework |
| Language | TypeScript | Type safety |
| Styling | Tailwind CSS | Utility-first CSS |
| Components | shadcn/ui | UI component library |
| API Client | Fetch API | HTTP requests |

### Infrastructure

| Component | Technology | Purpose |
|-----------|------------|---------|
| CI/CD | GitHub Actions | Automated testing and deployment |
| Containers | Docker | Containerized deployment |
| Documentation | mdbook | Documentation site |

## Communication Flow

```
┌─────────────┐     HTTP/JSON     ┌─────────────┐
│   Browser   │ ◄───────────────► │  Frontend   │
└─────────────┘                   │  (Next.js)  │
                                  └──────┬──────┘
                                         │
                                    API calls
                                         │
                                  ┌──────▼──────┐
                                  │   Backend   │
                                  │   (Axum)    │
                                  └──────┬──────┘
                                         │
                                    SQL queries
                                         │
                                  ┌──────▼──────┐
                                  │  Database   │
                                  │ (PostgreSQL)│
                                  └─────────────┘
```

## Workspaces

The project uses Cargo workspaces for the Rust code:

```toml
# Cargo.toml (root)
[workspace]
members = ["backend"]
```

This allows for shared dependencies and unified building across multiple Rust crates if needed in the future.
