### Phase 0: Account Management

This phase addresses how accounts are created, initialized, reset, and managed outside of the standard Schwab API endpoints.

## Design Decisions ✅

### 1. Account Creation Method

**Decision:** Admin REST endpoints only

- `POST /admin/v1/accounts` - Create new account
- No CLI commands
- No configuration file
- No automatic seeding on startup

### 2. Initial Account Settings

**Decision:** Simple, learning-focused defaults

- **Starting cash balance:** $200,000 (fixed default)
- **Account type:** CASH only (MARGIN not supported yet)
- **Initial positions:** Empty (positions not supported in initial version)

### 3. Account Number Generation

**Decision:** Random 8-digit numbers

- Format: 10000000-99999999 range
- Generated using random number generator
- Checked for uniqueness before creation
- Not sequential, not user-provided

### 4. Hash Value Generation

**Decision:** Full SHA256 hash (64 hex characters)

- **Method:** SHA256 of account number (as string)
- **Format:** 64 uppercase hexadecimal characters (full 256-bit hash)
- **Implementation:** `sha256(account_number.to_string())` → uppercase hex string
- Matches Schwab's real hash format exactly

### 5. Account Lifecycle Management

**a) Delete Accounts:** Hard delete with full cleanup

- `DELETE /admin/v1/accounts/{accountNumber}`
- Permanently removes account from database
- Cascades to all related tables:
  - Orders (all statuses including pending)
  - Transactions
  - Positions (when implemented)
- No soft delete, no archive
- No restrictions on deletion (can delete accounts with open orders)

**b) Reset Accounts:** Full state reset

- `POST /admin/v1/accounts/{accountNumber}/reset`
- Keeps: account_number, hash_value
- Resets: balances to initial $200,000, positions to empty
- Deletes: all orders, all transactions
- Account goes back to fresh state as if just created

**c) Manual Adjustments:** NOT SUPPORTED

- No manual balance adjustments
- No manual position additions
- To change account state, use trading operations or reset/recreate account

### 6. Test Fixtures and Data

**Decision:** Manual creation only via REST API

- No seed scripts
- No JSON fixture files
- No automatic account creation on startup
- Clean database on each test run
- Create accounts manually via curl/HTTP as needed for each test scenario

---

## Admin Endpoints Specification

```
POST   /admin/accounts              # Create new CASH account with $200k
DELETE /admin/accounts/{accountNumber}  # Hard delete account + all related data
POST   /admin/accounts/{accountNumber}/reset  # Reset to initial state
```

**Not Implemented:**

- `PUT /admin/v1/accounts/{accountNumber}/balance` - Manual adjustments not supported
- `POST /admin/v1/accounts/{accountNumber}/positions` - Positions not supported yet
- `PATCH /admin/v1/accounts/{accountNumber}` - No account updates

---

## Implementation Phases

### Phase 0.1: Account Creation ⏳ IN PROGRESS

**Goal:** Create new CASH accounts with random account numbers and SHA256 hash values

**Tasks:**

1. ✅ Create admin handler stub (`handlers/admin.rs`)
2. ✅ Create admin router and mount at `/admin/v1`
3. ✅ Build account data structures (CASH account with balances)
4. ⏸️ Implement account number generation (random 8-digit)
5. ⏸️ Implement hash value generation (SHA256)
6. ⏸️ Add service method: `account_service.create_account()`
7. ⏸️ Connect handler to service layer
8. ⏸️ Test account creation via curl
9. ⏸️ Verify account appears in `GET /accounts` endpoint

**Acceptance Criteria:**

- Can create CASH account with `POST /admin/v1/accounts`
- Account gets random 8-digit account number
- Hash value is SHA256 of account number (first 16 hex chars)
- Account has $200,000 initial balance
- Account appears in regular Schwab API endpoints
- Returns account_number and hash_value in response

---

### Phase 0.2: Account Deletion

**Goal:** Hard delete accounts and all related data

**Tasks:**

1. Add repository method: `account_repo.delete(account_number)`
2. Add repository methods to delete related data:
   - `order_repo.delete_by_account(account_number)`
   - `transaction_repo.delete_by_account(account_number)`
3. Add service method: `account_service.delete_account(account_number)`
4. Implement handler: `DELETE /admin/v1/accounts/{accountNumber}`
5. Test deletion:
   - Create account
   - Place orders
   - Delete account
   - Verify account, orders, transactions all gone

**Acceptance Criteria:**

- `DELETE /admin/v1/accounts/{accountNumber}` returns 204 No Content
- Account removed from accounts table
- All orders for account removed from orders table
- All transactions for account removed from transactions table
- Returns 404 if account doesn't exist

---

### Phase 0.3: Account Reset

**Goal:** Reset account to initial state while preserving account_number and hash

**Tasks:**

1. Add repository method: `account_repo.reset(account_number)`
2. Update account_data to initial state (balances, empty positions)
3. Delete all orders: `order_repo.delete_by_account(account_number)`
4. Delete all transactions: `transaction_repo.delete_by_account(account_number)`
5. Add service method: `account_service.reset_account(account_number)`
6. Implement handler: `POST /admin/v1/accounts/{accountNumber}/reset`
7. Test reset:
   - Create account
   - Place orders, execute trades
   - Reset account
   - Verify balance back to $200k, orders/transactions gone

**Acceptance Criteria:**

- `POST /admin/v1/accounts/{accountNumber}/reset` returns 200 OK
- Account balances reset to $200,000
- All orders deleted
- All transactions deleted
- Account number and hash unchanged
- Returns 404 if account doesn't exist

---

### Phase 0.4: Testing & Documentation

**Goal:** Validate all admin operations work correctly

**Tasks:**

1. Write curl examples for all admin endpoints
2. Test account lifecycle:
   - Create → Trade → Delete
   - Create → Trade → Reset → Trade again
3. Test edge cases:
   - Delete non-existent account (expect 404)
   - Reset non-existent account (expect 404)
   - Create account, verify in GET /accounts
4. Update TRADING_GUIDE.md with admin endpoint examples
5. Document account management workflow in README

**Acceptance Criteria:**

- All admin endpoints working as specified
- Can create, use, reset, and delete accounts via HTTP
- Documentation includes curl examples
- Ready to move on to Phase 1 (trading operations)

---

## Current Status

**Completed:**

- ✅ Design decisions documented
- ✅ Admin router infrastructure created
- ✅ Basic handler stubs in place
- ✅ CASH account structure helpers written

**In Progress:**

- ⏸️ Phase 0.1: Account Creation (need to wire up service layer)

**Next Steps:**

1. Implement random account number generation
2. Implement SHA256 hash generation
3. Wire handler → service → repository
4. Test account creation end-to-end
