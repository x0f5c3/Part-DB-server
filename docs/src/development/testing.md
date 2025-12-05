# Testing

This page documents how to test Part-DB.

## Backend Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_health_check

# Run tests matching pattern
cargo test api_
```

### Test Structure

Tests are located in:

- `backend/tests/` - Integration tests
- `backend/src/` - Unit tests (in same file as code)

### Example Tests

#### Unit Test

```rust
// In src/models.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_creation() {
        let part = Part {
            id: 1,
            name: "Test Part".to_string(),
            description: None,
            quantity: 10,
            // ...
        };
        
        assert_eq!(part.name, "Test Part");
        assert_eq!(part.quantity, 10);
    }
}
```

#### Integration Test

```rust
// In tests/api_tests.rs
use axum::http::StatusCode;
use axum_test::TestServer;

#[tokio::test]
async fn test_health_check() {
    let app = create_test_app().await;
    let server = TestServer::new(app).unwrap();
    
    let response = server.get("/health").await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_list_parts() {
    let app = create_test_app().await;
    let server = TestServer::new(app).unwrap();
    
    let response = server
        .get("/api/parts")
        .add_header("Authorization", "Bearer test-token")
        .await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
}
```

### Test Database

For tests requiring a database:

```rust
async fn create_test_db() -> PgPool {
    let url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or("postgres://test:test@localhost/partdb_test".to_string());
    
    PgPool::connect(&url).await.unwrap()
}
```

## Frontend Testing

### Running Tests

```bash
cd frontend

# Run tests
npm test

# Run with coverage
npm test -- --coverage

# Run in watch mode
npm test -- --watch
```

### Test Structure

Tests are co-located with components:

```
frontend/src/
├── components/
│   └── ui/
│       ├── button.tsx
│       └── button.test.tsx
└── lib/
    ├── api.ts
    └── api.test.ts
```

### Example Tests

#### Component Test

```tsx
// button.test.tsx
import { render, screen } from '@testing-library/react';
import { Button } from './button';

describe('Button', () => {
  it('renders with text', () => {
    render(<Button>Click me</Button>);
    expect(screen.getByText('Click me')).toBeInTheDocument();
  });

  it('handles click events', () => {
    const handleClick = jest.fn();
    render(<Button onClick={handleClick}>Click me</Button>);
    screen.getByText('Click me').click();
    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it('can be disabled', () => {
    render(<Button disabled>Click me</Button>);
    expect(screen.getByText('Click me')).toBeDisabled();
  });
});
```

#### API Test

```tsx
// api.test.ts
import { api } from './api';

// Mock fetch
global.fetch = jest.fn();

describe('API', () => {
  beforeEach(() => {
    (fetch as jest.Mock).mockClear();
  });

  it('fetches parts', async () => {
    const mockParts = { items: [], total: 0, page: 1, per_page: 30, total_pages: 0 };
    (fetch as jest.Mock).mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockParts),
    });

    const result = await api.listParts();
    
    expect(fetch).toHaveBeenCalledWith('/api/parts?page=1&per_page=30', expect.any(Object));
    expect(result).toEqual(mockParts);
  });

  it('handles errors', async () => {
    (fetch as jest.Mock).mockResolvedValue({
      ok: false,
      json: () => Promise.resolve({ message: 'Not found' }),
    });

    await expect(api.getPart(999)).rejects.toThrow('Not found');
  });
});
```

## Test Coverage

### Backend Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

### Frontend Coverage

```bash
cd frontend
npm test -- --coverage
```

Coverage reports are generated in:
- Backend: `target/tarpaulin/`
- Frontend: `frontend/coverage/`

## CI Testing

Tests run automatically on GitHub Actions:

```yaml
# Backend tests
- name: Run tests
  run: cargo test --all

# Frontend tests
- name: Run tests
  working-directory: frontend
  run: npm test -- --coverage
```

## Test Configuration

### Backend (Cargo.toml)

```toml
[dev-dependencies]
axum-test = "16"
tokio-test = "0.4"
```

### Frontend (jest.config.js)

```javascript
module.exports = {
  testEnvironment: 'jsdom',
  setupFilesAfterEnv: ['<rootDir>/jest.setup.js'],
  moduleNameMapper: {
    '^@/(.*)$': '<rootDir>/src/$1',
  },
};
```

## Best Practices

1. **Test behavior, not implementation** - Focus on what the code does
2. **Use descriptive names** - Tests serve as documentation
3. **Keep tests independent** - Each test should run in isolation
4. **Mock external dependencies** - API calls, database, etc.
5. **Test edge cases** - Empty inputs, errors, boundaries
