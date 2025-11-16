# Paper Trading Guide for Software Developers

This guide explains trading concepts, account types, order types, and market mechanics using the Schwab API as implemented in this paper trader.

## Table of Contents

1. [Account Types](#account-types)
2. [Asset Types](#asset-types)
3. [Order Types](#order-types)
4. [Order Duration](#order-duration)
5. [Trading Sessions](#trading-sessions)
6. [Order Instructions](#order-instructions)
7. [Common Trading Scenarios](#common-trading-scenarios)

---

## Account Types

### CASH Account

**What it is:** A basic brokerage account where you can only trade with money you actually have deposited.

**Key characteristics:**

- ✅ Simple and safe - can't go into debt
- ✅ No borrowing or leverage
- ❌ Must wait for trades to "settle" (T+2 = 2 business days)
- ❌ Subject to "free-riding" violations if you buy and sell before cash settles
- **Buying Power:** Equal to your cash balance

**Example:**

```
You have: $10,000 cash
You can buy: $10,000 worth of stocks
```

**Balance fields (CashBalance):**

- `cash_available_for_trading` - Cash you can use right now
- `total_cash` - Total cash including unsettled amounts
- `cash_available_for_withdrawal` - Cash you can withdraw

### MARGIN Account

**What it is:** An account where the broker lets you borrow money to trade (up to 50% of purchase price for stocks).

**Key characteristics:**

- ✅ Can borrow money from broker (margin)
- ✅ Higher buying power (2x your cash for stocks)
- ✅ Can day trade more freely
- ✅ Can sell stocks short
- ❌ More complex - can lose more than you invested
- ❌ Pay interest on borrowed money
- **Buying Power:** Typically 2x your account value for stocks (Regulation T)

**Example:**

```
You have: $10,000 cash
You can buy: $20,000 worth of stocks (borrowing $10,000)
```

**Balance fields (MarginBalance):**

- `available_funds` - Money available to trade with
- `buying_power` - Total purchasing power (usually 2x available funds)
- `equity` - Your actual ownership (total value - borrowed amount)
- `maintenance_excess` - Buffer above minimum required equity

**Pattern Day Trader (PDT) Rule:**
If you make 4+ day trades (buy and sell same stock same day) in 5 business days, you must maintain $25,000 minimum balance.

---

## Asset Types

### EQUITY (Stocks)

Common stock shares of publicly traded companies.

**Examples:** AAPL (Apple), TSLA (Tesla), MSFT (Microsoft)

**Fields:**

- `symbol` - Ticker symbol (e.g., "AAPL")
- `cusip` - 9-digit security identifier
- `asset_type` - "EQUITY"

### OPTION (Options Contracts)

Contracts giving you the right (but not obligation) to buy or sell a stock at a specific price by a specific date.

**Two types:**

- **CALL option:** Right to BUY stock at strike price
- **PUT option:** Right to SELL stock at strike price

**Example:**

```
AAPL Jan 20, 2024 $150 CALL
- Underlying: AAPL stock
- Expiration: January 20, 2024
- Strike: $150
- Type: CALL (right to buy)
```

**Key concepts:**

- **In the money (ITM):** Option has intrinsic value
  - Call: stock price > strike price
  - Put: stock price < strike price
- **Out of the money (OTM):** Option has no intrinsic value
- **At the money (ATM):** Stock price ≈ strike price

### Other Asset Types

- **MUTUAL_FUND** - Professionally managed investment fund
- **FIXED_INCOME** - Bonds and other debt instruments
- **CASH_EQUIVALENT** - Money market funds, savings
- **INDEX** - Market index (S&P 500, Nasdaq, etc.)
- **CURRENCY** - Foreign exchange
- **COLLECTIVE_INVESTMENT** - ETFs, REITs, etc.

---

## Order Types

### MARKET Order

**What:** Buy/sell immediately at current market price

**Use when:** You want to execute quickly and price isn't critical

**Example:**

```json
{
  "orderType": "MARKET",
  "quantity": 100,
  "instruction": "BUY"
}
```

**Behavior:**

- Executes immediately (in paper trader)
- Gets filled at current market price
- No price guarantee - could execute at any price

### LIMIT Order

**What:** Buy/sell only at a specific price or better

**Use when:** You want price control

**Example:**

```json
{
  "orderType": "LIMIT",
  "price": 150.0,
  "quantity": 100,
  "instruction": "BUY"
}
```

**Behavior:**

- **BUY limit:** Only execute at limit price or LOWER
- **SELL limit:** Only execute at limit price or HIGHER
- May not execute if price never reaches your limit
- In paper trader: Fills when market price reaches limit

### STOP Order (Stop Loss)

**What:** Becomes a MARKET order when price reaches stop price

**Use when:** You want to limit losses or protect profits

**Example:**

```json
{
  "orderType": "STOP",
  "stopPrice": 140.0,
  "quantity": 100,
  "instruction": "SELL"
}
```

**Behavior:**

- Sits waiting until stock hits stop price
- Then converts to MARKET order
- **Sell stop:** Triggers when price falls to/below stop price (prevent further losses)
- **Buy stop:** Triggers when price rises to/above stop price (enter on breakout)

### STOP_LIMIT Order

**What:** Becomes a LIMIT order when price reaches stop price

**Use when:** You want both trigger control AND price control

**Example:**

```json
{
  "orderType": "STOP_LIMIT",
  "stopPrice": 140.0,
  "price": 139.0,
  "quantity": 100,
  "instruction": "SELL"
}
```

**Behavior:**

- Triggers at stop price
- Then tries to fill at limit price
- More control but may not execute if price moves too fast

### Other Order Types

- **TRAILING_STOP** - Stop price trails the market by fixed amount or percentage
- **TRAILING_STOP_LIMIT** - Trailing stop that becomes limit order
- **MARKET_ON_CLOSE** - Execute at closing price
- **LIMIT_ON_CLOSE** - Execute at close if price is at/better than limit
- **NET_DEBIT/CREDIT/ZERO** - Multi-leg options strategies

---

## Order Duration

How long an order stays active:

### DAY

Order valid until end of trading day. Cancels automatically if not filled.

### GOOD_TILL_CANCEL (GTC)

Order stays active until you cancel it or it executes (typically up to 90 days).

### FILL_OR_KILL (FOK)

Execute entire order immediately or cancel it. All-or-nothing.

### IMMEDIATE_OR_CANCEL (IOC)

Execute whatever you can immediately, cancel the rest. Partial fills ok.

### END_OF_WEEK / END_OF_MONTH / NEXT_END_OF_MONTH

Order active until specified period ends.

---

## Trading Sessions

When the order can execute:

### NORMAL

Regular trading hours: 9:30 AM - 4:00 PM ET

### AM

Pre-market trading: 7:00 AM - 9:30 AM ET

### PM

After-hours trading: 4:00 PM - 8:00 PM ET

### SEAMLESS

Can execute in pre-market, regular hours, and after-hours

---

## Order Instructions

What action to take:

### BUY

Purchase shares/contracts. Increases your position.

### SELL

Sell shares/contracts you own. Decreases or closes position.

### SELL_SHORT

Borrow and sell shares you don't own (hoping price drops).

- Only in margin accounts
- Must "buy to cover" later to return borrowed shares

### BUY_TO_COVER

Buy shares to return borrowed shares (close short position).

### SELL_TO_OPEN

Sell options contracts to open a new position (collect premium).

### BUY_TO_OPEN

Buy options contracts to open a new position (pay premium).

### SELL_TO_CLOSE

Sell options contracts to close an existing position.

### BUY_TO_CLOSE

Buy options contracts to close an existing short position.

---

## Common Trading Scenarios

### Scenario 1: Basic Stock Purchase (Long Position)

**Goal:** Buy 100 shares of AAPL

**Account Type:** CASH or MARGIN

**Order:**

```json
{
  "session": "NORMAL",
  "duration": "DAY",
  "orderType": "MARKET",
  "quantity": 100,
  "orderStrategyType": "SINGLE",
  "orderLegCollection": [
    {
      "instruction": "BUY",
      "quantity": 100,
      "instrument": {
        "symbol": "AAPL",
        "assetType": "EQUITY"
      }
    }
  ]
}
```

**What happens:**

1. Order submitted to paper trader
2. Immediately filled at current market price (e.g., $175.50)
3. Cost: 100 × $175.50 = $17,550
4. Your position: +100 shares AAPL
5. Your cash decreases by $17,550

### Scenario 2: Sell with Limit Price

**Goal:** Sell your 100 AAPL shares, but only if you can get $180 or better

**Order:**

```json
{
  "session": "NORMAL",
  "duration": "GTC",
  "orderType": "LIMIT",
  "price": 180.0,
  "quantity": 100,
  "orderStrategyType": "SINGLE",
  "orderLegCollection": [
    {
      "instruction": "SELL",
      "quantity": 100,
      "instrument": {
        "symbol": "AAPL",
        "assetType": "EQUITY"
      }
    }
  ]
}
```

**What happens:**

1. Order placed with "QUEUED" status
2. Order executor checks periodically if AAPL price ≥ $180
3. When price reaches $180, order fills
4. Your position: 0 shares AAPL (position closed)
5. Your cash increases by $18,000

### Scenario 3: Stop Loss Protection

**Goal:** Protect your AAPL position from dropping below $170

**Order:**

```json
{
  "session": "NORMAL",
  "duration": "GTC",
  "orderType": "STOP",
  "stopPrice": 170.0,
  "quantity": 100,
  "orderStrategyType": "SINGLE",
  "orderLegCollection": [
    {
      "instruction": "SELL",
      "quantity": 100,
      "instrument": {
        "symbol": "AAPL",
        "assetType": "EQUITY"
      }
    }
  ]
}
```

**What happens:**

1. Order sits with "QUEUED" status
2. If AAPL drops to $170, stop triggers
3. Becomes MARKET order and executes immediately
4. Sells at current price (could be $170, $169, or lower if price moving fast)
5. Limits your loss

### Scenario 4: Short Selling (Margin Account Only)

**Goal:** Bet that TSLA will drop from $250 to $200

**Order:**

```json
{
  "session": "NORMAL",
  "duration": "DAY",
  "orderType": "MARKET",
  "quantity": 50,
  "orderStrategyType": "SINGLE",
  "orderLegCollection": [
    {
      "instruction": "SELL_SHORT",
      "quantity": 50,
      "instrument": {
        "symbol": "TSLA",
        "assetType": "EQUITY"
      }
    }
  ]
}
```

**What happens:**

1. Borrow 50 TSLA shares and sell them at $250
2. Your position: -50 shares TSLA (short position)
3. Your cash increases by $12,500
4. Later, if TSLA drops to $200, you buy to cover:

```json
{
  "orderLegCollection": [
    {
      "instruction": "BUY_TO_COVER",
      "quantity": 50,
      "instrument": { "symbol": "TSLA", "assetType": "EQUITY" }
    }
  ]
}
```

5. Buy 50 shares at $200 = $10,000
6. Profit: $12,500 - $10,000 = $2,500

**Risk:** If TSLA goes UP to $300, you lose money:

- Sold at: $250 × 50 = $12,500
- Buy back at: $300 × 50 = $15,000
- Loss: $2,500

---

## Paper Trader Implementation Notes

### Current Features

✅ CASH and MARGIN account types
✅ EQUITY asset type with positions
✅ MARKET, LIMIT, STOP order types
✅ Order execution simulation
✅ Balance tracking
✅ Position management

### Limitations

⚠️ No real-time market data (uses simulated prices)
⚠️ Instant fills for market orders (no slippage)
⚠️ No partial fills
⚠️ No options trading yet
⚠️ No advanced order types (trailing stop, etc.)
⚠️ No margin interest calculation
⚠️ No day trading restrictions enforced

### Coming Soon

🔄 Order executor background task
🔄 More sophisticated price simulation
🔄 Transaction history generation
🔄 Options support
🔄 Multi-leg strategies

---

## Quick Reference: Order Examples

### Market Order (Buy Now)

```bash
curl -X POST http://localhost:9000/trader/v1/accounts/12345678/orders \
  -H "Content-Type: application/json" \
  -d '{
    "session": "NORMAL",
    "duration": "DAY",
    "orderType": "MARKET",
    "orderStrategyType": "SINGLE",
    "orderLegCollection": [{
      "instruction": "BUY",
      "quantity": 100,
      "instrument": {"symbol": "AAPL", "assetType": "EQUITY"}
    }]
  }'
```

### Limit Order (Buy at $150 or less)

```bash
curl -X POST http://localhost:9000/trader/v1/accounts/12345678/orders \
  -H "Content-Type: application/json" \
  -d '{
    "session": "NORMAL",
    "duration": "GTC",
    "orderType": "LIMIT",
    "price": 150.00,
    "orderStrategyType": "SINGLE",
    "orderLegCollection": [{
      "instruction": "BUY",
      "quantity": 100,
      "instrument": {"symbol": "AAPL", "assetType": "EQUITY"}
    }]
  }'
```

### Stop Loss (Sell if drops to $140)

```bash
curl -X POST http://localhost:9000/trader/v1/accounts/12345678/orders \
  -H "Content-Type: application/json" \
  -d '{
    "session": "NORMAL",
    "duration": "GTC",
    "orderType": "STOP",
    "stopPrice": 140.00,
    "orderStrategyType": "SINGLE",
    "orderLegCollection": [{
      "instruction": "SELL",
      "quantity": 100,
      "instrument": {"symbol": "AAPL", "assetType": "EQUITY"}
    }]
  }'
```

---

## Glossary

**Ask Price:** Price sellers are asking for a security  
**Bid Price:** Price buyers are willing to pay  
**Spread:** Difference between bid and ask  
**Slippage:** Difference between expected and actual execution price  
**Fill:** Execution of an order  
**Position:** Number of shares/contracts you own (positive = long, negative = short)  
**Equity:** Your ownership value (total value - debt)  
**Buying Power:** Maximum amount you can purchase  
**Maintenance Margin:** Minimum equity required to keep positions open  
**Mark:** Current market price  
**Unrealized P/L:** Profit/loss on open positions (not yet closed)  
**Realized P/L:** Profit/loss on closed positions

---

## Further Learning

- **Options:** https://www.investopedia.com/options-basics-tutorial-4583012
- **Margin Trading:** https://www.investopedia.com/terms/m/margin.asp
- **Short Selling:** https://www.investopedia.com/terms/s/shortselling.asp
- **Order Types:** https://www.investopedia.com/investing/basics-trading-stock-know-your-orders/
- **Pattern Day Trader:** https://www.investopedia.com/terms/p/patterndaytrader.asp
