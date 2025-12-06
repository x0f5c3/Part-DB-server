# GitHub Actions

Part-DB uses GitHub Actions for continuous integration and deployment.

## Workflows

### Backend CI (`rust.yml`)

Runs on changes to Rust code:

```yaml
name: Rust Backend CI

on:
  push:
    paths: ['backend/**', 'Cargo.toml', 'Cargo.lock']
  pull_request:
    paths: ['backend/**', 'Cargo.toml', 'Cargo.lock']

jobs:
  fmt:
    name: Rustfmt
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --all -- --check

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      - run: cargo clippy --all-targets --all-features -- -D warnings

  test:
    name: Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      - uses: dtolnay/rust-toolchain@stable
      - uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-test-${{ hashFiles('**/Cargo.lock') }}
      - run: cargo test --all

  build:
    name: Build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      - uses: dtolnay/rust-toolchain@stable
      - uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-build-${{ hashFiles('**/Cargo.lock') }}
      - run: cargo build --release
```

### Frontend CI (`frontend.yml`)

Runs on changes to frontend code:

```yaml
name: Frontend CI

on:
  push:
    paths: ['frontend/**']
  pull_request:
    paths: ['frontend/**']

jobs:
  lint:
    name: ESLint
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: frontend
    steps:
      - uses: actions/checkout@v5
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - run: npm install
      - run: npm run lint

  typecheck:
    name: TypeScript
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: frontend
    steps:
      - uses: actions/checkout@v5
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - run: npm install
      - run: npx tsc --noEmit

  build:
    name: Build
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: frontend
    steps:
      - uses: actions/checkout@v5
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - run: npm install
      - run: npm run build
```

### Documentation (`docs.yml`)

Builds and deploys documentation to GitHub Pages:

```yaml
name: Deploy Documentation

on:
  push:
    branches: [master]
    paths: ['docs/**']
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      
      - name: Setup mdBook
        uses: peaceiris/actions-mdbook@v2
        with:
          mdbook-version: 'latest'
      
      - name: Build documentation
        run: mdbook build docs
      
      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: docs/book

  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

## Workflow Triggers

| Trigger | Description |
|---------|-------------|
| `push` | Runs on push to specified branches |
| `pull_request` | Runs on pull requests |
| `workflow_dispatch` | Manual trigger from GitHub UI |
| `schedule` | Runs on a cron schedule |

## Caching

Workflows use caching to speed up builds:

```yaml
- uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    restore-keys: |
      ${{ runner.os }}-cargo-
```

## Secrets

Required secrets for deployment workflows:

| Secret | Description |
|--------|-------------|
| `GITHUB_TOKEN` | Automatic, for GitHub API access |

For Docker deployment (if added):

| Secret | Description |
|--------|-------------|
| `DOCKER_USERNAME` | Docker Hub username |
| `DOCKER_PASSWORD` | Docker Hub password or token |

## Branch Protection

Recommended branch protection settings for `master`:

- Require pull request reviews
- Require status checks to pass
- Require branches to be up to date

## Adding New Workflows

1. Create a new file in `.github/workflows/`
2. Define triggers, jobs, and steps
3. Test on a feature branch first
4. Merge to enable the workflow
