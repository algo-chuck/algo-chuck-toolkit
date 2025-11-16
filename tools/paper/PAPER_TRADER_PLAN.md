# Paper Trader Application Implementation Plan

A comprehensive guide for building a "drop-in" replacement mock server for the Schwab Trader API using SQLite (with easy PostgreSQL migration path).

---

## Implementation Progress

**Current Phase: Phase 4 Complete** (November 16, 2025)

- ✅ **Phase 1**: Database schema design and planning decisions
- ✅ **Phase 2**: Repository layer with 4 repositories (November 15, 2025)
- ✅ **Phase 3**: Service layer with 4 services (November 16, 2025)
- ✅ **Phase 4**: Query parameter filtering + Order execution (November 16, 2025)
  - ✅ **Part 1**: Query parameter filtering implemented in repositories and services
  - ✅ **Part 2**: Order execution framework (MarketDataService, OrderExecutor core structure)
- ⏳ **Phase 5**: Handler layer & API integration (Not Started)
- ⏳ **Phase 6**: Application state & dependency injection (Not Started)

**Known Limitations:**

- **Field filtering** for accounts deferred (query parameter accepted but not implemented)
- **Symbol filtering** for transactions deferred (requires JSON extraction or indexed column)
- **Position/Balance updates** after order fills deferred (complex SecuritiesAccount enum handling)
- **Transaction generation** from fills deferred (Transaction type structure needs clarification)
- **Order execution validation** (balance checks) not implemented yet
- Order executor background task structure in place but not integrated into main application

---

## Project Architecture Overview

Your paper trader will be a drop-in replacement mock server that implements all 13 Schwab Trader API endpoints with SQLite persistence.

---

## Phase 1: Database Schema Design

### Design Philosophy

After analyzing the OpenAPI spec (`api-spec-trader.json`), the schema follows these principles:

1. **Store API objects as JSON** - Match what the API returns exactly
2. **Minimal normalization** - Only extract fields needed for querying/filtering
3. **4 core tables** - Accounts, Orders, Transactions, UserPreferences
4. **No over-engineering** - Positions and balances are part of account data (as per spec)

### Core Tables (Based on OpenAPI Spec)

```sql
-- Table 1: Accounts
-- Stores full SecuritiesAccount JSON (MarginAccount or CashAccount)
-- SecuritiesAccount contains positions array inline, plus initialBalances, currentBalances, projectedBalances
CREATE TABLE accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_number TEXT UNIQUE NOT NULL,      -- plaintext account number
    hash_value TEXT UNIQUE NOT NULL,          -- encrypted value (for API URLs)
    account_type TEXT NOT NULL,               -- 'CASH' or 'MARGIN' (from SecuritiesAccount.type)
    account_data TEXT NOT NULL,               -- Full SecuritiesAccount JSON (includes positions, balances)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- UTC ISO-8601
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP   -- UTC ISO-8601
);

-- Table 2: Orders
-- Stores full Order/OrderRequest JSON
-- Order contains orderLegCollection, orderActivityCollection, childOrderStrategies inline
CREATE TABLE orders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id INTEGER NOT NULL,                -- from Order.orderId (int64 in spec), starts at 1001
    account_number TEXT NOT NULL,             -- from Order.accountNumber (int64 in spec, but we use TEXT for FK)
    status TEXT NOT NULL,                     -- from Order.status enum (WORKING, FILLED, CANCELED, etc.)
    entered_time TIMESTAMP,                   -- from Order.enteredTime (ISO-8601 string)
    close_time TIMESTAMP,                     -- from Order.closeTime (ISO-8601 string)
    order_data TEXT NOT NULL,                 -- Full Order/OrderRequest JSON
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- UTC ISO-8601
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- UTC ISO-8601
    FOREIGN KEY (account_number) REFERENCES accounts(account_number)
);

-- Initialize order_id sequence to start at 1001
-- Note: In implementation, use: INSERT INTO orders (order_id, ...) VALUES ((SELECT COALESCE(MAX(order_id), 1000) + 1 FROM orders), ...)
-- Or set sqlite_sequence: INSERT INTO sqlite_sequence (name, seq) VALUES ('orders', 1000) ON CONFLICT(name) DO UPDATE SET seq = 1000;

-- Table 3: Transactions
-- Stores full Transaction JSON
-- Transaction contains transferItems array, user object inline
CREATE TABLE transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    activity_id INTEGER NOT NULL,            -- from Transaction.activityId (int64 in spec), starts at 1001
    account_number TEXT NOT NULL,            -- from Transaction.accountNumber (string in spec)
    type TEXT NOT NULL,                      -- from Transaction.type enum (TransactionType)
    time TIMESTAMP,                          -- from Transaction.time (ISO-8601 string)
    transaction_data TEXT NOT NULL,          -- Full Transaction JSON
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- UTC ISO-8601
    FOREIGN KEY (account_number) REFERENCES accounts(account_number)
);

-- Initialize activity_id sequence to start at 1001
-- Note: Similar to orders, initialize sqlite_sequence for transactions table

-- Table 4: User Preferences
-- Stores full UserPreference JSON
-- UserPreference contains accounts array, streamerInfo array, offers array inline
CREATE TABLE user_preferences (
    id INTEGER PRIMARY KEY,
    preference_data TEXT NOT NULL,           -- Full UserPreference JSON
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP  -- UTC ISO-8601
);

-- Indexes for common queries
CREATE INDEX idx_orders_account_number ON orders(account_number);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_entered_time ON orders(entered_time);
CREATE INDEX idx_transactions_account_number ON transactions(account_number);
CREATE INDEX idx_transactions_type ON transactions(type);
CREATE INDEX idx_transactions_time ON transactions(time);
```

### Phase 1 Planning Questions

**Status: ✅ DECISIONS COMPLETE**

#### 1. Database Library Choice ✅

- **Option A: sqlx** - Async-first, compile-time checked SQL, works with raw SQL
  - Pros: Modern async/await, flexible, no ORM overhead
  - Cons: Need to write SQL by hand
- ~~**Option B: diesel** - Type-safe ORM, can be sync or async~~
  - ~~Pros: Strong typing, migrations built-in, less SQL writing~~
  - ~~Cons: Steeper learning curve, more boilerplate~~

**Decision: [✅] sqlx**

#### 2. Order ID Generation ✅

Real Schwab API uses `int64` for `orderId`. Options:

- ~~**Option A:** Use SQLite AUTOINCREMENT as the order ID~~
- ~~**Option B:** Generate random int64s (e.g., timestamp-based or UUID as int)~~
- **Option C:** Use sequential counter starting from 1001

**Decision: [✅] AUTOINCREMENT starting at 1001**  
_Implementation: Set initial sequence value or start ID counter at 1001_

#### 3. Initial Account Setup ✅

- ~~**Option A:** Seed database with 2-3 sample accounts on first run~~
- **Option B:** Start with empty database, no seeding for now
- ~~**Option C:** Add admin endpoint to create accounts (e.g., POST /admin/accounts)~~

**Decision: [✅] No seeding initially**  
**TODO: Create Phase 0 (Account Management) to address:**

- How new accounts are created (admin endpoints?)
- Initial account balance and type configuration
- Account reset/deletion functionality
- Test data fixture generation

#### 4. Account Number/Hash Relationship ✅

The `/accounts/accountNumbers` endpoint returns `{ accountNumber, hashValue }[]`

- ~~**Option A:** Generate account numbers and hashes on startup/first run~~
- **Option B:** Let users create accounts via separate admin endpoint series
- ~~**Option C:** Use fixed account numbers for testing (e.g., "12345678", "87654321")~~

**Decision: [✅] Admin endpoints for account creation**  
_Part of Phase 0 (Account Management) - to be designed_

#### 5. Timestamp Format ✅

Per OpenAPI spec: `"type": "string", "format": "date-time"` with ISO-8601 format `yyyy-MM-dd'T'HH:mm:ss.SSSZ`

- **Option A:** Use SQLite's `CURRENT_TIMESTAMP` (UTC, ISO-8601 format)
- ~~**Option B:** Store as Unix epoch integers (easier for calculations)~~
- ~~**Option C:** Store as TEXT in ISO-8601 format for human readability~~

**Decision: [✅] CURRENT_TIMESTAMP (UTC)**  
_SQLite's CURRENT_TIMESTAMP returns ISO-8601 format in UTC, matching API spec requirement_

#### 6. Transaction ID Generation ✅

Similar to Order IDs, Transaction.activityId is int64:

- **Option A:** Use SQLite AUTOINCREMENT (starting at 1001, same as orders)
- ~~**Option B:** Generate timestamp-based int64s~~
- ~~**Option C:** Use sequential counter~~

**Decision: [✅] AUTOINCREMENT starting at 1001**  
_Consistent with order ID generation strategy_

#### 7. Database Indexes ✅

**Decision: [✅] Add indexes on frequently queried fields**  
_Already included in schema: orders.account_number, orders.status, orders.entered_time, transactions.account_number, transactions.type, transactions.time_

**TODO: Review and optimize indexes after development/load testing**

---

### Schema Rationale

**Why JSON storage instead of normalized tables?**

1. ✅ **API Fidelity**: The OpenAPI spec defines complex nested structures:
   - `SecuritiesAccount` → `positions[]` (inline, not separate table)
   - `MarginBalance`/`CashBalance` → nested objects (30+ fields each)
   - `Order` → `orderLegCollection[]`, `childOrderStrategies[]` (inline)
2. ✅ **Simplicity**: 4 tables vs 8+ normalized tables

3. ✅ **Flexibility**: API changes don't require schema migrations

4. ✅ **Query Performance**: We index only what we filter/sort by (status, time, account)

5. ✅ **Type Safety**: Rust structs (from generated types) serialize/deserialize directly

**What we extract for indexing:**

- `orders.status` - Filter by order status (WORKING, FILLED, etc.)
- `orders.entered_time` - Date range filtering per spec
- `transactions.type` - Filter by transaction type per spec
- `transactions.time` - Date range filtering per spec

---

## Phase 2: Database Layer (Repository Pattern)

**Status: ✅ COMPLETE (November 15, 2025)**

### Implementation Summary

Successfully implemented all 4 repositories with:

- ✅ Runtime SQL queries (no DATABASE_URL compile dependency)
- ✅ Custom error types per repository (AccountError, OrderError, TransactionError, UserPreferenceError)
- ✅ All method names aligned with OpenAPI operationIds
- ✅ ID generation starting at 1001 for orders and transactions
- ✅ JSON blob storage with indexed query fields
- ✅ Database migrations using sqlx::migrate!
- ✅ Compiles successfully with 0 errors

### Files Created

```
tools/paper/src/
├── db/
│   ├── mod.rs                          # ✅ Database pool initialization with migrations
│   ├── migrations/
│   │   └── 001_initial_schema.sql      # ✅ Initial migration (single source of truth)
│   └── repositories/
│       ├── mod.rs                      # ✅ Re-exports all repositories
│       ├── accounts.rs                 # ✅ AccountRepository (3 API methods + 3 helpers)
│       ├── orders.rs                   # ✅ OrderRepository (6 API methods + 2 helpers)
│       ├── transactions.rs             # ✅ TransactionRepository (2 API methods + 1 helper)
│       └── user_preference.rs          # ✅ UserPreferenceRepository (1 API method + 1 helper)
```

### Design Decisions

**Naming Conventions (Aligned with OpenAPI Spec):**

1. **"Repository" Pattern** - Standard naming for data access layer (not "tables" or "models")
2. **File Names** - Based on OpenAPI `tags`, converted to Rust snake_case:
   - `Accounts` → `accounts.rs`
   - `Orders` → `orders.rs`
   - `Transactions` → `transactions.rs`
   - `User Preference` → `user_preference.rs`
3. **Method Names** - Based on OpenAPI `operationId`, converted to Rust snake_case:
   - `getAccountNumbers` → `get_account_numbers()`
   - `placeOrder` → `place_order()`
   - `cancelOrder` → `cancel_order()`
   - `getTransactionsByPathParam` → `get_transactions_by_path_param()`
   - `getUserPreference` → `get_user_preference()`

### Dependencies Added

**Using sqlx (Decision from Phase 1):**

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "macros", "chrono"] }
chrono = { version = "0.4", features = ["serde"] }  # For timestamp handling
```

### Implementation Notes

1. **Runtime vs Compile-time Queries**: Used `sqlx::query()` instead of `query!()` macro to avoid requiring DATABASE_URL at compile time
2. **Error Handling**: Each repository has its own error enum with From implementations for sqlx::Error and serde_json::Error
   - **Note**: These are internal errors, NOT the OpenAPI `ServiceError` type
   - Phase 5 will convert these to `ServiceError` for API responses
3. **Type Path**: Types are imported from `schwab_api::types::trader` (not `schwab_api::trader`)
4. **ID Generation**: Using `SELECT COALESCE(MAX(id), 1000) + 1` pattern to start IDs at 1001
5. **Tests Included**: Each repository has basic test setup (not yet implemented)
6. **Migration Safety**: `sqlx::migrate!()` is idempotent - it tracks applied migrations in `_sqlx_migrations` table and only runs new ones
7. **Schema Management**: Single source of truth in `src/db/migrations/001_initial_schema.sql` (executed on startup)
8. **Early Development Schema Changes**:
   - Until deployed to production, directly edit `001_initial_schema.sql` for schema changes
   - Delete the database file (`rm -f paper-trader.db paper-trader.db-shm paper-trader.db-wal`) after schema edits
   - No need for multiple migration files during early development
   - Once deployed or shared with others, start creating new migration files (`002_*.sql`, etc.)

### Repository Pattern Examples

**Using sqlx with async/await:**

**Note:** Method names follow OpenAPI `operationId` converted to snake_case

```rust
// db/repositories/accounts.rs
// Implements operations from OpenAPI tag: "Accounts"
use sqlx::SqlitePool;
use serde_json;
use schwab_api_types::trader::SecuritiesAccount;  // From your existing types

pub struct AccountRepository {
    pool: SqlitePool,
}

impl AccountRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // operationId: getAccountNumbers
    pub async fn get_account_numbers(&self) -> Result<Vec<(String, String)>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT account_number, hash_value FROM accounts ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| (r.account_number, r.hash_value)).collect())
    }

    // operationId: getAccounts (list all accounts)
    pub async fn get_accounts(&self) -> Result<Vec<SecuritiesAccount>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT account_data FROM accounts ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|r| serde_json::from_str(&r.account_data))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))
    }

    // operationId: getAccount (get specific account by hash)
    pub async fn get_account(&self, hash: &str) -> Result<Option<SecuritiesAccount>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT account_data FROM accounts WHERE hash_value = ?
            "#,
            hash
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(serde_json::from_str(&r.account_data)?)),
            None => Ok(None),
        }
    }

    // Additional helper methods (not directly from operationIds)

    pub async fn create(&self, account_number: &str, hash_value: &str, account_type: &str, account_data: &SecuritiesAccount) -> Result<i64, sqlx::Error> {
        let account_data_json = serde_json::to_string(account_data)?;

        let result = sqlx::query!(
            r#"
            INSERT INTO accounts (account_number, hash_value, account_type, account_data)
            VALUES (?, ?, ?, ?)
            "#,
            account_number,
            hash_value,
            account_type,
            account_data_json
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn find_by_account_number(&self, account_number: &str) -> Result<Option<SecuritiesAccount>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT account_data FROM accounts WHERE account_number = ?
            "#,
            account_number
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(serde_json::from_str(&r.account_data)?)),
            None => Ok(None),
        }
    }

    pub async fn update(&self, account_number: &str, account_data: &SecuritiesAccount) -> Result<(), sqlx::Error> {
        let account_data_json = serde_json::to_string(account_data)?;

        sqlx::query!(
            r#"
            UPDATE accounts
            SET account_data = ?, updated_at = CURRENT_TIMESTAMP
            WHERE account_number = ?
            "#,
            account_data_json,
            account_number
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

// db/repositories/orders.rs
// Implements operations from OpenAPI tag: "Orders"
use sqlx::SqlitePool;
use schwab_api_types::trader::{Order, OrderRequest};

pub struct OrderRepository {
    pool: SqlitePool,
}

impl OrderRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // operationId: placeOrder
    pub async fn place_order(&self, account_number: &str, order_data: &OrderRequest) -> Result<i64, sqlx::Error> {
        let order_data_json = serde_json::to_string(order_data)?;
        let status = "WORKING";  // Initial status

        // Get next order_id (starting from 1001)
        let order_id: i64 = sqlx::query_scalar!(
            r#"SELECT COALESCE(MAX(order_id), 1000) + 1 as "id!" FROM orders"#
        )
        .fetch_one(&self.pool)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO orders (order_id, account_number, status, order_data)
            VALUES (?, ?, ?, ?)
            "#,
            order_id,
            account_number,
            status,
            order_data_json
        )
        .execute(&self.pool)
        .await?;

        Ok(order_id)
    }

    // operationId: getOrder
    pub async fn get_order(&self, order_id: i64) -> Result<Option<Order>, sqlx::Error> {
        let row = sqlx::query!(
            r#"SELECT order_data FROM orders WHERE order_id = ?"#,
            order_id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(serde_json::from_str(&r.order_data)?)),
            None => Ok(None),
        }
    }

    // operationId: getOrdersByPathParam
    pub async fn get_orders_by_path_param(
        &self,
        account_number: &str,
        from_entered_time: Option<String>,
        to_entered_time: Option<String>,
        status_filter: Option<String>,
    ) -> Result<Vec<Order>, sqlx::Error> {
        // TODO: Implement date and status filtering
        let rows = sqlx::query!(
            r#"
            SELECT order_data FROM orders
            WHERE account_number = ?
            ORDER BY entered_time DESC
            "#,
            account_number
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|r| serde_json::from_str(&r.order_data))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))
    }

    // operationId: getOrdersByQueryParam
    pub async fn get_orders_by_query_param(
        &self,
        from_entered_time: Option<String>,
        to_entered_time: Option<String>,
        status_filter: Option<String>,
    ) -> Result<Vec<Order>, sqlx::Error> {
        // TODO: Implement date and status filtering for all accounts
        let rows = sqlx::query!(
            r#"
            SELECT order_data FROM orders
            ORDER BY entered_time DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|r| serde_json::from_str(&r.order_data))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))
    }

    // operationId: cancelOrder
    pub async fn cancel_order(&self, order_id: i64) -> Result<(), sqlx::Error> {
        let status = "CANCELED";

        sqlx::query!(
            r#"
            UPDATE orders
            SET status = ?, close_time = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
            WHERE order_id = ?
            "#,
            status,
            order_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // operationId: replaceOrder
    pub async fn replace_order(&self, order_id: i64, new_order_data: &OrderRequest) -> Result<i64, sqlx::Error> {
        // Cancel old order
        self.cancel_order(order_id).await?;

        // Get account_number from old order
        let old_order = self.get_order(order_id).await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?;

        let account_number = old_order.account_number
            .ok_or_else(|| sqlx::Error::RowNotFound)?
            .to_string();

        // Place new order
        self.place_order(&account_number, new_order_data).await
    }

    // operationId: previewOrder
    // Note: Preview doesn't persist to database, handled in service layer

    // Additional helper methods

    pub async fn update_status(&self, order_id: i64, status: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE orders
            SET status = ?, updated_at = CURRENT_TIMESTAMP
            WHERE order_id = ?
            "#,
            status,
            order_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update(&self, order_id: i64, order_data: &Order) -> Result<(), sqlx::Error> {
        let order_data_json = serde_json::to_string(order_data)?;
        let status = &order_data.status.as_ref().map(|s| s.to_string()).unwrap_or_else(|| "UNKNOWN".to_string());

        sqlx::query!(
            r#"
            UPDATE orders
            SET order_data = ?, status = ?, updated_at = CURRENT_TIMESTAMP
            WHERE order_id = ?
            "#,
            order_data_json,
            status,
            order_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

// db/repositories/transactions.rs
// Implements operations from OpenAPI tag: "Transactions"
use sqlx::SqlitePool;
use schwab_api_types::trader::Transaction;

pub struct TransactionRepository {
    pool: SqlitePool,
}

impl TransactionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // operationId: getTransactionsByPathParam
    pub async fn get_transactions_by_path_param(
        &self,
        account_number: &str,
        start_date: &str,
        end_date: &str,
        transaction_type: Option<&str>,
    ) -> Result<Vec<Transaction>, sqlx::Error> {
        // TODO: Implement date and type filtering
        let rows = sqlx::query!(
            r#"
            SELECT transaction_data FROM transactions
            WHERE account_number = ?
            ORDER BY time DESC
            "#,
            account_number
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|r| serde_json::from_str(&r.transaction_data))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))
    }

    // operationId: getTransactionsById
    pub async fn get_transactions_by_id(&self, activity_id: i64) -> Result<Option<Transaction>, sqlx::Error> {
        let row = sqlx::query!(
            r#"SELECT transaction_data FROM transactions WHERE activity_id = ?"#,
            activity_id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(serde_json::from_str(&r.transaction_data)?)),
            None => Ok(None),
        }
    }

    // Additional helper method

    pub async fn create(&self, account_number: &str, transaction_type: &str, transaction_data: &Transaction) -> Result<i64, sqlx::Error> {
        let transaction_data_json = serde_json::to_string(transaction_data)?;

        // Get next activity_id (starting from 1001)
        let activity_id: i64 = sqlx::query_scalar!(
            r#"SELECT COALESCE(MAX(activity_id), 1000) + 1 as "id!" FROM transactions"#
        )
        .fetch_one(&self.pool)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO transactions (activity_id, account_number, type, transaction_data)
            VALUES (?, ?, ?, ?)
            "#,
            activity_id,
            account_number,
            transaction_type,
            transaction_data_json
        )
        .execute(&self.pool)
        .await?;

        Ok(activity_id)
    }
}

// db/repositories/user_preference.rs
// Implements operations from OpenAPI tag: "User Preference"
use sqlx::SqlitePool;
use schwab_api_types::trader::UserPreference;

pub struct UserPreferenceRepository {
    pool: SqlitePool,
}

impl UserPreferenceRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // operationId: getUserPreference
    pub async fn get_user_preference(&self) -> Result<Option<UserPreference>, sqlx::Error> {
        let row = sqlx::query!(
            r#"SELECT preference_data FROM user_preferences WHERE id = 1"#
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(serde_json::from_str(&r.preference_data)?)),
            None => Ok(None),
        }
    }

    // Additional helper method

    pub async fn upsert(&self, preference_data: &UserPreference) -> Result<(), sqlx::Error> {
        let preference_data_json = serde_json::to_string(preference_data)?;

        sqlx::query!(
            r#"
            INSERT INTO user_preferences (id, preference_data)
            VALUES (1, ?)
            ON CONFLICT(id) DO UPDATE SET
                preference_data = excluded.preference_data,
                updated_at = CURRENT_TIMESTAMP
            "#,
            preference_data_json
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
```

### OpenAPI Spec Mapping Reference

**Complete mapping of tags and operationIds:**

| Tag                 | OpenAPI operationId          | Repository Method                      |
| ------------------- | ---------------------------- | -------------------------------------- |
| **Accounts**        | `getAccountNumbers`          | `get_account_numbers()`                |
|                     | `getAccounts`                | `get_accounts()`                       |
|                     | `getAccount`                 | `get_account()`                        |
| **Orders**          | `getOrdersByPathParam`       | `get_orders_by_path_param()`           |
|                     | `placeOrder`                 | `place_order()`                        |
|                     | `getOrder`                   | `get_order()`                          |
|                     | `cancelOrder`                | `cancel_order()`                       |
|                     | `replaceOrder`               | `replace_order()`                      |
|                     | `getOrdersByQueryParam`      | `get_orders_by_query_param()`          |
|                     | `previewOrder`               | _(service layer only - not persisted)_ |
| **Transactions**    | `getTransactionsByPathParam` | `get_transactions_by_path_param()`     |
|                     | `getTransactionsById`        | `get_transactions_by_id()`             |
| **User Preference** | `getUserPreference`          | `get_user_preference()`                |

---

## Phase 3: Service Layer (Thin CRUD Wrapper)

**Status: ✅ COMPLETE (November 16, 2025)**

### Implementation Summary

Phase 3 successfully implemented a thin service layer that wraps repositories with basic validation. All services use existing parameter types from `schwab_api::types::trader` and compile successfully.

**What Was Completed:**

- ✅ Service structs wrapping repositories (4 services)
- ✅ Custom service error types with `thiserror`
- ✅ Basic validation logic (non-empty checks)
- ✅ Proper error conversion (Repository errors → Service errors)
- ✅ Lifetime parameter handling for param types
- ✅ Added `thiserror` dependency to Cargo.toml
- ✅ Implemented `preview_order()` in OrderService (returns basic PreviewOrder structure)
- ❌ No order execution (deferred to Phase 4)
- ❌ No market data service (deferred to Phase 4)
- ❌ No background tasks (deferred to Phase 4)
- ❌ Unit tests (deferred - can be added when needed)
- ⚠️ Query parameter filtering not implemented (deferred to Phase 4)
- ⚠️ Preview order calculations (commissions, validation results) deferred to Phase 4

### Files Created

```
tools/paper/src/services/
├── mod.rs                      # Module exports for all 4 services
├── accounts.rs                 # AccountService (3 methods)
├── orders.rs                   # OrderService (7 methods including preview_order)
├── transactions.rs             # TransactionService (2 methods)
└── user_preference.rs          # UserPreferenceService (1 method)
```

Also updated: `tools/paper/Cargo.toml` (added `thiserror = "2"`)
Also updated: `tools/paper/src/main.rs` (added `mod services;`)

### Known Limitations

**Query Parameters Not Implemented:**

Services accept parameter structs but don't use the query parameters for filtering:

- `GetAccountsParams.fields` - ignored (returns all fields)
- `GetAccountParams.fields` - ignored (returns all fields)
- `GetOrdersByPathParams.maxResults` - ignored (returns all orders)
- `GetOrdersByPathParams.status` - ignored (no status filtering)
- Date range parameters - ignored (no date filtering)
- `GetTransactionsByPathParams.types` - ignored (no type filtering)
- `GetTransactionsByPathParams.symbol` - ignored (no symbol filtering)

**Reason:** Repository layer doesn't support filtering yet. This was intentional for Phase 3's "thin wrapper" scope.

**Resolution:** Phase 4 will add filtering logic to both repositories and services.

### Design Decisions (Based on Phase 1-2)

**Decision 1: Positions Storage**

- ✅ Positions remain in `account_data` JSON (as per OpenAPI spec)
- ❌ No separate `PositionRepository`
- **Rationale:** Matches API structure, simpler implementation

**Decision 2: Database Configuration**

- ✅ Use in-memory SQLite (`:memory:`) for unit/integration tests
- ✅ Use file-based SQLite for running server
- **Configuration:** Via `init_db()` parameter in `db/mod.rs`

**Decision 3: Service Scope**

- ✅ Thin wrapper around repositories
- ✅ Input validation only
- ❌ No order execution logic yet
- **Rationale:** Incremental development, test each layer

**Decision 4: Market Data**

- ❌ Not included in Phase 3
- **Rationale:** Not needed until order execution in Phase 4

### File Structure

```
tools/paper/src/
├── services/
│   ├── mod.rs                  # Re-exports all services
│   ├── accounts.rs             # AccountService
│   ├── orders.rs               # OrderService
│   ├── transactions.rs         # TransactionService
│   └── user_preference.rs      # UserPreferenceService
```

### Service Layer Architecture

**Services wrap repositories and add:**

1. Input validation
2. Business rule enforcement
3. Error mapping (repository errors → service errors)
4. Coordination between multiple repositories (if needed)

### Implementation Examples

```rust
// services/accounts.rs
use crate::db::repositories::{AccountRepository, AccountError};
use schwab_api::types::trader::SecuritiesAccount;

#[derive(Debug)]
pub enum AccountServiceError {
    Repository(AccountError),
    NotFound(String),
    InvalidInput(String),
}

impl From<AccountError> for AccountServiceError {
    fn from(e: AccountError) -> Self {
        match e {
            AccountError::NotFound(id) => AccountServiceError::NotFound(id),
            e => AccountServiceError::Repository(e),
        }
    }
}

pub struct AccountService {
    account_repo: AccountRepository,
}

impl AccountService {
    pub fn new(account_repo: AccountRepository) -> Self {
        Self { account_repo }
    }

    // operationId: getAccountNumbers
    pub async fn get_account_numbers(&self) -> Result<Vec<(String, String)>, AccountServiceError> {
        self.account_repo
            .get_account_numbers()
            .await
            .map_err(AccountServiceError::from)
    }

    // operationId: getAccounts
    pub async fn get_accounts(&self) -> Result<Vec<SecuritiesAccount>, AccountServiceError> {
        self.account_repo
            .get_accounts()
            .await
            .map_err(AccountServiceError::from)
    }

    // operationId: getAccount
    pub async fn get_account(&self, hash: &str) -> Result<SecuritiesAccount, AccountServiceError> {
        if hash.is_empty() {
            return Err(AccountServiceError::InvalidInput(
                "Account hash cannot be empty".to_string(),
            ));
        }

        self.account_repo
            .get_account(hash)
            .await
            .map_err(AccountServiceError::from)
    }
}

// services/orders.rs
use crate::db::repositories::{OrderRepository, OrderError};
use schwab_api::types::trader::{Order, OrderRequest};

#[derive(Debug)]
pub enum OrderServiceError {
    Repository(OrderError),
    NotFound(i64),
    InvalidInput(String),
}

impl From<OrderError> for OrderServiceError {
    fn from(e: OrderError) -> Self {
        match e {
            OrderError::NotFound(id) => OrderServiceError::NotFound(id),
            e => OrderServiceError::Repository(e),
        }
    }
}

pub struct OrderService {
    order_repo: OrderRepository,
}

impl OrderService {
    pub fn new(order_repo: OrderRepository) -> Self {
        Self { order_repo }
    }

    // operationId: placeOrder
    pub async fn place_order(
        &self,
        account_number: &str,
        order_data: &OrderRequest,
    ) -> Result<i64, OrderServiceError> {
        // Basic validation
        if account_number.is_empty() {
            return Err(OrderServiceError::InvalidInput(
                "Account number cannot be empty".to_string(),
            ));
        }

        // Note: Order execution happens in Phase 4
        // For now, just persist the order with WORKING status
        self.order_repo
            .place_order(account_number, order_data)
            .await
            .map_err(OrderServiceError::from)
    }

    // operationId: getOrder
    pub async fn get_order(&self, order_id: i64) -> Result<Order, OrderServiceError> {
        self.order_repo
            .get_order(order_id)
            .await
            .map_err(OrderServiceError::from)
    }

    // operationId: cancelOrder
    pub async fn cancel_order(&self, order_id: i64) -> Result<(), OrderServiceError> {
        // Basic validation: check if order exists and is cancelable
        let order = self.get_order(order_id).await?;

        // Check if order can be canceled (not already filled/canceled)
        if let Some(status) = &order.status {
            match status.as_str() {
                "FILLED" | "CANCELED" | "EXPIRED" | "REJECTED" => {
                    return Err(OrderServiceError::InvalidInput(format!(
                        "Cannot cancel order with status: {}",
                        status
                    )));
                }
                _ => {}
            }
        }

        self.order_repo
            .cancel_order(order_id)
            .await
            .map_err(OrderServiceError::from)
    }

    // operationId: replaceOrder
    pub async fn replace_order(
        &self,
        order_id: i64,
        new_order_data: &OrderRequest,
    ) -> Result<i64, OrderServiceError> {
        // Validate old order is replaceable (same as cancel logic)
        let old_order = self.get_order(order_id).await?;

        if let Some(status) = &old_order.status {
            match status.as_str() {
                "FILLED" | "CANCELED" | "EXPIRED" | "REJECTED" => {
                    return Err(OrderServiceError::InvalidInput(format!(
                        "Cannot replace order with status: {}",
                        status
                    )));
                }
                _ => {}
            }
        }

        self.order_repo
            .replace_order(order_id, new_order_data)
            .await
            .map_err(OrderServiceError::from)
    }

    // operationId: getOrdersByPathParam
    pub async fn get_orders_by_path_param(
        &self,
        account_number: &str,
        from_entered_time: Option<String>,
        to_entered_time: Option<String>,
        status_filter: Option<String>,
    ) -> Result<Vec<Order>, OrderServiceError> {
        if account_number.is_empty() {
            return Err(OrderServiceError::InvalidInput(
                "Account number cannot be empty".to_string(),
            ));
        }

        self.order_repo
            .get_orders_by_path_param(account_number, from_entered_time, to_entered_time, status_filter)
            .await
            .map_err(OrderServiceError::from)
    }

    // operationId: getOrdersByQueryParam
    pub async fn get_orders_by_query_param(
        &self,
        from_entered_time: Option<String>,
        to_entered_time: Option<String>,
        status_filter: Option<String>,
    ) -> Result<Vec<Order>, OrderServiceError> {
        self.order_repo
            .get_orders_by_query_param(from_entered_time, to_entered_time, status_filter)
            .await
            .map_err(OrderServiceError::from)
    }
}

// services/transactions.rs
use crate::db::repositories::{TransactionRepository, TransactionError};
use schwab_api::types::trader::Transaction;

#[derive(Debug)]
pub enum TransactionServiceError {
    Repository(TransactionError),
    NotFound(i64),
    InvalidInput(String),
}

impl From<TransactionError> for TransactionServiceError {
    fn from(e: TransactionError) -> Self {
        match e {
            TransactionError::NotFound(id) => TransactionServiceError::NotFound(id),
            e => TransactionServiceError::Repository(e),
        }
    }
}

pub struct TransactionService {
    transaction_repo: TransactionRepository,
}

impl TransactionService {
    pub fn new(transaction_repo: TransactionRepository) -> Self {
        Self { transaction_repo }
    }

    // operationId: getTransactionsByPathParam
    pub async fn get_transactions_by_path_param(
        &self,
        account_number: &str,
        start_date: &str,
        end_date: &str,
        transaction_type: Option<&str>,
    ) -> Result<Vec<Transaction>, TransactionServiceError> {
        if account_number.is_empty() {
            return Err(TransactionServiceError::InvalidInput(
                "Account number cannot be empty".to_string(),
            ));
        }

        self.transaction_repo
            .get_transactions_by_path_param(account_number, start_date, end_date, transaction_type)
            .await
            .map_err(TransactionServiceError::from)
    }

    // operationId: getTransactionsById
    pub async fn get_transactions_by_id(
        &self,
        activity_id: i64,
    ) -> Result<Transaction, TransactionServiceError> {
        self.transaction_repo
            .get_transactions_by_id(activity_id)
            .await
            .map_err(TransactionServiceError::from)
    }
}

// services/user_preference.rs
use crate::db::repositories::{UserPreferenceRepository, UserPreferenceError};
use schwab_api::types::trader::UserPreference;

#[derive(Debug)]
pub enum UserPreferenceServiceError {
    Repository(UserPreferenceError),
    NotFound,
}

impl From<UserPreferenceError> for UserPreferenceServiceError {
    fn from(e: UserPreferenceError) -> Self {
        match e {
            UserPreferenceError::NotFound => UserPreferenceServiceError::NotFound,
            e => UserPreferenceServiceError::Repository(e),
        }
    }
}

pub struct UserPreferenceService {
    preference_repo: UserPreferenceRepository,
}

impl UserPreferenceService {
    pub fn new(preference_repo: UserPreferenceRepository) -> Self {
        Self { preference_repo }
    }

    // operationId: getUserPreference
    pub async fn get_user_preference(&self) -> Result<UserPreference, UserPreferenceServiceError> {
        self.preference_repo
            .get_user_preference()
            .await
            .map_err(UserPreferenceServiceError::from)
    }
}
```

### Testing Strategy

**Unit Tests with In-Memory Database:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn setup_test_service() -> AccountService {
        // Use in-memory database for tests
        let pool = SqlitePool::connect(":memory:").await.unwrap();

        // Run migrations
        sqlx::migrate!("./src/db/migrations")
            .run(&pool)
            .await
            .unwrap();

        let account_repo = AccountRepository::new(pool);
        AccountService::new(account_repo)
    }

    #[tokio::test]
    async fn test_get_account_numbers_empty() {
        let service = setup_test_service().await;
        let result = service.get_account_numbers().await.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[tokio::test]
    async fn test_get_account_invalid_hash() {
        let service = setup_test_service().await;
        let result = service.get_account("").await;
        assert!(matches!(result, Err(AccountServiceError::InvalidInput(_))));
    }
}
```

### Phase 3 Implementation Tasks

- [x] Create `src/services/mod.rs` with module exports
- [x] Implement `AccountService` with 3 methods + error type
- [x] Implement `OrderService` with 7 methods + error type + validation (includes `preview_order`)
- [x] Implement `TransactionService` with 2 methods + error type
- [x] Implement `UserPreferenceService` with 1 method + error type
- [ ] Add unit tests for each service using in-memory database (deferred)
- [x] Update `main.rs` to include services module

### Success Criteria

- [x] All 4 services compile successfully
- [x] Services properly wrap repository methods
- [x] Error types convert correctly
- [x] Basic validation logic works (non-empty checks, status checks)
- [ ] Unit tests pass with in-memory database (deferred)
- [x] No order execution logic included (deferred to Phase 4)

### Important Note on Error Handling

**Phase 3 uses internal error types** (`AccountServiceError`, `OrderServiceError`, etc.) for business logic only. These are NOT exposed to the API.

**Phase 5 will convert to OpenAPI spec errors:**

- All API error responses use `ServiceError` type (from `schwab_api::types::trader::ServiceError`)
- HTTP status codes: 400 (BadRequest), 401 (NotAuthorized), 403 (Forbidden), 404 (NotFound), 500 (InternalServerError), 503 (ServiceUnavailable)
- Handler layer maps internal errors → `ServiceError` + appropriate HTTP status code
- Example: `AccountServiceError::NotFound("123")` → `ServiceError { message: "Account not found", errors: [...] }` + HTTP 404

**Error Conversion Architecture:**

```
Repository Layer: AccountError, OrderError (internal)
    ↓
Service Layer: AccountServiceError, OrderServiceError (business logic)
    ↓
Handler Layer: Converts to ServiceError + HTTP status (Phase 5)
    ↓
API Response: ServiceError JSON matching OpenAPI spec
```

**TODO for Phase 5:**

- [ ] Implement error conversion from service errors to `ServiceError`
- [ ] Map error types to correct HTTP status codes per OpenAPI spec
- [ ] Ensure all error responses match `ServiceError` schema format
- [ ] Add error details with `ServiceErrorItem` for validation errors

---

## Phase 4: Query Parameter Filtering + Order Execution

**Status: ✅ COMPLETE (November 16, 2025)**

Phase 4 implemented query parameter filtering and the order execution framework.

### Part 1: Query Parameter Filtering - Implementation Summary

**Design Decision: Pass Params Structs Directly**

Repositories accept the full params struct by reference instead of unpacking into individual arguments. This approach:

- Avoids parameter explosion as query params grow
- Types already validated by param structs
- Easier to extend with new query parameters
- Matches pattern already used in service layer
- Single source of truth for available parameters

**Repository Changes Implemented:**

1. **AccountRepository**:

   - `get_accounts(&self, params: &GetAccountsParams<'_>)` - accepts params but filtering deferred
   - `get_account(&self, params: &GetAccountParams<'_>)` - uses `params.account_hash`
   - Field filtering (for `params.fields`) deferred - TODO added

2. **OrderRepository**:

   - `get_orders_by_path_param(&self, params: &GetOrdersByPathParams<'_>)` - implemented
     - Date range filtering: `>= from_entered_time`, `<= to_entered_time` (required fields)
     - Status filtering: `WHERE status = ?` (optional)
     - Pagination: `LIMIT max_results` (optional)
   - `get_orders_by_query_param(&self, params: &GetOrdersByQueryParams<'_>)` - implemented
     - Same filtering as path param version but no account filter
     - Uses `WHERE 1=1` pattern for clean SQL building

3. **TransactionRepository**:
   - `get_transactions_by_path_param(&self, params: &GetTransactionsByPathParams<'_>)` - implemented
     - Date range filtering: `>= start_date`, `<= end_date` (required fields)
     - Types filtering: Splits comma-separated list, builds `IN (?, ?, ...)` clause
     - Symbol filtering deferred - TODO added (requires JSON extraction)

**Service Layer Changes:**

All services updated to pass params structs directly to repositories instead of extracting individual fields:

- `AccountService::get_accounts(&params)` and `get_account(&params)`
- `OrderService::get_orders_by_path(&params)` and `get_orders_by_query(&params)`
- `TransactionService::get_transactions(&params)`

**Dynamic SQL Building Pattern:**

```rust
let mut query = String::from("SELECT ... WHERE ...");
let mut bind_values: Vec<String> = vec![];

// Required fields
query.push_str(" AND field >= ?");
bind_values.push(params.field.to_string());

// Optional fields
if let Some(value) = params.optional_field {
    query.push_str(" AND field = ?");
    bind_values.push(value.to_string());
}

let mut sqlx_query = sqlx::query_scalar::<_, String>(&query);
for value in bind_values {
    sqlx_query = sqlx_query.bind(value);
}
```

**Known Limitations:**

- **Field filtering** for accounts not implemented (query parameter accepted but ignored)
- **Symbol filtering** for transactions not implemented (requires JSON extraction or indexed column)
- Date parameters are required fields (&str), not Optional
- Types parameter is required field (&str), not Optional

**Files Modified:**

- `tools/paper/src/db/repositories/accounts.rs` - Updated 2 method signatures
- `tools/paper/src/db/repositories/orders.rs` - Updated 2 methods with dynamic SQL
- `tools/paper/src/db/repositories/transactions.rs` - Updated 1 method with dynamic SQL
- `tools/paper/src/services/accounts.rs` - Pass params structs (2 methods)
- `tools/paper/src/services/orders.rs` - Pass params structs (2 methods)
- `tools/paper/src/services/transactions.rs` - Pass params structs (1 method)

### Part 2: Order Execution Framework - Implementation Summary

**MarketDataService** (`tools/paper/src/services/market_data.rs`):

Mock market data service with simulated real-time prices:

- Provides base prices for 12 common symbols (AAPL, GOOGL, MSFT, AMZN, TSLA, NVDA, META, JPM, V, WMT, SPY, QQQ)
- `get_current_price(symbol)` returns simulated price with ±1% random variation
- `add_symbol()` allows adding custom symbols for testing
- `has_symbol()` checks symbol availability

**OrderExecutor** (`tools/paper/src/services/order_executor.rs`):

Order execution engine with fill logic for different order types:

1. **Structure**:

   - Holds references to OrderRepository, AccountRepository, TransactionRepository, MarketDataService
   - `new()` constructor
   - `run_execution_loop()` for background execution (tokio task)

2. **Order Fill Methods**:

   - `check_and_fill_market_order()` - Fills MARKET orders immediately at current price
   - `check_and_fill_limit_order()` - Fills LIMIT orders when market price meets limit:
     - BUY: market_price <= limit_price
     - SELL: market_price >= limit_price
   - `check_and_fill_stop_order()` - Activates STOP orders when price crosses stop level:
     - SELL STOP: activates when market_price <= stop_price
     - BUY STOP: activates when market_price >= stop_price

3. **Helper Methods**:
   - `fill_order()` - Updates order status to FILLED, sets filled_quantity, close_time
   - `extract_symbol()` - Extracts symbol from order's first leg (handles AccountsInstrument enum)
   - `extract_instruction()` - Extracts BUY/SELL instruction from order's first leg

**Known Limitations:**

- **Position updates** after fills not implemented (complex SecuritiesAccount enum with Margin/Cash variants)
- **Balance updates** after fills not implemented (MarginBalance vs CashBalance have different field structures)
- **Transaction generation** from fills not implemented (Transaction type structure needs clarification)
- **Order validation** (balance/position checks) not implemented in OrderService.place_order()
- **Background task integration** not implemented (OrderExecutor not started in main application)

**Files Created:**

- `tools/paper/src/services/market_data.rs` - Mock price feeds with 12 symbols
- `tools/paper/src/services/order_executor.rs` - Order execution framework (core structure)
- `tools/paper/Cargo.toml` - Added `rand = "0.8"` dependency for price simulation

**Files Modified:**

- `tools/paper/src/services/mod.rs` - Exported MarketDataService and OrderExecutor

### Success Criteria

- [x] Query parameter filtering implemented in all repositories
- [x] Dynamic SQL building works for date ranges, status, types, max_results
- [x] Services pass params structs to repositories
- [x] Build succeeds with no errors
- [x] MarketDataService provides simulated prices
- [x] OrderExecutor has fill logic for MARKET, LIMIT, STOP orders
- [ ] Position/balance updates implemented (deferred - complex type handling)
- [ ] Transaction generation implemented (deferred - type structure unclear)
- [ ] Order validation implemented (deferred - requires balance/position checks)
- [ ] Background execution task integrated (deferred - requires app state setup in Phase 6)

### Next Steps (Phase 5+)

When position/balance updates are needed:

1. Study SecuritiesAccount enum structure (Margin vs Cash variants)
2. Understand MarginBalance vs CashBalance field differences
3. Map balance fields correctly (e.g., cash_balance doesn't exist, use cash_available_for_trading or total_cash)
4. Implement Position instrument details properly (AccountsInstrument enum handling)
5. Add Transaction type field mapping for TRADE transactions

When integrating order execution:

1. Add OrderExecutor to application state (Phase 6)
2. Start background execution loop in main()
3. Implement query for fetching WORKING orders
4. Add order validation in OrderService.place_order()
5. Test full order lifecycle: place → execute → fill → update account

---

## Phase 5: Handler Layer & API Integration (Future)

**Status: ⏸️ NOT STARTED (Waiting for Phase 4 completion)**

Phase 5 connects services to HTTP handlers and implements proper error response formatting per OpenAPI spec.

### Key Responsibilities

1. **HTTP Request/Response Handling** - Extract parameters, call services, format responses
2. **Error Conversion** - Convert internal errors to `ServiceError` per OpenAPI spec
3. **HTTP Status Codes** - Map errors to correct status codes (400, 401, 403, 404, 500, 503)
4. **Response Formatting** - Ensure all responses match OpenAPI schema

### Error Response Implementation

**CRITICAL:** All error responses MUST use `ServiceError` from `schwab_api::types::trader::ServiceError`

```rust
// Error conversion helper
use schwab_api::types::trader::{ServiceError, ServiceErrorItem};

fn map_service_error(err: AccountServiceError) -> (StatusCode, Json<ServiceError>) {
    match err {
        AccountServiceError::NotFound(id) => (
            StatusCode::NOT_FOUND,
            Json(ServiceError {
                message: Some(format!("Account not found: {}", id)),
                errors: Some(vec![ServiceErrorItem {
                    id: Some(id),
                    status: Some(404),
                    title: Some("Not Found".to_string()),
                    detail: Some("The requested account does not exist".to_string()),
                }]),
            })
        ),
        AccountServiceError::InvalidInput(msg) => (
            StatusCode::BAD_REQUEST,
            Json(ServiceError {
                message: Some("Invalid request".to_string()),
                errors: Some(vec![ServiceErrorItem {
                    id: None,
                    status: Some(400),
                    title: Some("Bad Request".to_string()),
                    detail: Some(msg),
                }]),
            })
        ),
        AccountServiceError::Repository(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ServiceError {
                message: Some("Internal server error".to_string()),
                errors: Some(vec![ServiceErrorItem {
                    id: None,
                    status: Some(500),
                    title: Some("Internal Server Error".to_string()),
                    detail: Some("An unexpected error occurred".to_string()),
                }]),
            })
        ),
    }
}
```

### Handler Implementation Examples

Update existing handlers to use services and proper error responses:

```rust
// handlers/accounts.rs
use crate::services::{AccountService, AccountServiceError};
use schwab_api::types::trader::{SecuritiesAccount, ServiceError};
use axum::{extract::{Path, State}, http::StatusCode, response::Json};

pub async fn get_account_numbers(
    State(account_service): State<Arc<AccountService>>
) -> Result<Json<Vec<(String, String)>>, (StatusCode, Json<ServiceError>)> {
    account_service
        .get_account_numbers()
        .await
        .map(Json)
        .map_err(map_service_error)
}

pub async fn get_account(
    Path(account_hash): Path<String>,
    State(account_service): State<Arc<AccountService>>
) -> Result<Json<SecuritiesAccount>, (StatusCode, Json<ServiceError>)> {
    account_service
        .get_account(&account_hash)
        .await
        .map(Json)
        .map_err(map_service_error)
}

// handlers/orders.rs
pub async fn place_order(
    Path(account_number): Path<String>,
    State(order_service): State<Arc<OrderService>>,
    Json(order): Json<OrderRequest>
) -> Result<Json<i64>, (StatusCode, Json<ServiceError>)> {
    order_service
        .place_order(&account_number, &order)
        .await
        .map(Json)
        .map_err(map_order_service_error)
}

pub async fn cancel_order(
    Path((account_number, order_id)): Path<(String, i64)>,
    State(order_service): State<Arc<OrderService>>
) -> Result<StatusCode, (StatusCode, Json<ServiceError>)> {
    order_service
        .cancel_order(order_id)
        .await
        .map(|_| StatusCode::OK)
        .map_err(map_order_service_error)
}
```

### Phase 5 Implementation Tasks

- [ ] Create error conversion functions for all service error types
- [ ] Map each error variant to correct HTTP status code per OpenAPI spec:
  - `NotFound` → 404
  - `InvalidInput` → 400
  - `Repository/Database` → 500
- [ ] Update all handlers to return `Result<T, (StatusCode, Json<ServiceError>)>`
- [ ] Ensure `ServiceError` format matches OpenAPI spec exactly
- [ ] Add integration tests verifying error response format
- [ ] Test all error paths return correct status codes and `ServiceError` JSON

### OpenAPI Error Response Mapping

Per the OpenAPI spec, all endpoints can return these error responses:

| HTTP Status | OpenAPI Response      | Use Case                                |
| ----------- | --------------------- | --------------------------------------- |
| 400         | `BadRequest`          | Invalid input, validation errors        |
| 401         | `NotAuthorized`       | Invalid/missing auth token (future)     |
| 403         | `Forbidden`           | Not allowed to access resource (future) |
| 404         | `NotFound`            | Resource doesn't exist                  |
| 500         | `InternalServerError` | Unexpected server error                 |
| 503         | `ServiceUnavailable`  | Temporary server problem (future)       |

All responses use `ServiceError` schema:

```json
{
  "message": "Human-readable error message",
  "errors": [
    {
      "id": "optional-resource-id",
      "status": 404,
      "title": "Not Found",
      "detail": "Detailed error description"
    }
  ]
}
```

---

## Phase 6: Application State & Dependency Injection (Future)

**Status: ⏸️ NOT STARTED (Waiting for Phase 5)**

```rust
// main.rs
pub struct AppState {
    account_service: Arc<AccountService>,
    order_service: Arc<OrderService>,
    transaction_service: Arc<TransactionService>,
    user_preference_service: Arc<UserPreferenceService>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize database (file-based for server)
    let db_pool = crate::db::init_db("sqlite:paper_trader.db").await?;

    // Initialize repositories
    let account_repo = AccountRepository::new(db_pool.clone());
    let order_repo = OrderRepository::new(db_pool.clone());
    let transaction_repo = TransactionRepository::new(db_pool.clone());
    let preference_repo = UserPreferenceRepository::new(db_pool.clone());

    // Initialize services
    let account_service = Arc::new(AccountService::new(account_repo));
    let order_service = Arc::new(OrderService::new(order_repo));
    let transaction_service = Arc::new(TransactionService::new(transaction_repo));
    let preference_service = Arc::new(UserPreferenceService::new(preference_repo));

    let state = Arc::new(AppState {
        account_service,
        order_service,
        transaction_service,
        user_preference_service: preference_service,
    });

    // Start order executor background task (Phase 4)
    // tokio::spawn(async move { order_executor.run_execution_loop().await });

    let app = Router::new()
        .nest("/trader/v1", api::router())
        .with_state(state);

    // ... rest of setup
}
```

---

## Phase 7: SQLite → PostgreSQL Migration Path (Future) ));

    let state = Arc::new(AppState {
        db_pool,
        account_service,
        order_service,
        transaction_service,
    });

    let app = Router::new()
        .nest("/trader/v1", api::router())
        .layer(map_response(main_response_mapper))
        .with_state(state);

    // ... rest of setup

}

// api/mod.rs
pub fn router() -> Router<Arc<AppState>> {
Router::new()
.route("/accounts/accountNumbers", get(handlers::get_account_numbers))
.route("/accounts", get(handlers::get_accounts))
.route("/accounts/:accountNumber", get(handlers::get_account))
.route("/accounts/:accountNumber/orders",
get(handlers::get_orders_by_path_param)
.post(handlers::place_order))

---

## Implementation Progress Summary

### Phase 0: Account Management (Future) ⏸️ DEFERRED

**Status: Planning deferred until after core phases complete**

This phase addresses how accounts are created, initialized, reset, and managed outside of the standard Schwab API endpoints.

**Questions to Answer:**

- [ ] How are new paper trading accounts created?
  - Admin REST endpoints (e.g., `POST /admin/accounts`)?
  - CLI commands (e.g., `paper-trader create-account --type CASH --balance 100000`)?
  - Configuration file on startup?
- [ ] What are the initial account settings?
  - Starting cash balance (default $100,000? $1,000,000?)
  - Account type (CASH vs MARGIN)
  - Initial positions (start empty? seed with positions?)
- [ ] How are account numbers generated?
  - Sequential (100001, 100002, ...)?
  - Random 8-digit numbers?
  - User-provided?
- [ ] How are hash values generated?
  - SHA256 of account number?
  - Random UUID converted to hex?
  - Simple base64 encoding?
- [ ] Account lifecycle management:
  - Can accounts be deleted?
  - Can accounts be reset to initial state?
  - Can positions be manually added/removed?
  - Can balances be manually adjusted?
- [ ] Test fixtures and data:
  - Seed script for development/testing?
  - JSON fixture files?
  - SQL seed files?

**Proposed Admin Endpoints (To Be Designed):**

```
POST   /admin/accounts              # Create new account
DELETE /admin/accounts/{accountNumber}  # Delete account
POST   /admin/accounts/{accountNumber}/reset  # Reset to initial state
PUT    /admin/accounts/{accountNumber}/balance  # Manually adjust balance
POST   /admin/accounts/{accountNumber}/positions  # Manually add position
```

**This phase will be fully designed after Phase 1-2 are complete and we understand the account data structure better.**

---

### Phase 1: Foundation (Week 1) ✅ PLANNING COMPLETE → 🔄 READY TO IMPLEMENT

**Planning Decisions (see Phase 1 Planning Questions above):**

- [x] 1. Choose database library (sqlx vs diesel) ✅ **sqlx**
- [x] 2. Decide order ID generation strategy ✅ **AUTOINCREMENT starting at 1001**
- [x] 3. Decide initial account setup approach ✅ **No seeding initially**
- [x] 4. Decide account number/hash generation ✅ **Admin endpoints (Phase 0 - deferred)**
- [x] 5. Choose timestamp format ✅ **CURRENT_TIMESTAMP (UTC)**
- [x] 6. Decide transaction ID generation strategy ✅ **AUTOINCREMENT starting at 1001**

**Implementation Tasks:**

- [x] Create database migration file (001_initial_schema.sql) ✅
- [x] Set up database connection and migrations ✅
- [x] Implement AccountRepository with basic CRUD ✅
- [ ] Test with `/accounts/accountNumbers` endpoint (Phase 5)
- [ ] Test with `/accounts` endpoint (without positions) (Phase 5)
- [ ] Test with `/accounts/{accountNumber}` endpoint (Phase 5)

**Success Criteria:**

- [x] Database schema defined in migration ✅
- [x] Repository layer implemented ✅
- [ ] Can retrieve list of account numbers with hashes (Phase 5)
- [ ] Can retrieve all accounts (Phase 5)
- [ ] Can retrieve specific account by hash (Phase 5)
- [ ] Database persists data between restarts (Phase 5)

---

### Phase 2: Orders (Week 2) ⏸️ NOT STARTED

**Tasks:**

- [ ] Implement OrderRepository with full CRUD
- [ ] Implement OrderService with validation logic
- [ ] Update handlers for place/cancel/get/replace order
- [ ] Implement basic MARKET order execution (immediate fill)
- [ ] Test all order endpoints

**Success Criteria:**

- [ ] Can place market orders
- [ ] Can retrieve orders by account
- [ ] Can retrieve specific order by ID
- [ ] Can cancel orders
- [ ] Orders persist correctly

---

### Phase 3: Positions & Balances (Week 3) ⏸️ NOT STARTED

**Tasks:**

- [ ] Implement position tracking logic in AccountService
- [ ] Update account balances after order fills
- [ ] Calculate P&L for positions
- [ ] Test `/accounts?fields=positions` endpoint
- [ ] Test balance recalculation after trades

**Success Criteria:**

- [ ] Positions update correctly after fills
- [ ] Balances reflect order executions
- [ ] P&L calculations are accurate
- [ ] Can retrieve accounts with positions

---

### Phase 4: Advanced Orders (Week 4) ⏸️ NOT STARTED

**Tasks:**

- [ ] Implement LIMIT order execution
- [ ] Implement STOP and STOP_LIMIT orders
- [ ] Implement multi-leg orders (spreads)
- [ ] Implement complex order strategies (OCO, Bracket)
- [ ] Implement order preview with commission calculation
- [ ] Test `/accounts/{accountNumber}/previewOrder` endpoint

**Success Criteria:**

- [ ] All order types execute correctly
- [ ] Multi-leg orders work
- [ ] Order preview returns accurate estimates
- [ ] Complex strategies function properly

---

### Phase 5: Transactions (Week 5) ⏸️ NOT STARTED

**Tasks:**

- [ ] Implement TransactionRepository
- [ ] Generate transactions from order fills
- [ ] Implement transaction history endpoints
- [ ] Add support for corporate actions (dividends, splits)
- [ ] Test `/accounts/{accountNumber}/transactions` endpoint
- [ ] Test `/accounts/{accountNumber}/transactions/{transactionId}` endpoint

**Success Criteria:**

- [ ] Transactions generated for all order fills
- [ ] Can retrieve transaction history with filtering
- [ ] Can retrieve specific transaction by ID
- [ ] Date range filtering works correctly

---

### Phase 6: Polish & User Preferences (Week 6) ⏸️ NOT STARTED

**Tasks:**

- [ ] Implement PreferencesRepository
- [ ] Implement `/userPreference` endpoint
- [ ] Add comprehensive error handling
- [ ] Write integration tests for all endpoints
- [ ] Add logging and observability
- [ ] Write documentation

**Success Criteria:**

- [ ] User preferences can be retrieved/updated
- [ ] All 13 API endpoints working
- [ ] Integration test suite passing
- [ ] Documentation complete

---

## Key Design Decisions

### ✅ Confirmed Decisions (Phases 1-2 Complete)

1. **JSON Blobs for Complex Types**: Store full `OrderRequest`, `SecuritiesAccount`, `Transaction`, `UserPreference` as JSON to match API spec exactly
2. **4 Core Tables**: accounts, orders, transactions, user_preferences (not 8+ normalized tables)
3. **Database Library**: sqlx with runtime queries (no compile-time DATABASE_URL requirement)
4. **Order/Transaction IDs**: AUTOINCREMENT starting at 1001 for both order_id and activity_id
5. **Timestamps**: SQLite CURRENT_TIMESTAMP (UTC, ISO-8601) matching OpenAPI spec format
6. **Indexes**: Added on account_number, status, type, time fields for query filtering
7. **Repository Pattern**: Data access layer with custom error types per repository
8. **No Separate Position Table**: Positions stored in account_data JSON (matches API spec)
9. **In-Memory Testing**: Use `:memory:` SQLite for tests, file-based for server
10. **Service Layer Scope**: Phase 3 = thin CRUD wrapper, Phase 4 = order execution
11. **No Auth Initially**: Focus on CRUD and business logic first
12. **No Account Seeding**: Start with empty database, accounts created via admin endpoints

### 📋 Phase 3 Decisions

- **No PositionRepository**: Positions are part of account JSON (Question 1: Option A)
- **In-Memory for Tests**: File-based for server (Question 2: Option C)
- **Simple Services**: Just CRUD wrappers, no execution (Question 3: Option A)
- **No Market Data Yet**: Deferred to Phase 4 (Question 4: Option A)
- **Fixed Path**: Updated to `tools/paper/src/` (Question 5: Yes)

### 📋 TODO Items

- [ ] **Phase 0: Account Management** - Design account creation, initialization, reset, test fixtures (see Phase 0 section)
- [ ] **Index Review**: Optimize indexes after development/load testing
- [ ] **Phase 4: Order Execution** - Background task, market data service, fill logic
- [ ] **Phase 5: Handler Integration** - Connect services to Axum handlers
- [ ] **Phase 6: App State** - Dependency injection and main.rs setup

### ⏳ Deferred to Future Phases

- **Phase 0**: Account creation workflow, initial balances, reset/deletion, test fixtures
- **Phase 4**: Order execution simulator, market data service, order fills
- **Phase 7**: SQLite → PostgreSQL migration
- **Future**: WebSocket updates, market replay, analytics, multi-account portfolios

---

## Progress Tracking

### Completed ✅

- **Phase 1**: Database schema design and planning decisions
- **Phase 2**: Repository layer with all 4 repositories (accounts, orders, transactions, user_preference)

### Current 🔄

- **Phase 3**: Service layer (ready to start)

### Upcoming ⏸️

- **Phase 4**: Order execution and business logic
- **Phase 5**: Handler layer and API integration
- **Phase 6**: Application state and dependency injection
- **Phase 0**: Account management (deferred)

---

## Additional Notes

### Testing Strategy

- Unit tests for repositories ✅ (setup complete in Phase 2)
- Unit tests for services with in-memory database (Phase 3)
- Integration tests for services (Phase 3)
- E2E tests using existing handler tests (Phase 5)
- Mock market data for deterministic testing (Phase 4)

### Performance Considerations

- Connection pooling for database ✅ (implemented in Phase 2)
- Background task for order execution (Phase 4)
- Indexing on frequently queried fields ✅ (implemented in Phase 1)
- Consider caching for frequently accessed data (future optimization)

### Future Enhancements

- WebSocket support for real-time order updates
- Market replay functionality (replay historical market data)
- Advanced charting and analytics
- Multi-account portfolios
- Risk management features
- Admin dashboard for account management

---

## Order Execution Deep Dive: Design Decisions & Implementation Roadmap

**Status: 📋 ANALYSIS COMPLETE - IMPLEMENTATION DEFERRED**

This section outlines comprehensive design decisions for order execution and market data simulation. The current Phase 4 Part 2 implementation provides a basic framework, but full order execution requires careful consideration of many variables and edge cases.

### Overview: What Does Order Execution Mean?

Order execution in a paper trading system simulates the process of:

1. **Order Acceptance** - Validate and accept orders from users
2. **Order Queuing** - Store orders in WORKING state waiting for market conditions
3. **Market Monitoring** - Check current market prices for relevant symbols
4. **Fill Logic** - Determine when orders should fill based on type and price
5. **Account Updates** - Update positions, balances after fills
6. **Transaction Recording** - Create transaction records for audit trail
7. **Order Lifecycle** - Manage order states (WORKING → FILLED/CANCELED/EXPIRED/REJECTED)

### Key Design Questions

#### 1. What Order Types Should We Support?

The Schwab API defines many order types. We need to decide which to implement:

**Order Types from API** (from `OrderType` enum):

- `MARKET` - Fill at current market price immediately
- `LIMIT` - Fill when market reaches limit price
- `STOP` - Becomes market order when stop price hit
- `STOP_LIMIT` - Becomes limit order when stop price hit
- `TRAILING_STOP` - Stop price trails market by fixed amount or percentage
- `TRAILING_STOP_LIMIT` - Limit order with trailing stop
- `MARKET_ON_CLOSE` - Fill at closing price
- `LIMIT_ON_CLOSE` - Fill at limit price at close
- `CABINET` - Option order type for deep out-of-the-money options
- `NON_MARKETABLE` - Limit order that won't fill immediately
- `EXERCISE` - Exercise an option
- `NET_DEBIT` - Multi-leg option strategy
- `NET_CREDIT` - Multi-leg option strategy
- `NET_ZERO` - Multi-leg option strategy

#### 2. What Order Instructions Should We Support?

**Instructions from API** (from `Instruction` enum):

- `BUY` - Buy stock
- `SELL` - Sell stock
- `BUY_TO_COVER` - Buy to close short position
- `SELL_SHORT` - Sell stock short
- `BUY_TO_OPEN` - Buy option to open position
- `BUY_TO_CLOSE` - Buy option to close position
- `SELL_TO_OPEN` - Sell option to open position
- `SELL_TO_CLOSE` - Sell option to close position
- `EXCHANGE` - Exchange (for mutual funds)
- `SELL_SHORT_EXEMPT` - Short sale exempt from uptick rule

#### 3. What Order Durations Should We Support?

**Duration Types** (from `Duration` enum):

- `DAY` - Order expires at end of trading day
- `GOOD_TILL_CANCEL` (GTC) - Order active until filled or canceled
- `FILL_OR_KILL` (FOK) - Fill entire order immediately or cancel
- `IMMEDIATE_OR_CANCEL` (IOC) - Fill what you can immediately, cancel rest
- `END_OF_WEEK` - Expires end of week
- `END_OF_MONTH` - Expires end of month
- `NEXT_END_OF_MONTH` - Expires at next month end
- `UNKNOWN` - Unknown duration

#### 4. What Order Sessions Should We Support?

**Session Types** (from `Session` enum):

- `NORMAL` - Regular trading hours (9:30 AM - 4:00 PM ET)
- `AM` - Pre-market session
- `PM` - After-hours session
- `SEAMLESS` - Extended hours (pre-market + regular + after-hours)

#### 5. What Asset Types Should We Support?

**Asset Types** (from `AccountsInstrument` enum):

- `EQUITY` - Stocks
- `OPTION` - Options contracts
- `MUTUAL_FUND` - Mutual funds
- `FIXED_INCOME` - Bonds
- `CASH_EQUIVALENT` - Money market, etc.

#### 6. What Order Validations Are Needed?

**Pre-Placement Validation:**

- Account exists and is active
- Sufficient buying power for BUY orders
- Sufficient positions for SELL orders
- Symbol exists and is tradable
- Order quantity is positive and within limits
- Price/stop price are positive (if applicable)
- Order type is compatible with asset type
- Instruction is compatible with account type (e.g., no SELL_SHORT in cash accounts)

**During Execution Validation:**

- Order hasn't expired (check duration)
- Market is open (check session)
- Symbol is still tradable
- Account still has funds/positions

#### 7. How Should Market Data Be Simulated?

**Market Data Requirements:**

1. **Price Data:**

   - Current price (last traded price)
   - Bid/Ask prices and sizes
   - High/Low for the day
   - Open price
   - Previous close
   - Volume

2. **Market Hours:**

   - Is market currently open?
   - Pre-market hours (4:00 AM - 9:30 AM ET)
   - Regular hours (9:30 AM - 4:00 PM ET)
   - After-hours (4:00 PM - 8:00 PM ET)
   - Holidays (market closed)

3. **Price Movement Simulation:**

   - Random walk with volatility
   - Realistic bid/ask spreads
   - Time-based updates (every second? every minute?)
   - Historical price data for backtesting?

4. **Symbol Coverage:**
   - How many symbols to support?
   - Real-time data vs static prices?
   - Option chains?
   - Mutual fund NAV updates?

### Implementation Tiers: Basic → Full Featured

#### Tier 1: Minimal Viable Order Execution (Current Implementation)

**Scope:** Support only the most common use case for testing.

**Order Types:**

- ✅ MARKET - Implemented
- ✅ LIMIT - Implemented
- ✅ STOP - Implemented

**Instructions:**

- BUY (for stocks)
- SELL (for stocks)

**Asset Types:**

- EQUITY only

**Durations:**

- DAY only (all orders expire at end of day)

**Sessions:**

- NORMAL only (assume market always open for simplicity)

**Market Data:**

- ✅ Static base prices with small random variation
- ✅ 12 pre-configured symbols
- No bid/ask spreads
- No market hours checking
- No volume tracking

**Validations:**

- Basic: order has required fields
- No balance checking
- No position checking
- No market hours checking

**Account Updates:**

- ⚠️ Deferred (complex type handling)

**Transaction Generation:**

- ⚠️ Deferred (type structure unclear)

**Status:** ✅ This is what Phase 4 Part 2 implemented

#### Tier 2: Enhanced Paper Trading

**Scope:** Add enough realism for testing realistic trading scenarios.

**Additional Order Types:**

- STOP_LIMIT
- TRAILING_STOP
- MARKET_ON_CLOSE

**Additional Instructions:**

- SELL_SHORT (for margin accounts)
- BUY_TO_COVER

**Additional Durations:**

- GOOD_TILL_CANCEL (GTC)
- FILL_OR_KILL (FOK)
- IMMEDIATE_OR_CANCEL (IOC)

**Market Data Enhancements:**

- Bid/ask spreads (e.g., 0.01-0.05 spread)
- Market hours checking (reject orders outside hours)
- Extended hours support (pre-market, after-hours)
- 100+ symbol support

**Validations:**

- ✅ Balance checking for BUY orders
- ✅ Position checking for SELL orders
- Market hours validation
- Symbol existence validation
- Order type/instruction compatibility

**Account Updates:**

- ✅ Position tracking (add/update/remove)
- ✅ Balance updates (cash, buying power)
- Average cost basis tracking
- Realized P&L calculation

**Transaction Generation:**

- ✅ TRADE transactions for fills
- Fees/commissions (optional)
- Settlement date tracking

**Order Lifecycle:**

- Partial fills support
- Order expiration logic
- Order replacement tracking

#### Tier 3: Full-Featured Trading Simulation

**Scope:** Simulate real brokerage behavior as closely as possible.

**Additional Order Types:**

- All order types from API
- Multi-leg option strategies (NET_DEBIT, NET_CREDIT, NET_ZERO)

**Additional Instructions:**

- All option instructions (BUY_TO_OPEN, SELL_TO_CLOSE, etc.)
- EXCHANGE for mutual funds

**Additional Asset Types:**

- OPTIONS (with option chains)
- MUTUAL_FUND
- FIXED_INCOME
- CASH_EQUIVALENT

**Market Data Enhancements:**

- Realistic price movements (with historical volatility)
- Order book simulation (depth of market)
- Level 2 quotes
- Option pricing (Black-Scholes)
- Greek calculations for options
- Corporate actions (splits, dividends)

**Advanced Validations:**

- Pattern day trader rules
- Margin requirements
- Options approval levels
- Short sale uptick rule
- Wash sale rules

**Account Features:**

- Margin account with leverage
- Options trading levels
- Short positions tracking
- Pending orders impact on buying power
- Day trading buying power

**Advanced Order Features:**

- One-Cancels-Other (OCO)
- One-Triggers-Other (OTO)
- Bracket orders
- Conditional orders

**Transaction Types:**

- All transaction types (TRADE, RECEIVE_AND_DELIVER, DIVIDEND, INTEREST, etc.)
- Fee tracking
- Tax lot selection (FIFO, LIFO, Highest Cost, Specific Lot)

### Recommended Phasing Strategy

**Phase 4 Part 2 (Current):** Tier 1 - Minimal framework ✅

- Basic structure in place
- Core order types (MARKET, LIMIT, STOP)
- No account updates yet

**Phase 6 (After Handlers):** Complete Tier 1

- Implement account position updates
- Implement balance updates
- Implement transaction generation
- Integrate background execution loop
- Add basic order validation

**Phase 7 (Future):** Implement Tier 2

- Add more order types and durations
- Add market hours checking
- Add balance/position validation
- Add bid/ask spreads
- Support more symbols

**Phase 8 (Future):** Implement Tier 3 (Optional)

- Add options support
- Add margin accounts
- Add advanced order types
- Add corporate actions

### Critical Implementation Details

#### Account Updates: The Complex Part

The main challenge with account updates is the Schwab API type structure:

**SecuritiesAccount Enum:**

```rust
pub enum SecuritiesAccount {
    Margin(Box<MarginAccount>),
    Cash(Box<CashAccount>),
}
```

**Balance Fields Differ:**

- `MarginAccount.current_balances` → `MarginBalance` with fields:
  - `available_funds`, `buying_power`, `cash_available_for_trading`, etc.
  - NO field called `cash_balance`
- `CashAccount.current_balances` → `CashBalance` with fields:
  - `cash_available_for_trading`, `total_cash`, `cash_available_for_withdrawal`, etc.
  - NO field called `cash_balance`

**Position Updates:**

- Must handle `AccountsInstrument` enum (Equity, Option, MutualFund, etc.)
- Must create instrument details (symbol, cusip, description)
- Must track quantity, average_price, market_value
- Must differentiate long_quantity vs short_quantity

**Recommendation:**

- Start with simple cash account tracking
- Use `CashBalance.total_cash` for cash tracking
- Use `CashBalance.cash_available_for_trading` for buying power
- Defer margin account complexity to Tier 2+

#### Market Data: Keep It Simple

For paper trading, we don't need real-time data feeds. Recommendations:

**Tier 1:**

- Static prices with random ±1% variation (current implementation)
- 10-20 common symbols hardcoded
- Assume market always open

**Tier 2:**

- Pull real prices from free API (Alpha Vantage, Yahoo Finance) once per minute
- Cache prices in memory
- Check actual market hours
- Support 100+ symbols

**Tier 3:**

- Integrate with paid market data feed
- Real-time tick data
- Option chains
- Level 2 quotes

### Decision: What Should We Build?

**Recommendation: Start with Tier 1, Plan for Tier 2**

**Immediate Goals (Phase 6):**

1. Complete Tier 1 implementation
2. Focus on EQUITY + BUY/SELL + MARKET/LIMIT orders only
3. Simple cash account with `total_cash` tracking
4. Static market data with 20 symbols
5. No market hours checking (assume always open)
6. Basic validation (has funds, has position)

**Near-Term Goals (Phase 7):**

1. Add STOP_LIMIT and GTC duration
2. Add SELL_SHORT for margin accounts
3. Add market hours checking
4. Pull real prices from free API
5. Add bid/ask spreads
6. Support 100+ symbols

**Long-Term Goals (Phase 8+):**

1. Add options support (if needed)
2. Add multi-leg strategies (if needed)
3. Add advanced order types (if needed)

### Implementation Checklist for Phase 6

When we revisit order execution in Phase 6, implement in this order:

- [ ] **Step 1:** Fix account update logic for CashAccount

  - Map to correct balance fields (`total_cash`, not `cash_balance`)
  - Update positions array correctly
  - Handle AccountsInstrument enum properly

- [ ] **Step 2:** Fix transaction generation

  - Study Transaction type structure
  - Map fields correctly for TRADE transactions
  - Set symbol, quantity, price, amount fields

- [ ] **Step 3:** Add order validation in OrderService

  - Check sufficient `cash_available_for_trading` for BUY
  - Check sufficient position quantity for SELL
  - Return InvalidInput error if validation fails

- [ ] **Step 4:** Integrate OrderExecutor into app state

  - Add to AppState struct
  - Start background execution loop in main()
  - Add method to fetch all WORKING orders

- [ ] **Step 5:** Test full lifecycle

  - Place MARKET order → verify immediate fill → verify account update
  - Place LIMIT order → verify waits for price → verify fills correctly
  - Place order without funds → verify rejection
  - Sell without position → verify rejection

- [ ] **Step 6:** Add more symbols to MarketDataService
  - Expand from 12 to 50+ common symbols
  - Consider pulling from CSV file or API

### References: Schwab API Order Fields

For reference, here are all the order-related fields from the API:

**Order Request Fields:**

- `session`, `duration`, `orderType`
- `complexOrderStrategyType`
- `quantity`, `filledQuantity`, `remainingQuantity`
- `requestedDestination`, `destinationLinkName`
- `price`, `stopPrice`, `stopType`
- `priceLinkBasis`, `priceLinkType`
- `stopPriceOffset`, `stopPriceLinkBasis`, `stopPriceLinkType`
- `taxLotMethod`
- `orderLegCollection[]` - array of legs with:
  - `instruction`, `quantity`, `instrument`
  - `positionEffect`, `quantityType`
- `orderStrategyType`
- `activationPrice`
- `specialInstruction`
- `childOrderStrategies[]` - nested orders

**Order Response Fields (additional):**

- `orderId`, `accountNumber`
- `status`, `enteredTime`, `closeTime`
- `cancelable`, `editable`
- `orderActivityCollection[]` - execution history
- `replacingOrderCollection[]` - replacement tracking
- `statusDescription`

This is a LOT of complexity. Start simple, iterate based on actual testing needs.

---

This plan provides a production-ready paper trading system that perfectly mimics the Schwab Trader API!
