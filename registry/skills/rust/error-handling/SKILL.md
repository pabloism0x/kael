---
name: rust-error-handling
description: Error handling patterns with Result, custom errors, and error propagation. Use when designing error types.
---

# Rust Error Handling

## Quick Reference

| Pattern | Use Case |
|---------|----------|
| `Result<T, E>` | Recoverable errors |
| `Option<T>` | Missing values |
| `?` operator | Error propagation |
| `thiserror` | Library errors |
| `anyhow` | Application errors |

## Error Type Design

### Using thiserror (Libraries)

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KaelError {
    #[error("context not found: {0}")]
    ContextNotFound(String),
    
    #[error("operation timed out after {duration:?}")]
    Timeout { duration: Duration },
    
    #[error("chain execution failed at stage {stage}")]
    ChainFailed {
        stage: usize,
        #[source]
        cause: Box<dyn std::error::Error + Send + Sync>,
    },
    
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    
    #[error("invalid configuration: {0}")]
    Config(String),
}
```

### Using anyhow (Applications)

```rust
use anyhow::{Context, Result, bail, ensure};

fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .context("failed to read config file")?;
    
    let config: Config = toml::from_str(&content)
        .context("failed to parse config")?;
    
    ensure!(config.timeout > 0, "timeout must be positive");
    
    if config.workers == 0 {
        bail!("workers cannot be zero");
    }
    
    Ok(config)
}
```

## Error Propagation

### The ? Operator

```rust
fn process() -> Result<Data, Error> {
    let input = read_input()?;      // Propagates error
    let parsed = parse(input)?;      // Propagates error
    let result = transform(parsed)?; // Propagates error
    Ok(result)
}
```

### Adding Context

```rust
use anyhow::Context;

fn load_user(id: u64) -> Result<User> {
    let path = format!("/users/{}.json", id);
    
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read user file: {}", path))?;
    
    serde_json::from_str(&content)
        .with_context(|| format!("failed to parse user {}", id))
}
```

## Error Categories

### Transient vs Fatal

```rust
#[derive(Error, Debug)]
pub enum Error {
    // Transient - can retry
    #[error("connection timeout")]
    Timeout,
    
    #[error("rate limited")]
    RateLimited { retry_after: Duration },
    
    // Fatal - cannot recover
    #[error("invalid input: {0}")]
    InvalidInput(String),
    
    #[error("not found: {0}")]
    NotFound(String),
}

impl Error {
    pub fn is_transient(&self) -> bool {
        matches!(self, Error::Timeout | Error::RateLimited { .. })
    }
    
    pub fn is_fatal(&self) -> bool {
        !self.is_transient()
    }
}
```

### Error Codes

```rust
#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    NotFound = 404,
    Timeout = 408,
    Conflict = 409,
    Internal = 500,
}

impl Error {
    pub fn code(&self) -> ErrorCode {
        match self {
            Error::NotFound(_) => ErrorCode::NotFound,
            Error::Timeout => ErrorCode::Timeout,
            _ => ErrorCode::Internal,
        }
    }
}
```

## Result Combinators

### Mapping

```rust
// Map success value
let result: Result<i32, Error> = Ok(5);
let doubled = result.map(|n| n * 2);  // Ok(10)

// Map error
let result: Result<i32, &str> = Err("error");
let mapped = result.map_err(|e| Error::new(e));

// Map both
result.map_or_else(
    |e| default_for_error(e),
    |v| transform(v)
)
```

### Chaining

```rust
fn process() -> Result<Output, Error> {
    input()
        .and_then(|i| parse(i))
        .and_then(|p| validate(p))
        .and_then(|v| transform(v))
}
```

### Unwrapping Safely

```rust
// With default
let value = result.unwrap_or(default);
let value = result.unwrap_or_else(|| compute_default());

// Panicking (only in tests or truly unrecoverable)
let value = result.expect("critical invariant violated");
```

## Option Handling

```rust
fn find_user(id: u64) -> Option<User> {
    users.get(&id).cloned()
}

// Convert to Result
let user = find_user(id).ok_or(Error::NotFound(id))?;

// With context
let user = find_user(id)
    .ok_or_else(|| anyhow!("user {} not found", id))?;
```

## Async Error Handling

```rust
async fn fetch_data() -> Result<Data, Error> {
    let response = client.get(url).await
        .map_err(Error::Network)?;
    
    if !response.status().is_success() {
        return Err(Error::HttpStatus(response.status()));
    }
    
    response.json().await
        .map_err(Error::Parse)
}
```

## Testing Errors

```rust
#[test]
fn test_error_case() {
    let result = process_invalid_input();
    
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        Error::InvalidInput(_)
    ));
}

#[test]
fn test_error_message() {
    let err = Error::NotFound("user".into());
    assert_eq!(err.to_string(), "not found: user");
}
```

## Anti-patterns

❌ Using unwrap() in production code
❌ Ignoring errors with `let _ = ...`
❌ Generic "something went wrong" messages
❌ Panic for recoverable errors
❌ String as error type
❌ Not implementing std::error::Error