# Paper Trader Application Implementation Plan

A comprehensive guide for building a "drop-in" replacement mock server for the Schwab Trader API using SQLite (with easy PostgreSQL migration path).

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

**Status: 🔄 READY TO START (Phase 1 decisions complete)**

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

### File Structure

```
tools/paper/src/
├── db/
│   ├── mod.rs                # Database connection manager
│   ├── schema.sql            # SQLite schema (from Phase 1)
│   ├── migrations/           # SQL migration files
│   │   └── 001_initial_schema.sql
│   └── repositories/
│       ├── mod.rs            # Re-exports all repositories
│       ├── accounts.rs       # AccountRepository (tag: Accounts)
│       ├── orders.rs         # OrderRepository (tag: Orders)
│       ├── transactions.rs   # TransactionRepository (tag: Transactions)
│       └── user_preference.rs # UserPreferenceRepository (tag: User Preference)
```

### Dependencies to Add

**Using sqlx (Decision from Phase 1):**

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "macros", "chrono"] }
serde_json = "1.0"  # For JSON serialization of account_data, order_data, etc.
chrono = { version = "0.4", features = ["serde"] }  # For timestamp handling
```

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

| Tag                 | OpenAPI operationId          | Repository Method                           |
| ------------------- | ---------------------------- | ------------------------------------------- |
| **Accounts**        | `getAccountNumbers`          | `get_account_numbers()`                     |
|                     | `getAccounts`                | `get_accounts()`                            |
|                     | `getAccount`                 | `get_account()`                             |
| **Orders**          | `getOrdersByPathParam`       | `get_orders_by_path_param()`                |
|                     | `placeOrder`                 | `place_order()`                             |
|                     | `getOrder`                   | `get_order()`                               |
|                     | `cancelOrder`                | `cancel_order()`                            |
|                     | `replaceOrder`               | `replace_order()`                           |
|                     | `getOrdersByQueryParam`      | `get_orders_by_query_param()`               |
|                     | `previewOrder`               | _(handled in service layer, not persisted)_ |
| **Transactions**    | `getTransactionsByPathParam` | `get_transactions_by_path_param()`          |
|                     | `getTransactionsById`        | `get_transactions_by_id()`                  |
| **User Preference** | `getUserPreference`          | `get_user_preference()`                     |

---

## Phase 3: Business Logic Layer (Services)

### File Structure

```
tools/trader/src/
├── services/
│   ├── mod.rs
│   ├── account_service.rs
│   ├── order_service.rs
│   ├── transaction_service.rs
│   └── market_data_service.rs  # Mock price feeds
```

### Key Services

```rust
// services/order_service.rs
pub struct OrderService {
    order_repo: OrderRepository,
    account_repo: AccountRepository,
    position_repo: PositionRepository,
    transaction_repo: TransactionRepository,
    market_data: MarketDataService,
}

impl OrderService {
    // Place order: validate, persist, simulate fill
    pub async fn place_order(&self, account_hash: &str, order: OrderRequest) -> Result<String>;

    // Cancel order: validate cancelable, update status
    pub async fn cancel_order(&self, account_hash: &str, order_id: &str) -> Result<()>;

    // Replace order: cancel old, place new
    pub async fn replace_order(&self, account_hash: &str, order_id: &str, new_order: OrderRequest) -> Result<String>;

    // Preview order: validate, calculate commissions/fees
    pub async fn preview_order(&self, account_hash: &str, order: PreviewOrderRequest) -> Result<PreviewOrder>;

    // Order execution simulator (background task)
    pub async fn process_pending_orders(&self) -> Result<()>;
}

// services/account_service.rs
pub struct AccountService {
    account_repo: AccountRepository,
    position_repo: PositionRepository,
}

impl AccountService {
    pub async fn get_account_numbers(&self) -> Result<Vec<AccountNumberHash>>;
    pub async fn get_accounts(&self, include_positions: bool) -> Result<Vec<Account>>;
    pub async fn get_account(&self, hash: &str, include_positions: bool) -> Result<Account>;

    // Update account balances based on positions
    pub async fn recalculate_balances(&self, account_id: i64) -> Result<()>;
}
```

---

## Phase 4: Handler Implementation

Update your handlers to use services:

```rust
// handlers/accounts.rs
use crate::services::AccountService;

pub async fn get_account_numbers(
    State(account_service): State<Arc<AccountService>>
) -> Result<Json<Vec<AccountNumberHash>>> {
    let accounts = account_service.get_account_numbers().await?;
    Ok(Json(accounts))
}

pub async fn get_accounts(
    Query(params): Query<GetAccountsQuery>,
    State(account_service): State<Arc<AccountService>>
) -> Result<Json<Vec<Account>>> {
    let include_positions = params.fields.as_deref() == Some("positions");
    let accounts = account_service.get_accounts(include_positions).await?;
    Ok(Json(accounts))
}

pub async fn get_account(
    Path(account_hash): Path<String>,
    Query(params): Query<GetAccountQuery>,
    State(account_service): State<Arc<AccountService>>
) -> Result<Json<Account>> {
    let include_positions = params.fields.as_deref() == Some("positions");
    let account = account_service.get_account(&account_hash, include_positions).await?;
    Ok(Json(account))
}

// handlers/orders.rs
pub async fn place_order(
    Path(account_hash): Path<String>,
    State(order_service): State<Arc<OrderService>>,
    Json(order): Json<OrderRequest>
) -> Result<Created> {
    let order_id = order_service.place_order(&account_hash, order).await?;
    Ok(Created { order_id })  // Could add Location header
}

pub async fn cancel_order(
    Path((account_hash, order_id)): Path<(String, String)>,
    State(order_service): State<Arc<OrderService>>
) -> Result<EmptyOK> {
    order_service.cancel_order(&account_hash, &order_id).await?;
    Ok(EmptyOK {})
}
```

---

## Phase 5: Application State & Dependency Injection

```rust
// main.rs
pub struct AppState {
    db_pool: SqlitePool,
    account_service: Arc<AccountService>,
    order_service: Arc<OrderService>,
    transaction_service: Arc<TransactionService>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize database
    let db_pool = SqlitePool::connect("sqlite:paper_trader.db").await?;
    sqlx::migrate!("./migrations").run(&db_pool).await?;

    // Initialize repositories
    let account_repo = AccountRepository::new(db_pool.clone());
    let order_repo = OrderRepository::new(db_pool.clone());
    let position_repo = PositionRepository::new(db_pool.clone());
    let transaction_repo = TransactionRepository::new(db_pool.clone());

    // Initialize services
    let market_data = MarketDataService::new();
    let account_service = Arc::new(AccountService::new(account_repo, position_repo));
    let order_service = Arc::new(OrderService::new(
        order_repo,
        account_repo,
        position_repo,
        transaction_repo,
        market_data,
    ));

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
        // ... other routes
}
```

---

## Phase 6: Order Execution Simulator

```rust
// services/order_executor.rs
pub struct OrderExecutor {
    order_service: Arc<OrderService>,
    market_data: MarketDataService,
}

impl OrderExecutor {
    // Background task that runs every N seconds
    pub async fn run_execution_loop(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;

            if let Err(e) = self.process_pending_orders().await {
                eprintln!("Order execution error: {}", e);
            }
        }
    }

    async fn process_pending_orders(&self) -> Result<()> {
        // Get all WORKING orders
        let pending = self.order_service.get_pending_orders().await?;

        for order in pending {
            // Get current market price
            let price = self.market_data.get_current_price(&order.symbol).await?;

            // Check if order should fill
            if self.should_fill(&order, price) {
                self.execute_order(order, price).await?;
            }
        }

        Ok(())
    }

    fn should_fill(&self, order: &Order, current_price: f64) -> bool {
        match order.order_type.as_str() {
            "MARKET" => true,  // Market orders fill immediately
            "LIMIT" if order.instruction == "BUY" => {
                current_price <= order.price.unwrap_or(f64::MAX)
            },
            "LIMIT" if order.instruction == "SELL" => {
                current_price >= order.price.unwrap_or(0.0)
            },
            "STOP" if order.instruction == "BUY" => {
                current_price >= order.stop_price.unwrap_or(f64::MAX)
            },
            "STOP" if order.instruction == "SELL" => {
                current_price <= order.stop_price.unwrap_or(0.0)
            },
            _ => false,
        }
    }
}
```

---

## Phase 7: Mock Market Data Service

```rust
// services/market_data_service.rs
pub struct MarketDataService {
    // Could use external API or generate random prices
}

impl MarketDataService {
    pub async fn get_current_price(&self, symbol: &str) -> Result<f64> {
        // Option 1: Use a real API (Alpha Vantage, IEX, etc.)
        // Option 2: Generate realistic random walks
        // Option 3: Use static prices for testing

        // For now, simple mock:
        Ok(match symbol {
            "AAPL" => 175.50,
            "TSLA" => 245.30,
            "SPY" => 450.25,
            _ => 100.0,  // default
        })
    }
}
```

---

## Phase 8: SQLite → PostgreSQL Migration Path

### Using SQLx (Easier Migration)

```rust
// Abstract database behind trait
#[async_trait]
pub trait Database: Send + Sync {
    async fn get_account(&self, id: i64) -> Result<Account>;
    // ... other methods
}

// SQLite implementation
pub struct SqliteDatabase {
    pool: SqlitePool,
}

// PostgreSQL implementation (later)
pub struct PostgresDatabase {
    pool: PgPool,
}

// Both implement the same trait
// Just swap the implementation in main.rs
```

### Using Diesel (Type-Safe Migration)

```rust
// Diesel supports both with same schema
// Just change connection string:
// SQLite: "file:paper_trader.db"
// PostgreSQL: "postgres://user:pass@localhost/paper_trader"

// Schema is database-agnostic with minor tweaks
```

---

## Implementation Phases

### Phase 0: Account Management (Future) ⏸️ DEFERRED

**Status: Planning deferred until after Phase 1-2 are working**

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

- [ ] 1. Choose database library (sqlx vs diesel)
- [ ] 2. Decide order ID generation strategy
- [ ] 3. Decide initial account setup approach
- [ ] 4. Decide account number/hash generation
- [ ] 5. Choose timestamp format
- [ ] 6. Decide transaction ID generation strategy

**Implementation Tasks:**

- [ ] Create database schema SQL file
- [ ] Set up database connection and migrations
- [ ] Implement AccountRepository with basic CRUD
- [ ] Test with `/accounts/accountNumbers` endpoint
- [ ] Test with `/accounts` endpoint (without positions)
- [ ] Test with `/accounts/{accountNumber}` endpoint

**Success Criteria:**

- [ ] Can retrieve list of account numbers with hashes
- [ ] Can retrieve all accounts
- [ ] Can retrieve specific account by hash
- [ ] Database persists data between restarts

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

### ✅ Confirmed Decisions (Phase 1 Complete)

1. **JSON Blobs for Complex Types**: Store full `OrderRequest`, `SecuritiesAccount`, `Transaction`, `UserPreference` as JSON to match API spec exactly
2. **4 Core Tables**: accounts, orders, transactions, user_preferences (not 8+ normalized tables)
3. **Database Library**: sqlx for async/await, compile-time SQL checking, flexibility
4. **Order/Transaction IDs**: AUTOINCREMENT starting at 1001 for both order_id and activity_id
5. **Timestamps**: SQLite CURRENT_TIMESTAMP (UTC, ISO-8601) matching OpenAPI spec format
6. **Indexes**: Added on account_number, status, type, time fields for query filtering
7. **Repository Pattern**: Easy to swap SQLite → PostgreSQL by changing connection string
8. **Service Layer**: Business logic separate from handlers and database
9. **Background Order Executor**: Separate tokio task simulates fills
10. **No Auth Initially**: Focus on CRUD and business logic first
11. **No Account Seeding**: Start with empty database, accounts created via admin endpoints

### 📋 TODO Items

- [ ] **Phase 0: Account Management** - Design account creation, initialization, reset, test fixtures (see Phase 0 section)
- [ ] **Index Review**: Optimize indexes after development/load testing
- [ ] **Sequence Initialization**: Implement sqlite_sequence setup for order_id/activity_id starting at 1001

### ⏳ Deferred to Phase 0 (Account Management)

- Account creation workflow (admin endpoints design)
- Initial account balances and positions
- Account reset/deletion strategy
- Test data fixture generation

---

## Additional Notes

### Testing Strategy

- Unit tests for repositories
- Integration tests for services
- E2E tests using the `dev_client.rs` test suite
- Mock market data for deterministic testing

### Performance Considerations

- Connection pooling for database
- Background task for order execution (separate from request handlers)
- Indexing on frequently queried fields (account_hash, order_id, symbol)
- Consider caching for frequently accessed data

### Future Enhancements

- WebSocket support for real-time order updates
- Market replay functionality (replay historical market data)
- Advanced charting and analytics
- Multi-account portfolios
- Risk management features

---

This plan provides a production-ready paper trading system that perfectly mimics the Schwab Trader API!
