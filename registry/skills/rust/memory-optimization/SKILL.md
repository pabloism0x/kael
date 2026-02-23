---
name: rust-memory-optimization
description: Memory optimization patterns for Rust including allocators, zero-copy, and cache efficiency. Use when optimizing memory usage.
---

# Rust Memory Optimization

## Quick Reference

| Technique | Impact | Complexity |
|-----------|--------|------------|
| Stack allocation | High | Low |
| Slab allocator | High | Medium |
| Bump allocator | High | Low |
| Zero-copy | High | Medium |
| Cache alignment | Medium | Low |

## Allocation Strategies

### Stack Over Heap

```rust
// ❌ Heap allocation
let data = Box::new([0u8; 1024]);

// ✅ Stack allocation (if size is known and reasonable)
let data = [0u8; 1024];

// ✅ Small vec optimization
use smallvec::SmallVec;
let vec: SmallVec<[u8; 64]> = SmallVec::new();
```

### Slab Allocator

```rust
use slab::Slab;

struct TaskPool {
    tasks: Slab<Task>,
}

impl TaskPool {
    fn new(capacity: usize) -> Self {
        Self {
            tasks: Slab::with_capacity(capacity),
        }
    }
    
    fn insert(&mut self, task: Task) -> usize {
        self.tasks.insert(task)  // Returns index, O(1)
    }
    
    fn remove(&mut self, key: usize) -> Task {
        self.tasks.remove(key)   // O(1)
    }
}
```

### Bump Allocator

```rust
use bumpalo::Bump;

fn process_batch(items: &[Item]) -> Vec<&Result> {
    let bump = Bump::new();
    
    items.iter()
        .map(|item| {
            // All allocations freed when bump is dropped
            bump.alloc(process(item))
        })
        .collect()
}
```

### Arena Allocation

```rust
use typed_arena::Arena;

struct Parser<'a> {
    arena: &'a Arena<Node>,
}

impl<'a> Parser<'a> {
    fn parse(&self, input: &str) -> &'a Node {
        let node = self.arena.alloc(Node::new(input));
        // node lives as long as arena
        node
    }
}
```

## Zero-Copy Patterns

### Borrowing Over Cloning

```rust
// ❌ Clones data
fn process(data: String) -> String {
    data.to_uppercase()
}

// ✅ Borrows data
fn process(data: &str) -> String {
    data.to_uppercase()
}
```

### Cow (Clone on Write)

```rust
use std::borrow::Cow;

fn process(input: &str) -> Cow<str> {
    if needs_modification(input) {
        Cow::Owned(modify(input))
    } else {
        Cow::Borrowed(input)  // No allocation
    }
}
```

### Bytes and BytesMut

```rust
use bytes::{Bytes, BytesMut};

// Zero-copy slicing
let data = Bytes::from_static(b"hello world");
let hello = data.slice(0..5);  // No copy, reference counted

// Efficient buffer building
let mut buf = BytesMut::with_capacity(1024);
buf.put_slice(b"hello");
buf.put_u32(42);
let frozen = buf.freeze();  // Convert to Bytes
```

## Cache Optimization

### Cache Line Alignment

```rust
#[repr(align(64))]  // Typical cache line size
struct CacheAligned {
    data: [u8; 64],
}

// Avoid false sharing in concurrent code
#[repr(align(64))]
struct PaddedCounter {
    value: AtomicU64,
    _padding: [u8; 56],
}
```

### Struct Layout

```rust
// ❌ Poor layout (padding waste)
struct Bad {
    a: u8,   // 1 byte + 7 padding
    b: u64,  // 8 bytes
    c: u8,   // 1 byte + 7 padding
}  // Total: 24 bytes

// ✅ Optimized layout
struct Good {
    b: u64,  // 8 bytes
    a: u8,   // 1 byte
    c: u8,   // 1 byte + 6 padding
}  // Total: 16 bytes
```

### Data-Oriented Design

```rust
// ❌ Array of Structs (AoS) - poor cache locality
struct Particle {
    x: f32, y: f32, z: f32,
    vx: f32, vy: f32, vz: f32,
}
let particles: Vec<Particle> = vec![];

// ✅ Struct of Arrays (SoA) - better cache locality
struct Particles {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
    vx: Vec<f32>,
    vy: Vec<f32>,
    vz: Vec<f32>,
}
```

## Memory Reuse

### Object Pooling

```rust
struct Pool<T> {
    items: Vec<T>,
    free: Vec<usize>,
}

impl<T: Default> Pool<T> {
    fn acquire(&mut self) -> &mut T {
        if let Some(idx) = self.free.pop() {
            &mut self.items[idx]
        } else {
            self.items.push(T::default());
            self.items.last_mut().unwrap()
        }
    }
    
    fn release(&mut self, idx: usize) {
        self.free.push(idx);
    }
}
```

### Buffer Reuse

```rust
struct BufferPool {
    buffers: Vec<Vec<u8>>,
}

impl BufferPool {
    fn get(&mut self, size: usize) -> Vec<u8> {
        self.buffers.pop()
            .map(|mut b| { b.clear(); b.reserve(size); b })
            .unwrap_or_else(|| Vec::with_capacity(size))
    }
    
    fn put(&mut self, mut buffer: Vec<u8>) {
        buffer.clear();
        self.buffers.push(buffer);
    }
}
```

## Compact Representations

### Bit Packing

```rust
// ❌ Wasteful
struct Flags {
    a: bool,  // 1 byte each
    b: bool,
    c: bool,
    d: bool,
}  // 4 bytes

// ✅ Compact
use bitflags::bitflags;

bitflags! {
    struct Flags: u8 {
        const A = 0b0001;
        const B = 0b0010;
        const C = 0b0100;
        const D = 0b1000;
    }
}  // 1 byte
```

### Small String Optimization

```rust
use compact_str::CompactString;

// Strings <= 24 bytes stored inline (no heap)
let s: CompactString = "hello".into();
```

## Profiling Tools

```bash
# Memory usage
cargo build --release
valgrind --tool=massif ./target/release/app
ms_print massif.out.*

# Heap profiling
MALLOC_CONF=prof:true ./target/release/app
jeprof --pdf ./target/release/app jeprof.*.heap > heap.pdf

# Allocation tracking
cargo install cargo-alloc
cargo alloc
```

## Anti-patterns

❌ Unnecessary cloning
❌ Unbounded buffers
❌ Ignoring struct layout
❌ Heap allocation in hot loops
❌ Not reusing allocations