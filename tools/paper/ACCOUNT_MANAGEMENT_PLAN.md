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

- `DELETE /admin/v1/accounts/{hashValue}`
- Uses encrypted account hash (consistent with `/trader/v1` endpoints)
- Permanently removes account from database
- Database CASCADE DELETE handles related data automatically:
  - Orders (all statuses including pending)
  - Transactions
  - Positions (when implemented)
- Foreign keys configured with `ON DELETE CASCADE`
- No soft delete, no archive
- No restrictions on deletion (can delete accounts with open orders)

**b) Reset Accounts:** Full state reset

- `POST /admin/v1/accounts/{hashValue}/reset`
- Uses encrypted account hash (consistent with `/trader/v1` endpoints)
- Keeps: account_number, hash_value
- Resets: balances to initial $200,000, positions to empty
- Deletes: all orders, all transactions (CASCADE DELETE handles this automatically)
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
POST   /admin/v1/accounts                    # Create new CASH account with $200k
DELETE /admin/v1/accounts/{hashValue}        # Hard delete account + all related data
POST   /admin/v1/accounts/{hashValue}/reset  # Reset to initial state
```

**Implementation Note:** All admin endpoints use `{hashValue}` (encrypted account ID) instead of plain `{accountNumber}` for consistency with `/trader/v1` endpoints.

**Not Implemented:**

- `PUT /admin/v1/accounts/{hashValue}/balance` - Manual adjustments not supported
- `POST /admin/v1/accounts/{hashValue}/positions` - Positions not supported yet
- `PATCH /admin/v1/accounts/{hashValue}` - No account updates

---

## Implementation Phases

### Phase 0.1: Account Creation ✅ COMPLETE

**Goal:** Create new CASH accounts with random account numbers and SHA256 hash values

**Tasks:**

1. ✅ Create admin handler stub (`handlers/admin.rs`)
2. ✅ Create admin router and mount at `/admin/v1`
3. ✅ Build account data structures (CASH account with balances)
4. ✅ Implement account number generation (random 8-digit)
5. ✅ Implement hash value generation (SHA256)
6. ✅ Add service method: `account_service.create_account()`
7. ✅ Connect handler to service layer
8. ✅ Test account creation via curl
9. ✅ Verify account appears in `GET /accounts` endpoint

**Acceptance Criteria:** ✅ ALL MET

- ✅ Can create CASH account with `POST /admin/v1/accounts`
- ✅ Account gets random 8-digit account number (range: 10000000-99999999)
- ✅ Hash value is SHA256 of account number (64 uppercase hex characters)
- ✅ Account has $200,000 initial balance (fixed, ignores request body)
- ✅ Account appears in regular Schwab API endpoints
- ✅ Returns account_number and hash_value in response

**Implementation Notes:**

- Random number generation uses `rand::thread_rng()` with uniqueness checking
- SHA256 hash computed using `sha2` crate, formatted as 64 uppercase hex chars
- RNG scoped to prevent Send trait issues across await boundaries
- Enabled `axum` macros feature for `#[axum::debug_handler]` attribute
- Request body accepted but ignored (fixed $200k CASH accounts only)

---

### Phase 0.2: Account Deletion ✅ COMPLETE

**Goal:** Hard delete accounts and all related data

**Tasks:**

1. ✅ Add repository method: `account_repo.delete(hash_value)`
2. ✅ Add database CASCADE DELETE constraints:
   - `orders.account_number` REFERENCES `accounts(account_number)` ON DELETE CASCADE
   - `transactions.account_number` REFERENCES `accounts(account_number)` ON DELETE CASCADE
3. ✅ Add service method: `account_service.delete_account(hash_value)`
4. ✅ Implement handler: `DELETE /admin/v1/accounts/{hashValue}`
5. ✅ Test deletion:
   - Create account
   - Delete account
   - Verify account and related data all gone

**Acceptance Criteria:** ✅ ALL MET

- ✅ `DELETE /admin/v1/accounts/{hashValue}` returns 204 No Content
- ✅ Account removed from accounts table
- ✅ All orders for account removed automatically (database CASCADE DELETE)
- ✅ All transactions for account removed automatically (database CASCADE DELETE)
- ✅ Returns 404 if account doesn't exist (via service layer validation)
- ✅ Uses hash_value instead of account_number (consistent with /trader/v1 endpoints)

**Implementation Notes:**

- Database foreign keys configured with `ON DELETE CASCADE` in migration
- Service layer simplified - just calls `repository.delete(hash_value)`
- Database handles cascade deletion automatically
- AccountService requires only AccountRepository (no OrderRepository/TransactionRepository needed)
- Handler properly maps AccountServiceError to HTTP status codes via error_mapping
- Refactored to use hash_value instead of account_number for API consistency

---

### Phase 0.3: Account Reset ✅ COMPLETE

**Goal:** Reset account to initial state while preserving account_number and hash

**Tasks:**

1. ✅ Add repository method: `account_repo.reset(hash_value, initial_account_data)`
2. ✅ Update account_data to initial state (balances back to $200,000, empty positions)
3. ✅ Add service method: `account_service.reset_account(hash_value)`
4. ✅ Implement handler: `POST /admin/v1/accounts/{hashValue}/reset`
5. ✅ Test reset:
   - Create account
   - Reset account
   - Verify balance back to $200k, orders/transactions gone (via CASCADE DELETE)

**Acceptance Criteria:** ✅ ALL MET

- ✅ `POST /admin/v1/accounts/{hashValue}/reset` returns 200 OK
- ✅ Account balances reset to $200,000
- ✅ All orders deleted (automatically via CASCADE DELETE when account_data updated)
- ✅ All transactions deleted (automatically via CASCADE DELETE when account_data updated)
- ✅ Account number and hash unchanged
- ✅ Returns 404 if account doesn't exist
- ✅ Uses hash_value instead of account_number (consistent with /trader/v1 endpoints)

**Implementation Notes:**

- Repository method updates account_data in place via UPDATE query
- Service fetches current account to extract account_number for creating fresh data
- Helper function `create_initial_cash_account()` generates fresh $200k CASH account
- Database CASCADE DELETE handles related records automatically
- Uses hash_value in API path for consistency with /trader/v1 endpoints
- Returns 200 OK (not 204) as specified in acceptance criteria

---

### Phase 0.4: Testing & Documentation ⏸️ NOT STARTED

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

- ✅ Design decisions documented (all 6 decisions finalized)
- ✅ Admin router infrastructure created
- ✅ Basic handler stubs in place
- ✅ CASH account structure helpers written
- ✅ **Phase 0.1: Account Creation** - Fully functional with random 8-digit account numbers and SHA256 hashes
- ✅ **Phase 0.2: Account Deletion** - CASCADE DELETE implemented in database, uses hash_value for consistency
- ✅ **Phase 0.3: Account Reset** - Update account_data in place, CASCADE DELETE handles related records

**In Progress:**

- None

**Next Steps:**

1. **Phase 0.4:** Write comprehensive tests and documentation
2. Move to Phase 1: Trading operations (orders, positions, executions)
