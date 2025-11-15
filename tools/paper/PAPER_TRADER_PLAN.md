# Paper Trader Application Implementation Plan

A comprehensive guide for building a "drop-in" replacement mock server for the Schwab Trader API using SQLite (with easy PostgreSQL migration path).

---

## Project Architecture Overview

Your paper trader will be a drop-in replacement mock server that implements all 13 Schwab Trader API endpoints with SQLite persistence.

---

## Phase 1: Database Schema Design

### Core Tables

```sql
-- Accounts table
CREATE TABLE accounts (
    id INTEGER PRIMARY KEY,
    account_number TEXT UNIQUE NOT NULL,
    account_hash TEXT UNIQUE NOT NULL,  -- encrypted value
    account_type TEXT NOT NULL,  -- CASH, MARGIN
    is_day_trader BOOLEAN DEFAULT 0,
    is_closing_only_restricted BOOLEAN DEFAULT 0,
    round_trips INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Balances table (current state)
CREATE TABLE account_balances (
    account_id INTEGER PRIMARY KEY REFERENCES accounts(id),
    cash_balance REAL DEFAULT 0,
    cash_available_for_trading REAL DEFAULT 0,
    cash_available_for_withdrawal REAL DEFAULT 0,
    liquidation_value REAL DEFAULT 0,
    long_market_value REAL DEFAULT 0,
    short_market_value REAL DEFAULT 0,
    -- ... other balance fields from spec
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Positions table
CREATE TABLE positions (
    id INTEGER PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id),
    symbol TEXT NOT NULL,
    asset_type TEXT NOT NULL,  -- EQUITY, OPTION, MUTUAL_FUND, etc.
    quantity REAL NOT NULL,
    average_price REAL NOT NULL,
    market_value REAL,
    day_pl REAL DEFAULT 0,
    day_pl_percent REAL DEFAULT 0,
    -- instrument details as JSON
    instrument_data TEXT,  -- JSON blob
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Orders table
CREATE TABLE orders (
    id INTEGER PRIMARY KEY,
    order_id TEXT UNIQUE NOT NULL,  -- external facing ID
    account_id INTEGER REFERENCES accounts(id),
    status TEXT NOT NULL,  -- WORKING, FILLED, CANCELED, etc.
    order_type TEXT NOT NULL,  -- MARKET, LIMIT, STOP, etc.
    session TEXT NOT NULL,  -- NORMAL, AM, PM
    duration TEXT NOT NULL,  -- DAY, GTC, etc.
    price REAL,
    stop_price REAL,
    quantity REAL NOT NULL,
    filled_quantity REAL DEFAULT 0,
    remaining_quantity REAL,
    strategy_type TEXT,  -- SINGLE, OCO, TRIGGER, etc.
    -- full order JSON for complex orders
    order_data TEXT,  -- JSON blob of full OrderRequest
    entered_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    close_time TIMESTAMP,
    cancelable BOOLEAN DEFAULT 1,
    editable BOOLEAN DEFAULT 1
);

-- Order legs (for multi-leg orders)
CREATE TABLE order_legs (
    id INTEGER PRIMARY KEY,
    order_id INTEGER REFERENCES orders(id),
    instruction TEXT NOT NULL,  -- BUY, SELL, etc.
    symbol TEXT NOT NULL,
    asset_type TEXT NOT NULL,
    quantity REAL NOT NULL,
    position_effect TEXT,  -- OPENING, CLOSING
    instrument_data TEXT  -- JSON blob
);

-- Executions (fills)
CREATE TABLE executions (
    id INTEGER PRIMARY KEY,
    order_id INTEGER REFERENCES orders(id),
    execution_type TEXT NOT NULL,  -- FILL
    quantity REAL NOT NULL,
    price REAL NOT NULL,
    execution_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Transactions
CREATE TABLE transactions (
    id INTEGER PRIMARY KEY,
    transaction_id TEXT UNIQUE NOT NULL,
    account_id INTEGER REFERENCES accounts(id),
    transaction_type TEXT NOT NULL,
    description TEXT,
    amount REAL NOT NULL,
    net_amount REAL,
    -- related order if applicable
    order_id INTEGER REFERENCES orders(id),
    -- full transaction JSON
    transaction_data TEXT,  -- JSON blob
    transaction_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- User preferences
CREATE TABLE user_preferences (
    id INTEGER PRIMARY KEY,
    express_trading BOOLEAN DEFAULT 0,
    direct_options_routing BOOLEAN DEFAULT 0,
    direct_equity_routing BOOLEAN DEFAULT 0,
    default_equity_order_leg_instruction TEXT,
    default_equity_order_type TEXT,
    default_equity_order_price_link_type TEXT,
    default_equity_order_duration TEXT,
    default_equity_order_market_session TEXT,
    -- ... other preference fields
    preferences_data TEXT  -- JSON blob for full UserPreference object
);
```

---

## Phase 2: Database Layer (Repository Pattern)

### File Structure

```
tools/trader/src/
├── db/
│   ├── mod.rs           # Database connection manager
│   ├── schema.sql       # SQLite schema
│   ├── migrations.rs    # Migration runner
│   └── repositories/
│       ├── mod.rs
│       ├── accounts.rs
│       ├── orders.rs
│       ├── positions.rs
│       ├── transactions.rs
│       └── preferences.rs
```

### Dependencies to Add

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "macros", "chrono"] }
# OR for easier PostgreSQL migration later:
diesel = { version = "2.2", features = ["sqlite", "chrono", "r2d2"] }
diesel_migrations = "2.2"
```

### Repository Pattern Example

```rust
// db/repositories/accounts.rs
pub struct AccountRepository {
    pool: SqlitePool,  // or PgPool later
}

impl AccountRepository {
    pub async fn create(&self, account: NewAccount) -> Result<Account>;
    pub async fn find_by_hash(&self, hash: &str) -> Result<Option<Account>>;
    pub async fn list_all(&self) -> Result<Vec<Account>>;
    pub async fn get_with_positions(&self, hash: &str) -> Result<AccountWithPositions>;
    pub async fn update_balance(&self, id: i64, balance: Balance) -> Result<()>;
}

// db/repositories/orders.rs
pub struct OrderRepository {
    pool: SqlitePool,
}

impl OrderRepository {
    pub async fn create(&self, order: NewOrder) -> Result<Order>;
    pub async fn find_by_id(&self, order_id: &str) -> Result<Option<Order>>;
    pub async fn list_by_account(&self, account_id: i64, params: OrderQueryParams) -> Result<Vec<Order>>;
    pub async fn update_status(&self, order_id: &str, status: OrderStatus) -> Result<()>;
    pub async fn cancel(&self, order_id: &str) -> Result<()>;
    pub async fn add_execution(&self, order_id: &str, execution: Execution) -> Result<()>;
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

### Phase 1: Foundation (Week 1)

- [ ] Database schema
- [ ] Repository pattern implementation
- [ ] Basic account CRUD
- [ ] Test with `/accounts` endpoints

### Phase 2: Orders (Week 2)

- [ ] Order repository
- [ ] Order service with validation
- [ ] Place/cancel/get order handlers
- [ ] Basic MARKET order execution

### Phase 3: Positions & Balances (Week 3)

- [ ] Position tracking
- [ ] Balance calculations
- [ ] Order execution updates positions
- [ ] P&L calculations

### Phase 4: Advanced Orders (Week 4)

- [ ] LIMIT, STOP, STOP_LIMIT orders
- [ ] Multi-leg orders (spreads)
- [ ] OCO, Bracket orders
- [ ] Order preview with commission calculation

### Phase 5: Transactions (Week 5)

- [ ] Transaction generation from fills
- [ ] Transaction history
- [ ] Corporate actions (dividends, splits)

### Phase 6: Polish (Week 6)

- [ ] User preferences
- [ ] Error handling refinement
- [ ] Integration tests
- [ ] Documentation

---

## Key Design Decisions

1. **JSON Blobs for Complex Types**: Store full `OrderRequest`, `Account`, etc. as JSON to avoid over-normalization
2. **Repository Pattern**: Easy to swap SQLite → PostgreSQL
3. **Service Layer**: Business logic separate from handlers and database
4. **Background Order Executor**: Separate tokio task simulates fills
5. **Mock Market Data**: Start simple, can integrate real feeds later
6. **No Auth Initially**: Focus on CRUD and business logic first

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
