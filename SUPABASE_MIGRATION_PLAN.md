# Part-DB: Supabase-First Migration Plan

**Audience:** Developers with strong Rust/Go knowledge and minimal SQL experience.  
**Goal:** Deliver a fully functional Part-DB replacement faster by leveraging
Supabase (managed PostgreSQL + Auth + REST) and keeping the Rust Axum backend
as thin business-logic middleware.

---

## Why Supabase?

| Problem | Supabase solution |
|---------|------------------|
| Writing raw SQL migrations | Supabase Dashboard visual table editor + auto-generated PostgREST API |
| Self-managed authentication | Supabase Auth with OAuth2, magic-link, SAML, and TOTP built-in |
| Token management & refresh | Supabase JS/Rust client handles it transparently |
| Real-time inventory updates | Supabase Realtime (Postgres logical replication) |
| File/image attachments | Supabase Storage (S3-compatible) |
| Local development | `supabase` CLI spins up a full local stack with one command |

Supabase IS PostgreSQL — so you keep all of Postgres's power without needing
to write a single migration file if you don't want to (use the Dashboard
instead).

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                          Browser / Client                        │
│                                                                  │
│  Next.js 14 (React + Tailwind + shadcn/ui)                       │
│  ┌─────────────────┐   ┌────────────────────────────────────┐   │
│  │  @supabase/ssr  │   │  Zustand auth store                │   │
│  │  (session mgmt) │   │  (SSO + magic-link + password)     │   │
│  └────────┬────────┘   └───────────────┬────────────────────┘   │
└───────────┼───────────────────────────┼────────────────────────-┘
            │                           │
            │ JWT in cookie/header      │ REST calls
            ▼                           ▼
┌───────────────────────┐   ┌──────────────────────────────────┐
│   Supabase Auth       │   │   Rust Axum backend              │
│   (Google, GitHub,    │   │   (business logic, validation)   │
│    Magic-link, SAML)  │   │   Validates Supabase JWTs        │
└───────────┬───────────┘   └───────────┬──────────────────────┘
            │                           │
            │ session JWT               │ sqlx queries (PostgreSQL)
            ▼                           ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Supabase Cloud                           │
│  ┌─────────────────┐  ┌──────────────┐  ┌──────────────────┐   │
│  │  PostgreSQL DB   │  │  PostgREST   │  │  Realtime /       │   │
│  │  (managed,       │  │  (auto REST  │  │  Storage          │   │
│  │  point-in-time   │  │  from schema)│  │                   │   │
│  │  backups)        │  │              │  │                   │   │
│  └─────────────────┘  └──────────────┘  └──────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### Key Design Decisions

1. **Supabase Auth is the single source of truth for identity.**  
   The Rust backend validates Supabase-issued JWTs (HS256, shared secret) —
   no custom token generation is needed in production.

2. **The Rust backend uses the Supabase PostgreSQL connection string.**  
   No schema changes required beyond what's already in `backend/migrations/`.
   Use the Supabase Dashboard to view and edit data visually.

3. **The frontend calls the Rust backend for all business operations.**  
   The Supabase JS client is only used for auth flows.  Data mutations go
   through the Axum API (which can add validation, audit logs, etc.).

4. **Row-Level Security (RLS) is enforced at the database level.**  
   Supabase RLS policies protect data even if the API layer has a bug.

---

## Getting Started in < 30 Minutes

### 1. Create a Supabase project

1. Go to [supabase.com](https://supabase.com) and sign in.
2. Click **New project**, choose a region, set a strong DB password.
3. Wait ~1 minute for provisioning.

### 2. Apply the database schema

Option A — Supabase Dashboard (no SQL required):
- Use the **Table Editor** to create tables visually.
- Follow the schema in `backend/DATABASE_SCHEMA.md`.

Option B — SQL editor (copy-paste):
```sql
-- Paste the contents of backend/migrations/0001_init.sql
-- into the Supabase SQL Editor and click Run
```

### 3. Configure OAuth providers

In Supabase Dashboard → Authentication → Providers:
- **Google** — add your Google OAuth client ID + secret.
- **GitHub** — add your GitHub OAuth app credentials.
- Save redirect URL `https://<your-domain>/auth/callback` in each provider.

### 4. Copy environment variables

**Backend** (`backend/.env`):
```bash
cp backend/.env.supabase.example backend/.env
# Fill in DATABASE_URL and SUPABASE_JWT_SECRET from Supabase Dashboard → Settings → API
```

**Frontend** (`frontend/.env.local`):
```bash
cp frontend/.env.local.example frontend/.env.local
# Fill in VITE_SUPABASE_URL and VITE_SUPABASE_ANON_KEY
```

### 5. Run locally

```bash
# Terminal 1 — Rust backend
cd backend && cargo run

# Terminal 2 — Vite frontend
cd frontend && npm install && npm run dev
```

Open http://localhost:4000 and click **Sign in with Google** or **GitHub**.

---

## Authentication Flow

### OAuth SSO (Google / GitHub)

```
User clicks "Sign in with Google"
        │
        ▼
signInWithOAuth("google") in auth store
        │  (Supabase JS redirects browser)
        ▼
accounts.google.com → user approves
        │  (Google redirects to Supabase)
        ▼
Supabase exchanges code, issues JWT
        │  (Supabase redirects to /auth/callback?code=...)
        ▼
/auth/callback/route.ts  →  exchangeCodeForSession(code)
        │  (sets session cookie)
        ▼
Redirect to original page
```

### API requests after login

```
Frontend sends:  Authorization: Bearer <supabase-jwt>
                                        │
                                        ▼
                 Rust backend  →  validate_token()
                                  reads SUPABASE_JWT_SECRET
                                  verifies HS256 signature + expiry
                                        │
                                        ▼
                                  extracts sub (UUID) + email
                                  looks up local users table
                                        │
                                        ▼
                                  returns data
```

---

## Enabling Row-Level Security (Recommended)

Add these policies in the Supabase Dashboard → Authentication → Policies:

```sql
-- Allow any authenticated user to read all parts
CREATE POLICY "authenticated read parts"
  ON parts FOR SELECT
  TO authenticated
  USING (true);

-- Allow any authenticated user to insert parts
CREATE POLICY "authenticated insert parts"
  ON parts FOR INSERT
  TO authenticated
  WITH CHECK (true);
```

RLS provides a second defence layer — even if the Rust API is bypassed, the
database enforces access control.

---

## Supabase Storage for Attachments

Part-DB supports file/image attachments.  Use Supabase Storage:

```typescript
// Upload an attachment
const { data, error } = await supabase.storage
  .from("attachments")
  .upload(`parts/${partId}/${file.name}`, file);

// Get public URL
const { data: { publicUrl } } = supabase.storage
  .from("attachments")
  .getPublicUrl(`parts/${partId}/${file.name}`);
```

No S3 bucket or server file system required.

---

## Supabase Realtime (Live Inventory Updates)

Subscribe to stock level changes without polling:

```typescript
const channel = supabase
  .channel("part-lots-changes")
  .on(
    "postgres_changes",
    { event: "UPDATE", schema: "public", table: "part_lots" },
    (payload) => {
      // Update local state with new stock levels
      updatePartLot(payload.new);
    }
  )
  .subscribe();
```

---

## Local Development with Supabase CLI

For fully offline development:

```bash
# Install Supabase CLI
brew install supabase/tap/supabase   # or see https://supabase.com/docs/guides/cli

# Start local Supabase stack (PostgreSQL + Auth + Studio)
supabase start

# Apply migrations
supabase db push

# Access local Studio at http://localhost:54323
```

The local stack uses the same config as production — no surprises when you deploy.

---

## Migration Phases

### Phase 1: Foundation (Complete ✅)
- [x] Rust Axum backend with PostgreSQL (SQLX)
- [x] Database schema validated against PHP entities
- [x] Parts, Categories, Footprints, Manufacturers, StorageLocations, Suppliers API
- [x] JWT auth middleware
- [x] React + Vite frontend scaffold with shadcn/ui

### Phase 2: Supabase Integration (This PR ✅)
- [x] Supabase JWT validation in Rust backend (accepts both Supabase and local JWTs)
- [x] UUID-based user lookup (Supabase `sub` claim)
- [x] `@supabase/supabase-js` browser client in React + Vite SPA
- [x] Supabase browser client (`frontend/src/lib/supabase.ts`)
- [x] Client-only Supabase architecture (no dedicated frontend server client)
- [x] SSO login page (Google + GitHub + magic-link + password)
- [x] OAuth callback handling in frontend auth flow
- [x] `AuthProvider` for session initialisation
- [x] `ProtectedRoute` wrapper component
- [x] Auth-aware navbar on home page
- [x] Configuration example files

### Phase 3: Complete CRUD + UI (Next)
- [ ] Complete CRUD for Footprints, Manufacturers, StorageLocations, Suppliers
- [ ] Real-time stock level display using Supabase Realtime
- [ ] File attachment support via Supabase Storage
- [ ] Parts page wired to live API (remove placeholder data)
- [ ] Categories tree view
- [ ] Storage locations hierarchical browser

### Phase 4: Advanced Auth + Permissions
- [ ] User roles synced from Supabase Auth metadata
- [ ] Row-Level Security policies for multi-tenant support
- [ ] SAML/OIDC enterprise SSO (Supabase supports this via custom SAML provider)
- [ ] Audit log for inventory changes

### Phase 5: Removal of PHP Legacy
- [ ] Snapshot test to confirm API parity
- [ ] Remove PHP/Symfony source tree
- [ ] Update Dockerfile to only run Rust + Next.js

---

## FAQ

**Q: Do I need to write SQL at all?**  
A: No. The Supabase Dashboard's Table Editor lets you create tables, add
columns, set types, and define foreign keys through a UI.  The SQL migration
files in `backend/migrations/` are available as a reference but are not
required — paste them into the SQL editor or use the visual editor.

**Q: Can I use SurrealDB or MongoDB instead?**  
A: Yes. The Rust backend uses SQLX which only targets PostgreSQL.  To switch
to SurrealDB, replace `backend/src/db.rs` with a SurrealDB client and rewrite
the query functions in `backend/src/routes.rs`.  The API surface and models
stay the same.  Supabase is recommended for the fastest time-to-working-product
because of its built-in auth and Dashboard.

**Q: How do I configure Google/GitHub OAuth?**  
A: See the [Supabase Auth documentation](https://supabase.com/docs/guides/auth/social-login).
The short version: create an OAuth app in Google Cloud Console / GitHub
Developer Settings, copy the client ID and secret into Supabase Dashboard →
Authentication → Providers, and add `https://<your-domain>/auth/callback` as
an allowed redirect URL.

**Q: Is Supabase free?**  
A: The Supabase free tier includes 500 MB database, 1 GB file storage, 50 000
monthly active users, and unlimited API requests.  More than enough for
personal or small-team use.

**Q: Can I self-host Supabase?**  
A: Yes — `supabase start` runs a full local Supabase stack via Docker.  The
same Docker images can be used for self-hosted production deployments.

---

## Reference: Supabase Resources

- Dashboard: https://app.supabase.com
- Docs: https://supabase.com/docs
- Auth guide: https://supabase.com/docs/guides/auth
- Storage guide: https://supabase.com/docs/guides/storage
- Realtime guide: https://supabase.com/docs/guides/realtime
- CLI reference: https://supabase.com/docs/reference/cli
- Rust client (`postgrest-rs`): https://github.com/supabase-community/postgrest-rs
