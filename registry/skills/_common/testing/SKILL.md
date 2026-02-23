---
name: testing
description: Testing patterns, strategies, and best practices across languages. Use when writing or improving tests.
---

# Testing Patterns

## Quick Reference

| Language | Test Command | Coverage |
|----------|--------------|----------|
| Rust | `cargo test` | `cargo tarpaulin` |
| TypeScript | `npm test` | `npm run coverage` |
| Python | `pytest` | `pytest --cov` |
| Go | `go test ./...` | `go test -cover ./...` |

## Test Structure

### Arrange-Act-Assert (AAA)

```rust
#[test]
fn should_add_numbers() {
    // Arrange
    let a = 2;
    let b = 3;
    
    // Act
    let result = add(a, b);
    
    // Assert
    assert_eq!(result, 5);
}
```

### Given-When-Then (BDD)

```rust
#[test]
fn given_valid_user_when_login_then_returns_token() {
    // Given
    let user = User::new("test@example.com", "password");
    
    // When
    let result = auth.login(&user);
    
    // Then
    assert!(result.is_ok());
    assert!(!result.unwrap().token.is_empty());
}
```

## Naming Conventions

```
should_[action]_when_[condition]
test_[function]_[scenario]_[expected]
```

**Examples:**
- `should_return_error_when_input_is_empty`
- `test_parse_valid_json_returns_object`
- `should_retry_on_transient_failure`

## Test Types

### Unit Tests

```rust
// Test single function in isolation
#[test]
fn parse_valid_id() {
    let id = BrunieId::parse("abc123").unwrap();
    assert_eq!(id.to_string(), "abc123");
}
```

### Integration Tests

```rust
// Test multiple components together
#[test]
fn conductor_executes_chain() {
    let conductor = Conductor::new();
    let result = conductor
        .chain(|_| async { Ok(1) })
        .then(|_, n| async move { Ok(n + 1) })
        .execute()
        .block();
    
    assert_eq!(result.unwrap(), 2);
}
```

### Property-Based Tests

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn parse_roundtrip(id: u128) {
        let original = BrunieId(id);
        let parsed = BrunieId::parse(&original.to_string()).unwrap();
        prop_assert_eq!(original, parsed);
    }
}
```

## Mocking Patterns

### Trait-Based Mocking

```rust
// Define trait
trait Storage {
    fn get(&self, key: &str) -> Option<String>;
}

// Real implementation
struct RedisStorage { /* ... */ }
impl Storage for RedisStorage { /* ... */ }

// Mock implementation
struct MockStorage {
    data: HashMap<String, String>,
}
impl Storage for MockStorage { /* ... */ }

// Use in tests
#[test]
fn test_with_mock() {
    let mock = MockStorage::new();
    mock.set("key", "value");
    
    let service = Service::new(mock);
    assert_eq!(service.fetch("key"), Some("value"));
}
```

### Dependency Injection

```rust
struct Service<S: Storage> {
    storage: S,
}

impl<S: Storage> Service<S> {
    fn new(storage: S) -> Self {
        Self { storage }
    }
}
```

## Async Testing

### Rust

```rust
#[tokio::test]
async fn async_operation_completes() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### TypeScript

```typescript
test('async operation', async () => {
    const result = await asyncFunction();
    expect(result).toBeDefined();
});
```

## Test Organization

```
src/
├── lib.rs
├── parser.rs
└── parser_test.rs    # Unit tests next to source

tests/
├── integration/      # Integration tests
│   ├── api_test.rs
│   └── db_test.rs
└── e2e/              # End-to-end tests
    └── workflow_test.rs
```

## Coverage Targets

| Type | Target |
|------|--------|
| Unit | 80%+ |
| Integration | 60%+ |
| Critical paths | 100% |

## Anti-patterns

❌ Testing implementation details
❌ Flaky tests (time-dependent, order-dependent)
❌ No assertions (test does nothing)
❌ Testing framework internals
❌ Excessive mocking (test means nothing)
❌ Ignoring edge cases