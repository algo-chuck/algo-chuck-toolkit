-- Paper Trader Database Schema
-- SQLite schema for paper trading application
-- Follows Schwab Trader API OpenAPI specification

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
