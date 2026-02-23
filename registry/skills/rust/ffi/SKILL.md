---
name: rust-ffi
description: Rust FFI patterns for C bindings, Python (PyO3), and Node.js (napi-rs). Use when creating language bindings.
---

# Rust FFI Patterns

## Quick Reference

| Target | Crate | Build Output |
|--------|-------|--------------|
| C/C++ | `libc` | `.so`, `.dylib`, `.dll` |
| Python | `pyo3`, `maturin` | `.so` (wheel) |
| Node.js | `napi-rs` | `.node` |
| WebAssembly | `wasm-bindgen` | `.wasm` |

## C FFI Basics

### Cargo.toml

```toml
[lib]
name = "mylib"
crate-type = ["cdylib", "staticlib"]

[dependencies]
libc = "0.2"
```

### Exporting Functions

```rust
// src/lib.rs
use std::ffi::{c_char, c_int, CStr, CString};
use std::ptr;

/// # Safety
/// `name` must be a valid null-terminated C string
#[no_mangle]
pub unsafe extern "C" fn greet(name: *const c_char) -> *mut c_char {
    if name.is_null() {
        return ptr::null_mut();
    }

    let name_str = match CStr::from_ptr(name).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let greeting = format!("Hello, {}!", name_str);

    match CString::new(greeting) {
        Ok(s) => s.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Free a string allocated by Rust
/// # Safety
/// `s` must be a string previously returned by Rust functions
#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

#[no_mangle]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}
```

### C Header (cbindgen)

```toml
# cbindgen.toml
language = "C"
include_guard = "MYLIB_H"
```

```bash
# Generate header
cbindgen --config cbindgen.toml --crate mylib --output include/mylib.h
```

Generated `mylib.h`:
```c
#ifndef MYLIB_H
#define MYLIB_H

#include <stdint.h>

char *greet(const char *name);
void free_string(char *s);
int32_t add(int32_t a, int32_t b);

#endif /* MYLIB_H */
```

### Using from C

```c
// main.c
#include <stdio.h>
#include "mylib.h"

int main() {
    char *greeting = greet("World");
    if (greeting) {
        printf("%s\n", greeting);
        free_string(greeting);
    }

    printf("1 + 2 = %d\n", add(1, 2));
    return 0;
}
```

## Opaque Types & Handles

```rust
// Opaque struct - C only sees pointer
pub struct Database {
    connection: Connection,
}

#[no_mangle]
pub extern "C" fn database_new(url: *const c_char) -> *mut Database {
    let url = unsafe { CStr::from_ptr(url).to_str().unwrap() };
    let db = Database {
        connection: Connection::new(url),
    };
    Box::into_raw(Box::new(db))
}

#[no_mangle]
pub unsafe extern "C" fn database_query(
    db: *mut Database,
    query: *const c_char,
) -> *mut c_char {
    let db = &*db;
    let query = CStr::from_ptr(query).to_str().unwrap();
    let result = db.connection.query(query);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn database_free(db: *mut Database) {
    if !db.is_null() {
        drop(Box::from_raw(db));
    }
}
```

## Error Handling

```rust
#[repr(C)]
pub struct FFIResult {
    data: *mut c_char,
    error: *mut c_char,
    success: bool,
}

impl FFIResult {
    fn ok(data: String) -> Self {
        Self {
            data: CString::new(data).unwrap().into_raw(),
            error: ptr::null_mut(),
            success: true,
        }
    }

    fn err(error: String) -> Self {
        Self {
            data: ptr::null_mut(),
            error: CString::new(error).unwrap().into_raw(),
            success: false,
        }
    }
}

#[no_mangle]
pub extern "C" fn process(input: *const c_char) -> FFIResult {
    let input = unsafe {
        match CStr::from_ptr(input).to_str() {
            Ok(s) => s,
            Err(e) => return FFIResult::err(e.to_string()),
        }
    };

    match do_process(input) {
        Ok(result) => FFIResult::ok(result),
        Err(e) => FFIResult::err(e.to_string()),
    }
}

#[no_mangle]
pub unsafe extern "C" fn ffi_result_free(result: FFIResult) {
    if !result.data.is_null() {
        drop(CString::from_raw(result.data));
    }
    if !result.error.is_null() {
        drop(CString::from_raw(result.error));
    }
}
```

## PyO3 (Python Bindings)

### Cargo.toml

```toml
[lib]
name = "mylib"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.21", features = ["extension-module"] }
```

### Python Module

```rust
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

/// Formats a greeting
#[pyfunction]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Process data with error handling
#[pyfunction]
fn process(data: Vec<i32>) -> PyResult<Vec<i32>> {
    if data.is_empty() {
        return Err(PyValueError::new_err("Data cannot be empty"));
    }
    Ok(data.iter().map(|x| x * 2).collect())
}

/// A Python class implemented in Rust
#[pyclass]
struct Counter {
    value: i64,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(initial: i64) -> Self {
        Counter { value: initial }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn decrement(&mut self) {
        self.value -= 1;
    }

    #[getter]
    fn value(&self) -> i64 {
        self.value
    }
}

/// Async function
#[pyfunction]
fn fetch_data<'py>(py: Python<'py>, url: String) -> PyResult<Bound<'py, PyAny>> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let response = reqwest::get(&url).await.map_err(|e| {
            PyValueError::new_err(e.to_string())
        })?;
        let text = response.text().await.map_err(|e| {
            PyValueError::new_err(e.to_string())
        })?;
        Ok(text)
    })
}

/// The Python module
#[pymodule]
fn mylib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    m.add_function(wrap_pyfunction!(process, m)?)?;
    m.add_class::<Counter>()?;
    Ok(())
}
```

### Build with Maturin

```bash
# Install maturin
pip install maturin

# Development build
maturin develop

# Build wheel
maturin build --release

# Publish to PyPI
maturin publish
```

### pyproject.toml

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "mylib"
version = "0.1.0"
requires-python = ">=3.8"

[tool.maturin]
features = ["pyo3/extension-module"]
```

## napi-rs (Node.js Bindings)

### Cargo.toml

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
napi = { version = "2", features = ["async", "serde-json"] }
napi-derive = "2"

[build-dependencies]
napi-build = "2"
```

### Node.js Module

```rust
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[napi]
fn sum(numbers: Vec<i32>) -> i32 {
    numbers.iter().sum()
}

#[napi]
struct Calculator {
    value: f64,
}

#[napi]
impl Calculator {
    #[napi(constructor)]
    pub fn new(initial: f64) -> Self {
        Calculator { value: initial }
    }

    #[napi]
    pub fn add(&mut self, n: f64) -> f64 {
        self.value += n;
        self.value
    }

    #[napi(getter)]
    pub fn value(&self) -> f64 {
        self.value
    }
}

// Async function
#[napi]
async fn fetch_data(url: String) -> Result<String> {
    let response = reqwest::get(&url)
        .await
        .map_err(|e| Error::from_reason(e.to_string()))?;
    let text = response
        .text()
        .await
        .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(text)
}

// Return complex types with serde
#[napi(object)]
struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[napi]
fn create_user(id: u32, name: String, email: String) -> User {
    User { id, name, email }
}
```

### Build

```bash
# Install napi-cli
npm install -g @napi-rs/cli

# Initialize project
napi new

# Build
npm run build

# Build for multiple platforms
napi build --platform --release
```

### package.json

```json
{
  "name": "mylib",
  "version": "0.1.0",
  "main": "index.js",
  "types": "index.d.ts",
  "napi": {
    "name": "mylib",
    "triples": {
      "defaults": true,
      "additional": ["aarch64-apple-darwin"]
    }
  },
  "scripts": {
    "build": "napi build --platform --release",
    "prepublishOnly": "napi prepublish -t npm"
  }
}
```

## Memory Safety Rules

### Critical Safety Guidelines

```rust
// NEVER: Return reference to local data
#[no_mangle]
pub extern "C" fn bad_return() -> *const c_char {
    let s = CString::new("hello").unwrap();
    s.as_ptr() // Dangling pointer!
}

// GOOD: Transfer ownership
#[no_mangle]
pub extern "C" fn good_return() -> *mut c_char {
    CString::new("hello").unwrap().into_raw()
}

// NEVER: Forget to provide free function
// GOOD: Always provide matching free
#[no_mangle]
pub unsafe extern "C" fn free_cstring(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

// Document safety requirements
/// # Safety
/// - `ptr` must be valid and non-null
/// - `ptr` must point to memory allocated by this library
/// - After this call, `ptr` is invalid and must not be used
#[no_mangle]
pub unsafe extern "C" fn free_data(ptr: *mut Data) { ... }
```

## Anti-patterns

### Avoid: Panicking Across FFI Boundary

```rust
// Bad: Panic crosses FFI boundary (undefined behavior)
#[no_mangle]
pub extern "C" fn bad_function(data: *const c_char) -> i32 {
    let s = unsafe { CStr::from_ptr(data).to_str().unwrap() }; // Panic!
    s.len() as i32
}

// Good: Catch panics
#[no_mangle]
pub extern "C" fn good_function(data: *const c_char) -> i32 {
    std::panic::catch_unwind(|| {
        let s = unsafe { CStr::from_ptr(data).to_str().ok()? };
        Some(s.len() as i32)
    })
    .ok()
    .flatten()
    .unwrap_or(-1)
}
```

### Avoid: Not Handling Null Pointers

```rust
// Bad: No null check
#[no_mangle]
pub unsafe extern "C" fn process(data: *const Data) {
    let data = &*data; // Crash if null
}

// Good: Validate input
#[no_mangle]
pub unsafe extern "C" fn process(data: *const Data) -> bool {
    if data.is_null() {
        return false;
    }
    let data = &*data;
    // ...
    true
}
```

## Testing FFI

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let name = CString::new("World").unwrap();
        let result = unsafe { greet(name.as_ptr()) };
        let result_str = unsafe { CStr::from_ptr(result).to_str().unwrap() };
        assert_eq!(result_str, "Hello, World!");
        unsafe { free_string(result) };
    }

    #[test]
    fn test_null_input() {
        let result = unsafe { greet(std::ptr::null()) };
        assert!(result.is_null());
    }
}
```
