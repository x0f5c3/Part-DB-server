# Contributing

We welcome contributions to Part-DB! This guide explains how to contribute.

## Ways to Contribute

- **Bug Reports** - Report issues you find
- **Feature Requests** - Suggest new features
- **Documentation** - Improve docs
- **Code** - Fix bugs or implement features
- **Testing** - Add or improve tests

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Set up the development environment (see [Development Setup](./setup.md))
4. Create a branch for your changes
5. Make your changes
6. Submit a pull request

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/my-new-feature
# or
git checkout -b fix/bug-description
```

### 2. Make Changes

Follow the coding standards for the project:

- **Rust**: Follow `rustfmt` style
- **TypeScript**: Follow ESLint rules
- Write tests for new functionality
- Update documentation as needed

### 3. Run Checks

```bash
# Rust formatting and linting
cargo fmt --check
cargo clippy

# Rust tests
cargo test

# Frontend linting and tests
cd frontend
npm run lint
npm test
```

### 4. Commit Changes

Write clear commit messages:

```bash
git commit -m "Add feature: description of what was added"
git commit -m "Fix: description of what was fixed"
git commit -m "Docs: update installation guide"
```

### 5. Push and Create PR

```bash
git push origin feature/my-new-feature
```

Then create a pull request on GitHub.

## Code Standards

### Rust

- Run `cargo fmt` before committing
- Fix all `clippy` warnings
- Add doc comments for public APIs
- Write tests for new functionality

```rust
/// Creates a new part in the database.
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `part` - Part creation data
///
/// # Returns
///
/// The created part with generated ID
pub async fn create_part(
    pool: &PgPool,
    part: CreatePart,
) -> Result<Part, AppError> {
    // Implementation
}
```

### TypeScript

- Run ESLint before committing
- Use TypeScript types, avoid `any`
- Use functional components
- Keep components small and focused

```tsx
interface PartCardProps {
  part: Part;
  onEdit?: (id: number) => void;
}

export function PartCard({ part, onEdit }: PartCardProps) {
  return (
    <Card>
      <CardTitle>{part.name}</CardTitle>
      {onEdit && <Button onClick={() => onEdit(part.id)}>Edit</Button>}
    </Card>
  );
}
```

## Pull Request Guidelines

### Title

Use a clear, descriptive title:

- `feat: add barcode scanning support`
- `fix: correct pagination calculation`
- `docs: improve API documentation`

### Description

Include in your PR description:

- What changes were made
- Why the changes are needed
- How to test the changes
- Screenshots (for UI changes)

### Checklist

- [ ] Code follows project style guidelines
- [ ] Tests pass locally
- [ ] Documentation updated if needed
- [ ] No breaking changes (or documented if unavoidable)

## Issue Guidelines

### Bug Reports

Include:

- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment (OS, browser, versions)
- Error messages or logs

### Feature Requests

Include:

- Use case / problem to solve
- Proposed solution
- Alternatives considered

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Help others learn
- Focus on what's best for the project

## Questions?

- Open a GitHub issue for questions
- Join discussions on existing issues
- Check the documentation first

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (AGPL-3.0).
