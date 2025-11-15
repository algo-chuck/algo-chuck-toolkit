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
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table 2: Orders
-- Stores full Order/OrderRequest JSON
-- Order contains orderLegCollection, orderActivityCollection, childOrderStrategies inline
CREATE TABLE orders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id INTEGER NOT NULL,                -- from Order.orderId (int64 in spec)
    account_number TEXT NOT NULL,             -- from Order.accountNumber (int64 in spec, but we use TEXT for FK)
    status TEXT NOT NULL,                     -- from Order.status enum (WORKING, FILLED, CANCELED, etc.)
    entered_time TIMESTAMP,                   -- from Order.enteredTime (ISO-8601)
    close_time TIMESTAMP,                     -- from Order.closeTime (ISO-8601)
    order_data TEXT NOT NULL,                 -- Full Order/OrderRequest JSON
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (account_number) REFERENCES accounts(account_number)
);

-- Table 3: Transactions
-- Stores full Transaction JSON
-- Transaction contains transferItems array, user object inline
CREATE TABLE transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    activity_id INTEGER NOT NULL,            -- from Transaction.activityId (int64 in spec)
    account_number TEXT NOT NULL,            -- from Transaction.accountNumber (string in spec)
    type TEXT NOT NULL,                      -- from Transaction.type enum (TransactionType)
    time TIMESTAMP,                          -- from Transaction.time (ISO-8601)
    transaction_data TEXT NOT NULL,          -- Full Transaction JSON
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (account_number) REFERENCES accounts(account_number)
);

-- Table 4: User Preferences
-- Stores full UserPreference JSON
-- UserPreference contains accounts array, streamerInfo array, offers array inline
CREATE TABLE user_preferences (
    id INTEGER PRIMARY KEY,
    preference_data TEXT NOT NULL,           -- Full UserPreference JSON
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
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

**Status: ⏳ AWAITING DECISIONS**

#### 1. Database Library Choice

- **Option A: sqlx** - Async-first, compile-time checked SQL, works with raw SQL
  - Pros: Modern async/await, flexible, no ORM overhead
  - Cons: Need to write SQL by hand
- **Option B: diesel** - Type-safe ORM, can be sync or async
  - Pros: Strong typing, migrations built-in, less SQL writing
  - Cons: Steeper learning curve, more boilerplate

**Decision:** [ ]

#### 2. Order ID Generation

Real Schwab API uses `int64` for `orderId`. Options:

- **Option A:** Use SQLite AUTOINCREMENT as the order ID
- **Option B:** Generate random int64s (e.g., timestamp-based or UUID as int)
- **Option C:** Use sequential counter starting from 1000000

**Decision:** [ ]

#### 3. Initial Account Setup

- **Option A:** Seed database with 2-3 sample accounts on first run
  - Example: 1 CASH account, 1 MARGIN account with positions
- **Option B:** Start with empty database, require manual account creation
- **Option C:** Add admin endpoint to create accounts (e.g., POST /admin/accounts)

**Decision:** [ ]

#### 4. Account Number/Hash Relationship

The `/accounts/accountNumbers` endpoint returns `{ accountNumber, hashValue }[]`

- **Option A:** Generate account numbers and hashes on startup/first run
- **Option B:** Let users create accounts via separate admin endpoint
- **Option C:** Use fixed account numbers for testing (e.g., "12345678", "87654321")

**Decision:** [ ]

#### 5. Timestamp Format

- **Option A:** Use SQLite's `CURRENT_TIMESTAMP` (UTC, ISO-8601 format)
- **Option B:** Store as Unix epoch integers (easier for calculations)
- **Option C:** Store as TEXT in ISO-8601 format for human readability

**Decision:** [ ]

#### 6. Transaction ID Generation

Similar to Order IDs, Transaction.activityId is int64:

- **Option A:** Use SQLite AUTOINCREMENT
- **Option B:** Generate timestamp-based int64s
- **Option C:** Use sequential counter

**Decision:** [ ]

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

**Status: ⏸️ PENDING (Blocked by Phase 1 decisions)**

### File Structure

```
tools/paper/src/
├── db/
│   ├── mod.rs           # Database connection manager
│   ├── schema.sql       # SQLite schema (from Phase 1)
│   ├── migrations/      # SQL migration files
│   │   └── 001_initial_schema.sql
│   └── repositories/
│       ├── mod.rs
│       ├── accounts.rs
│       ├── orders.rs
│       ├── transactions.rs
│       └── preferences.rs
```

### Dependencies to Add

**DECISION NEEDED: Choose between sqlx OR diesel**

**Option A: sqlx**

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "macros", "chrono"] }
serde_json = "1.0"  # For JSON serialization
```

**Option B: diesel**

```toml
[dependencies]
diesel = { version = "2.2", features = ["sqlite", "chrono", "r2d2"] }
diesel_migrations = "2.2"
serde_json = "1.0"  # For JSON serialization
```

### Repository Pattern Examples

**NOTE: These examples will be updated once database library choice is made**

```rust
// db/repositories/accounts.rs
use sqlx::SqlitePool;  // OR: use diesel::SqliteConnection;
use serde_json;

pub struct AccountRepository {
    pool: SqlitePool,  // OR: PgPool for PostgreSQL later
}

impl AccountRepository {
    pub async fn create(&self, account_number: &str, hash_value: &str, account_type: &str, account_data: &SecuritiesAccount) -> Result<i64>;
    pub async fn find_by_hash(&self, hash: &str) -> Result<Option<SecuritiesAccount>>;
    pub async fn find_by_account_number(&self, account_number: &str) -> Result<Option<SecuritiesAccount>>;
    pub async fn list_all(&self, include_positions: bool) -> Result<Vec<SecuritiesAccount>>;
    pub async fn update(&self, account_number: &str, account_data: &SecuritiesAccount) -> Result<()>;
}

// db/repositories/orders.rs
pub struct OrderRepository {
    pool: SqlitePool,
}

impl OrderRepository {
    pub async fn create(&self, account_number: &str, order_data: &OrderRequest) -> Result<i64>;  // Returns order_id
    pub async fn find_by_id(&self, order_id: i64) -> Result<Option<Order>>;
    pub async fn list_by_account(&self, account_number: &str, from_date: Option<DateTime>, to_date: Option<DateTime>, status: Option<String>) -> Result<Vec<Order>>;
    pub async fn update_status(&self, order_id: i64, status: &str) -> Result<()>;
    pub async fn update(&self, order_id: i64, order_data: &Order) -> Result<()>;
    pub async fn delete(&self, order_id: i64) -> Result<()>;  // For cancel
}

// db/repositories/transactions.rs
pub struct TransactionRepository {
    pool: SqlitePool,
}

impl TransactionRepository {
    pub async fn create(&self, account_number: &str, transaction_type: &str, transaction_data: &Transaction) -> Result<i64>;  // Returns activity_id
    pub async fn find_by_id(&self, activity_id: i64) -> Result<Option<Transaction>>;
    pub async fn list_by_account(&self, account_number: &str, start_date: &str, end_date: &str, transaction_type: Option<&str>) -> Result<Vec<Transaction>>;
}

// db/repositories/preferences.rs
pub struct PreferencesRepository {
    pool: SqlitePool,
}

impl PreferencesRepository {
    pub async fn get(&self) -> Result<Option<UserPreference>>;
    pub async fn upsert(&self, preference_data: &UserPreference) -> Result<()>;
}
```

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

### Phase 1: Foundation (Week 1) ⏳ IN PLANNING

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

### ✅ Confirmed Decisions

1. **JSON Blobs for Complex Types**: Store full `OrderRequest`, `SecuritiesAccount`, `Transaction`, `UserPreference` as JSON to match API spec exactly
2. **4 Core Tables**: accounts, orders, transactions, user_preferences (not 8+ normalized tables)
3. **Repository Pattern**: Easy to swap SQLite → PostgreSQL by changing connection string
4. **Service Layer**: Business logic separate from handlers and database
5. **Background Order Executor**: Separate tokio task simulates fills
6. **No Auth Initially**: Focus on CRUD and business logic first

### ⏳ Pending Decisions (Phase 1)

- Database library choice (sqlx vs diesel)
- Order ID generation strategy
- Transaction ID generation strategy
- Initial account setup approach
- Account number/hash generation
- Timestamp format

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
