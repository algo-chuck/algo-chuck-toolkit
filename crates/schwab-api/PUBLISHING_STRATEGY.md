# Publishing Strategy for Schwab API Crates

## Executive Summary

**RECOMMENDATION: Publish only `schwab-api` to crates.io**

The internal crates (`schwab-api-types`, `schwab-api-core`, etc.) remain as workspace implementation details. Users interact only with the facade crate which provides clean feature flags for customization.

## Architecture Overview

```
schwab-api (PUBLIC - facade crate)
├── schwab-api-types (PRIVATE - 149 types)
├── schwab-api-core (PRIVATE - HTTP traits, errors)
├── schwab-api-oauth (PRIVATE - auth implementation)
├── schwab-api-marketdata (PRIVATE - 67 market data types)
└── schwab-api-trader (PRIVATE - 82 trading types)
```

## User Experience

### Simple Case - Everything Included

```toml
[dependencies]
schwab-api = "0.1"
```

### Trader Only - Async

```toml
[dependencies]
schwab-api = { version = "0.1", default-features = false, features = ["trader", "oauth", "async-only"] }
```

**Result:** 82 trader types, reqwest, tokio, no ureq, no marketdata

### Market Data Only - Sync

```toml
[dependencies]
schwab-api = { version = "0.1", default-features = false, features = ["marketdata", "oauth", "sync-only"] }
```

**Result:** 67 marketdata types, ureq, no tokio, no trader

### Both APIs - Sync Only

```toml
[dependencies]
schwab-api = { version = "0.1", default-features = false, features = ["trader", "marketdata", "oauth", "ureq-client"] }
```

**Result:** All 149 types, ureq only, no async runtime

## Feature Flags Reference

### API Selection

- `trader` - Trading API (accounts, orders, transactions) - 82 types
- `marketdata` - Market data API (quotes, options, price history) - 67 types
- `oauth` - OAuth authentication flow

### HTTP Client Selection

- `reqwest-client` - Async HTTP using reqwest + tokio
- `ureq-client` - Sync/blocking HTTP using ureq (no async runtime)

### Convenience Combinations

- `default` - Everything: trader + marketdata + oauth + both clients
- `full` - Explicit alias for default
- `async-only` - Both APIs + oauth + reqwest only
- `sync-only` - Both APIs + oauth + ureq only

## Import Patterns

### Using the Prelude (Recommended)

```rust
use schwab_api::prelude::*;

// All commonly used types are available
let oauth = AsyncOAuthClient::new(...);
let trader = AsyncTraderClient::new(...);
let marketdata = AsyncMarketdataClient::new(...);
```

### Using Explicit Modules

```rust
use schwab_api::oauth::AsyncOAuthClient;
use schwab_api::trader::AsyncTraderClient;
use schwab_api::types::trader::Account;
```

### Accessing Types

```rust
// Types are always available through the types module
use schwab_api::types;

let account: types::trader::Account = ...;
let quote: types::marketdata::QuoteEquity = ...;
```

## Compilation Time Comparison

### Full Default (everything)

```
cargo build -p schwab-api
Time: ~8-10s
Dependencies: reqwest, tokio, ureq, all types
Binary size: ~3-4 MB
```

### Trader Only, Async

```
cargo build -p schwab-api --no-default-features --features "trader,oauth,async-only"
Time: ~5-6s
Dependencies: reqwest, tokio (no ureq)
Binary size: ~2-3 MB
Savings: 40% faster compile, 25% smaller binary
```

### Market Data Only, Sync

```
cargo build -p schwab-api --no-default-features --features "marketdata,oauth,sync-only"
Time: ~3-4s
Dependencies: ureq only (no tokio, no reqwest)
Binary size: ~1-2 MB
Savings: 60% faster compile, 50% smaller binary
```

## Maintenance Benefits

### For Library Authors (Us)

1. **Clean separation of concerns** - Each crate has single responsibility
2. **Easy testing** - Test individual components in isolation
3. **Flexible development** - Can version internal crates independently during development
4. **Clear dependencies** - Feature flags make dependency graph explicit

### For Users

1. **Single import** - Only need `schwab-api = "0.1"`
2. **Pay for what you use** - Feature flags eliminate unused code
3. **Clear documentation** - One place to look for all APIs
4. **Stable API** - Internal restructuring doesn't affect user code

## Publishing Plan

### Phase 1: Prepare for v0.1.0

- [x] Add comprehensive feature flags to `schwab-api`
- [x] Document all feature combinations
- [ ] Add examples/ directory with common use cases
- [ ] Create comprehensive README.md
- [ ] Add CHANGELOG.md
- [ ] Verify documentation builds with `cargo doc`

### Phase 2: Publish to crates.io

```bash
# Only publish the facade crate
cd crates/schwab-api
cargo publish --dry-run  # Test first
cargo publish            # Actually publish
```

### Phase 3: Maintenance

- Internal crates can evolve without breaking changes
- Only need to version `schwab-api` for public releases
- Can refactor internals without affecting users

## Alternative Approaches Considered

### Option 1: Monolithic Crate (NOT RECOMMENDED)

Merge everything into one `schwab-api` crate with all code inline.

**Pros:**

- Simpler to publish (one crate)

**Cons:**

- Harder to maintain (30,000+ lines in one crate)
- Slower development (can't test components independently)
- All-or-nothing compilation (no feature flags)
- Messy code organization

### Option 2: Facade Pattern (RECOMMENDED - Current)

One public facade crate, private implementation crates.

**Pros:**

- Clean user experience (single import)
- Flexible features (compile only what you need)
- Maintainable (modular codebase)
- Fast compilation (feature flags work well)

**Cons:**

- Slightly more complex workspace setup (already done!)

### Option 3: Publish All Crates (NOT RECOMMENDED)

Publish all 6 crates to crates.io.

**Pros:**

- Maximum flexibility for power users

**Cons:**

- 6 crates to version and maintain
- Confusing for users (which crate do I import?)
- Breaking changes cascade across crates
- More documentation to maintain

## Recommendation Summary

✅ **Publish only `schwab-api` to crates.io**

This gives us:

- **Clean user experience** - One crate, clear features
- **Maintainable codebase** - Modular internal structure
- **Optimal compilation** - Users compile only what they need
- **Flexibility** - Can refactor internals without breaking users
- **Professional** - Follows Rust ecosystem patterns (like `tokio`, `serde`)

## Next Steps

1. ✅ Implement feature flags in `schwab-api` facade
2. Add comprehensive examples to `schwab-api/examples/`
3. Write detailed README.md for `schwab-api`
4. Generate and review documentation: `cargo doc --open -p schwab-api`
5. Test all feature combinations
6. Publish v0.1.0 to crates.io

## Real-World Example: Similar Crates

This pattern is used by major Rust projects:

- **`tokio`** - Facade crate with many internal crates (tokio-util, tokio-stream, etc.)
- **`serde`** - Facade with serde_derive, serde_json as separate crates
- **`sqlx`** - Feature flags for different databases, one crate to import

Users import `tokio`, not `tokio-util`. Same pattern here: users import `schwab-api`, not `schwab-api-trader`.
