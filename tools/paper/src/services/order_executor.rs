/// Order execution engine for paper trading.
/// Simulates order fills based on market data and order types.
use crate::db::repositories::{AccountRepository, OrderRepository, TransactionRepository};
use crate::services::market_data::MarketDataService;
use schwab_api::types::trader::{Instruction, Order, OrderType, Status};
use std::sync::Arc;
use tokio::time::Duration;

#[derive(Debug)]
pub enum OrderExecutorError {
    Database(String),
    MarketData(String),
    InvalidOrder(String),
}

impl std::fmt::Display for OrderExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderExecutorError::Database(msg) => write!(f, "Database error: {}", msg),
            OrderExecutorError::MarketData(msg) => write!(f, "Market data error: {}", msg),
            OrderExecutorError::InvalidOrder(msg) => write!(f, "Invalid order: {}", msg),
        }
    }
}

impl std::error::Error for OrderExecutorError {}

/// Order execution engine that processes pending orders in the background.
pub struct OrderExecutor {
    order_repo: Arc<OrderRepository>,
    account_repo: Arc<AccountRepository>,
    transaction_repo: Arc<TransactionRepository>,
    market_data: Arc<MarketDataService>,
}

impl OrderExecutor {
    /// Create a new order executor.
    pub fn new(
        order_repo: Arc<OrderRepository>,
        account_repo: Arc<AccountRepository>,
        transaction_repo: Arc<TransactionRepository>,
        market_data: Arc<MarketDataService>,
    ) -> Self {
        Self {
            order_repo,
            account_repo,
            transaction_repo,
            market_data,
        }
    }

    /// Run the execution loop as a background task.
    /// Checks for pending orders every second and attempts to fill them.
    pub async fn run_execution_loop(self: Arc<Self>) {
        let mut interval = tokio::time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;

            if let Err(e) = self.process_pending_orders().await {
                eprintln!("Order execution error: {}", e);
            }
        }
    }

    /// Process all pending orders and attempt to fill them.
    async fn process_pending_orders(&self) -> Result<(), OrderExecutorError> {
        // Get all WORKING orders (simple query without params for now)
        // TODO: Once we have a method to get all working orders, use it here
        // For now, this is a placeholder implementation

        Ok(())
    }

    /// Check if a MARKET order should be filled and execute it.
    pub async fn check_and_fill_market_order(
        &self,
        order: &mut Order,
    ) -> Result<bool, OrderExecutorError> {
        // MARKET orders fill immediately at current price
        if order.order_type != Some(OrderType::Market) {
            return Ok(false);
        }

        // Extract the symbol from the first order leg
        let symbol = self.extract_symbol(order)?;

        // Get current market price
        let fill_price = self
            .market_data
            .get_current_price(&symbol)
            .map_err(|e| OrderExecutorError::MarketData(e.to_string()))?;

        // Fill the order
        self.fill_order(order, fill_price).await?;

        Ok(true)
    }

    /// Check if a LIMIT order should be filled based on current market price.
    pub async fn check_and_fill_limit_order(
        &self,
        order: &mut Order,
    ) -> Result<bool, OrderExecutorError> {
        if order.order_type != Some(OrderType::Limit) {
            return Ok(false);
        }

        let limit_price = order
            .price
            .ok_or_else(|| OrderExecutorError::InvalidOrder("LIMIT order missing price".into()))?;

        // Extract the symbol and instruction
        let symbol = self.extract_symbol(order)?;
        let instruction = self.extract_instruction(order)?;

        // Get current market price
        let market_price = self
            .market_data
            .get_current_price(&symbol)
            .map_err(|e| OrderExecutorError::MarketData(e.to_string()))?;

        // Check if order should fill
        let should_fill = match instruction {
            // BUY fills when market price is at or below limit price
            Instruction::Buy | Instruction::BuyToOpen | Instruction::BuyToClose => {
                market_price <= limit_price
            }
            // SELL fills when market price is at or above limit price
            Instruction::Sell | Instruction::SellToClose | Instruction::SellToOpen => {
                market_price >= limit_price
            }
            _ => false,
        };

        if should_fill {
            self.fill_order(order, market_price).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Check if a STOP order should be activated.
    pub async fn check_and_fill_stop_order(
        &self,
        order: &mut Order,
    ) -> Result<bool, OrderExecutorError> {
        if order.order_type != Some(OrderType::Stop) {
            return Ok(false);
        }

        let stop_price = order.stop_price.ok_or_else(|| {
            OrderExecutorError::InvalidOrder("STOP order missing stop_price".into())
        })?;

        // Extract the symbol and instruction
        let symbol = self.extract_symbol(order)?;
        let instruction = self.extract_instruction(order)?;

        // Get current market price
        let market_price = self
            .market_data
            .get_current_price(&symbol)
            .map_err(|e| OrderExecutorError::MarketData(e.to_string()))?;

        // Check if stop should activate
        let should_activate = match instruction {
            // SELL STOP activates when market price falls to or below stop price
            Instruction::Sell | Instruction::SellToClose => market_price <= stop_price,
            // BUY STOP activates when market price rises to or above stop price
            Instruction::Buy | Instruction::BuyToOpen => market_price >= stop_price,
            _ => false,
        };

        if should_activate {
            // STOP orders become MARKET orders when activated, so fill at market price
            self.fill_order(order, market_price).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Fill an order at the given price and update account positions/balances.
    async fn fill_order(
        &self,
        order: &mut Order,
        fill_price: f64,
    ) -> Result<(), OrderExecutorError> {
        // Update order status
        order.status = Some(Status::Filled);
        order.filled_quantity = order.quantity;
        order.remaining_quantity = Some(0.0);
        order.close_time = Some(chrono::Utc::now().to_rfc3339());

        // Get account number
        let account_number = order
            .account_number
            .ok_or_else(|| OrderExecutorError::InvalidOrder("Order missing account_number".into()))?
            .to_string();

        // Update order in database
        let order_id = order
            .order_id
            .ok_or_else(|| OrderExecutorError::InvalidOrder("Order missing order_id".into()))?;

        self.order_repo
            .update(order_id, order)
            .await
            .map_err(|e| OrderExecutorError::Database(e.to_string()))?;

        // TODO: Get account and update positions/balances
        // This requires more complex handling of SecuritiesAccount enum (Margin vs Cash)
        // and understanding the various balance types (MarginBalance, CashBalance, etc.)
        // For now, we'll defer full implementation until account management is better understood

        // TODO: Generate transaction record
        // This requires understanding the Transaction type structure better

        Ok(())
    }

    /// Extract the symbol from an order's first order leg.
    fn extract_symbol(&self, order: &Order) -> Result<String, OrderExecutorError> {
        use schwab_api::types::trader::AccountsInstrument;

        let instrument = order
            .order_leg_collection
            .as_ref()
            .and_then(|legs| legs.first())
            .and_then(|leg| leg.instrument.as_ref())
            .ok_or_else(|| OrderExecutorError::InvalidOrder("Order missing instrument".into()))?;

        // Extract symbol based on instrument type
        let symbol = match instrument.as_ref() {
            AccountsInstrument::Equity(equity) => equity.symbol.clone(),
            AccountsInstrument::Option(option) => option.symbol.clone(),
            AccountsInstrument::MutualFund(fund) => fund.symbol.clone(),
            _ => None,
        };

        symbol.ok_or_else(|| OrderExecutorError::InvalidOrder("Instrument missing symbol".into()))
    }

    /// Extract the instruction from an order's first order leg.
    fn extract_instruction(&self, order: &Order) -> Result<Instruction, OrderExecutorError> {
        order
            .order_leg_collection
            .as_ref()
            .and_then(|legs| legs.first())
            .and_then(|leg| leg.instruction)
            .ok_or_else(|| OrderExecutorError::InvalidOrder("Order missing instruction".into()))
    }
}
