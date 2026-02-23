---
name: rust-async-patterns
description: Async Rust patterns for futures, executors, and concurrent programming. Use when working with async code.
---

# Rust Async Patterns

## Quick Reference

| Pattern | Use Case |
|---------|----------|
| `async fn` | Async function |
| `.await` | Wait for future |
| `join!` | Concurrent execution |
| `select!` | First completion |
| `spawn` | Background task |

## Future Basics

### Async Function

```rust
async fn fetch_data(url: &str) -> Result<Data, Error> {
    let response = client.get(url).await?;
    let data = response.json().await?;
    Ok(data)
}
```

### Manual Future

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture {
    state: State,
}

impl Future for MyFuture {
    type Output = i32;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state {
            State::Ready(value) => Poll::Ready(value),
            State::Pending => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}
```

## Concurrency Patterns

### Join (All Complete)

```rust
use futures::join;

async fn fetch_all() -> (User, Posts, Comments) {
    join!(
        fetch_user(),
        fetch_posts(),
        fetch_comments()
    )
}
```

### Select (First Completes)

```rust
use futures::select;

async fn fetch_with_timeout() -> Result<Data, Error> {
    select! {
        data = fetch_data().fuse() => Ok(data),
        _ = sleep(Duration::from_secs(5)).fuse() => Err(Error::Timeout),
    }
}
```

### Spawn (Background)

```rust
// Fire and forget
spawn(async move {
    process_in_background().await;
});

// With handle
let handle = spawn(async move {
    compute_result().await
});
let result = handle.await?;
```

## Cancellation

### Token-Based

```rust
struct CancellationToken {
    cancelled: AtomicBool,
}

impl CancellationToken {
    fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }
    
    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

async fn cancellable_task(token: &CancellationToken) {
    loop {
        if token.is_cancelled() {
            return;
        }
        do_work().await;
    }
}
```

### Drop-Based

```rust
struct TaskGuard {
    handle: JoinHandle<()>,
}

impl Drop for TaskGuard {
    fn drop(&mut self) {
        self.handle.abort();
    }
}
```

## Stream Patterns

### Async Iterator

```rust
use futures::Stream;

async fn process_stream<S: Stream<Item = Data>>(stream: S) {
    pin_mut!(stream);
    while let Some(item) = stream.next().await {
        process(item).await;
    }
}
```

### Buffered Concurrency

```rust
use futures::stream::StreamExt;

stream
    .map(|item| async move { process(item).await })
    .buffer_unordered(10)  // Max 10 concurrent
    .collect::<Vec<_>>()
    .await
```

## Error Handling

### Try Join

```rust
use futures::try_join;

async fn fetch_all() -> Result<(User, Posts), Error> {
    try_join!(
        fetch_user(),
        fetch_posts()
    )
}
```

### Error Recovery

```rust
async fn with_retry<T, F, Fut>(f: F, max_attempts: u32) -> Result<T, Error>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, Error>>,
{
    let mut attempts = 0;
    loop {
        match f().await {
            Ok(value) => return Ok(value),
            Err(e) if attempts < max_attempts => {
                attempts += 1;
                sleep(backoff(attempts)).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

## Pin and Unpin

```rust
// Self-referential future needs pinning
use std::pin::Pin;

async fn pinned_example() {
    let future = async { /* ... */ };
    pin_mut!(future);  // Pin to stack
    
    // Or heap-pinned
    let boxed: Pin<Box<dyn Future<Output = ()>>> = Box::pin(async { });
}
```

## Anti-patterns

❌ Blocking in async context

```rust
// Bad: blocks the executor
async fn bad() {
    std::thread::sleep(Duration::from_secs(1));
}

// Good: async sleep
async fn good() {
    sleep(Duration::from_secs(1)).await;
}
```

❌ Holding lock across await

```rust
// Bad: deadlock risk
async fn bad(mutex: &Mutex<Data>) {
    let guard = mutex.lock().unwrap();
    some_async_op().await;  // Lock held!
}

// Good: drop before await
async fn good(mutex: &Mutex<Data>) {
    let data = {
        let guard = mutex.lock().unwrap();
        guard.clone()
    };
    some_async_op().await;
}
```

❌ Unbounded spawning

```rust
// Bad: can exhaust resources
for item in items {
    spawn(process(item));
}

// Good: bounded concurrency
stream::iter(items)
    .map(|item| async move { process(item).await })
    .buffer_unordered(10)
    .collect()
    .await
```