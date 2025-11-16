### Phase 0: Account Management

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
