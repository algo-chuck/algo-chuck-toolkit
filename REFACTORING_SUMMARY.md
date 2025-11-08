# HTTP Client Refactoring Summary

## Overview

Successfully refactored `schwab-api-core` to support pluggable HTTP clients (both sync and async) from multiple libraries, enabling users to bring their own HTTP client implementation.

## Changes Made

### 1. Core Architecture Refactoring

#### `crates/schwab-api-core/src/lib.rs`

- **Renamed**: `AsyncClient` → `AsyncHttpClient` for clarity
- **Added**: New `SyncHttpClient` trait for blocking/sync HTTP clients
- **Extracted**: `parse_api_error()` from trait to standalone helper function
  - Removed Schwab-specific logic from HTTP client trait
  - Now available as `pub fn parse_api_error(status, body_text) -> SchwabError`
- **Added**: Module declarations for feature-gated client implementations

#### New Trait Definitions

```rust
/// Trait for async HTTP clients
pub trait AsyncHttpClient: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    async fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error>;
}

/// Trait for sync/blocking HTTP clients
pub trait SyncHttpClient: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error>;
}
```

### 2. Modular Client Implementations

#### `crates/schwab-api-core/src/reqwest_client.rs` (NEW)

- **Feature-gated**: `#[cfg(feature = "reqwest-client")]`
- **Implementations**:
  - `AsyncHttpClient` for `reqwest::Client` (owned)
  - `AsyncHttpClient` for `&reqwest::Client` (borrowed)
  - `AsyncHttpClient` for `Arc<reqwest::Client>` (shared ownership)
- **Helper**: `execute_with_reqwest()` shared by all implementations
- **Error handling**: Uses `crate::parse_api_error()` for API errors

#### `crates/schwab-api-core/src/ureq_client.rs` (NEW)

- **Feature-gated**: `#[cfg(feature = "ureq-client")]`
- **Implementations**:
  - `SyncHttpClient` for `ureq::Agent` (owned)
  - `SyncHttpClient` for `&ureq::Agent` (borrowed)
  - `SyncHttpClient` for `Arc<ureq::Agent>` (shared ownership)
- **Helper**: `execute_with_ureq()` shared by all implementations
- **Error handling**: Handles both `ureq::Error::Status` and `ureq::Error::Transport`

### 3. Dependency Management

#### `crates/schwab-api-core/Cargo.toml`

```toml
[features]
default = ["reqwest-client"]
reqwest-client = ["dep:reqwest"]
ureq-client = ["dep:ureq"]

[dependencies]
# Core dependencies (always included)
schwab-api-types = { path = "../schwab-api-types" }
serde = { workspace = true, features = ["derive", "std"] }
serde_json = { workspace = true, features = ["std"] }
thiserror = { workspace = true }
async-trait = { workspace = true }
http = { workspace = true, features = ["std"] }

# Optional HTTP client implementations
reqwest = { workspace = true, optional = true }
ureq = { version = "2.10", optional = true }
```

**Key changes**:

- `reqwest` and `ureq` are now optional dependencies
- Default feature enables `reqwest-client` for backwards compatibility
- Added `std` features to `serde`, `serde_json`, and `http` for proper no-default-features builds
- Removed `tokio` and `url` as direct dependencies (not needed in core)

### 4. Dependent Crate Updates

#### `crates/schwab-api-trader/src/asynchronous.rs`

- Changed `use schwab_api_core::AsyncClient` → `use schwab_api_core::AsyncHttpClient`
- Changed bound `C: AsyncClient` → `C: AsyncHttpClient`

#### `crates/schwab-api-marketdata/src/asynchronous.rs`

- Changed `use schwab_api_core::AsyncClient` → `use schwab_api_core::AsyncHttpClient`
- Changed bound `C: AsyncClient` → `C: AsyncHttpClient`

### 5. Documentation & Examples

#### `examples/custom_clients.rs` (NEW)

Demonstrates:

- Using reqwest with owned, borrowed, and Arc-wrapped clients
- Server-style shared client usage
- Custom HTTP client implementation by implementing `AsyncHttpClient`

## Benefits

### 1. Flexibility

- **User choice**: Users can bring their own HTTP client (sync or async)
- **Multiple patterns**: Supports owned, borrowed, and Arc-wrapped clients
- **Custom implementations**: Easy to implement custom clients with logging, retry logic, etc.

### 2. Modularity

- **Clean separation**: HTTP execution logic separate from API-specific parsing
- **Feature gates**: Only compile what you need
- **No unnecessary dependencies**: Core crate doesn't force specific HTTP client

### 3. Server-Friendly

- **Shared clients**: Borrowed and Arc patterns perfect for web servers
- **Zero-copy**: Reference-based clients avoid cloning
- **Thread-safe**: Arc enables safe sharing across threads

### 4. Future-Proof

- **Extensible**: Easy to add more HTTP clients (e.g., `curl`, `hyper` directly)
- **Sync support**: Foundation laid for sync client wrappers (TraderClient, MarketdataClient)
- **Custom errors**: Each client can have its own error type

## Testing

All feature combinations verified to compile:

```bash
# Default (reqwest only)
cargo build

# Ureq only
cargo build --no-default-features --features ureq-client

# Both clients
cargo build --all-features

# Entire workspace
cargo build --all-features
```

## Migration Guide (for future users)

### If using `AsyncClient` directly:

```rust
// Before
use schwab_api_core::AsyncClient;
fn my_func<C: AsyncClient>(client: C) { }

// After
use schwab_api_core::AsyncHttpClient;
fn my_func<C: AsyncHttpClient>(client: C) { }
```

### For custom client implementations:

The `parse_api_error` function is no longer part of the trait. If you were overriding it, move that logic to your error handling:

```rust
// Before
impl AsyncClient for MyClient {
    fn parse_api_error(...) -> SchwabError { /* custom logic */ }
}

// After
impl AsyncHttpClient for MyClient {
    async fn execute(&self, req: Request<String>) -> Result<Response<String>, Self::Error> {
        let resp = /* make request */;
        if !resp.status().is_success() {
            // Use helper or custom logic
            let err = schwab_api_core::parse_api_error(resp.status(), resp.body());
            return Err(HttpError::Api(err));
        }
        Ok(resp)
    }
}
```

## Next Steps (Future Work)

1. **Sync Client Wrappers**: Create `SyncTraderClient` and `SyncMarketdataClient` that use `SyncHttpClient`
2. **Additional Clients**: Add support for other popular HTTP clients (e.g., `curl`, `surf`, `isahc`)
3. **Retry Logic**: Example showing custom client with automatic retries
4. **Rate Limiting**: Example showing custom client with rate limiting
5. **Middleware Pattern**: Consider middleware/interceptor pattern for cross-cutting concerns

## Files Changed

### New Files

- `crates/schwab-api-core/src/reqwest_client.rs`
- `crates/schwab-api-core/src/ureq_client.rs`
- `examples/custom_clients.rs`

### Modified Files

- `crates/schwab-api-core/src/lib.rs`
- `crates/schwab-api-core/Cargo.toml`
- `crates/schwab-api-trader/src/asynchronous.rs`
- `crates/schwab-api-marketdata/src/asynchronous.rs`

## Breaking Changes

✅ **No concern** - User explicitly stated no existing users besides testing CLI

- Renamed `AsyncClient` to `AsyncHttpClient`
- Moved `parse_api_error` from trait method to standalone function
- Changed module structure (implementation details now in separate modules)

All changes compile successfully and the CLI continues to work!
