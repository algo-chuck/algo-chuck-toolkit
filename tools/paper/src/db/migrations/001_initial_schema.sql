-- Initial schema for paper trader application
-- Creates all 4 core tables: accounts, orders, transactions, user_preferences

-- Table 1: Accounts
CREATE TABLE accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_number TEXT UNIQUE NOT NULL,
    hash_value TEXT UNIQUE NOT NULL,
    account_type TEXT NOT NULL,
    account_data TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table 2: Orders
CREATE TABLE orders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id INTEGER NOT NULL,
    account_number TEXT NOT NULL,
    status TEXT NOT NULL,
    entered_time TIMESTAMP,
    close_time TIMESTAMP,
    order_data TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (account_number) REFERENCES accounts(account_number)
);

-- Table 3: Transactions
CREATE TABLE transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    activity_id INTEGER NOT NULL,
    account_number TEXT NOT NULL,
    type TEXT NOT NULL,
    time TIMESTAMP,
    transaction_data TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (account_number) REFERENCES accounts(account_number)
);

-- Table 4: User Preferences
CREATE TABLE user_preferences (
    id INTEGER PRIMARY KEY,
    preference_data TEXT NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for common queries
CREATE INDEX idx_orders_account_number ON orders(account_number);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_entered_time ON orders(entered_time);
CREATE INDEX idx_transactions_account_number ON transactions(account_number);
CREATE INDEX idx_transactions_type ON transactions(type);
CREATE INDEX idx_transactions_time ON transactions(time);
