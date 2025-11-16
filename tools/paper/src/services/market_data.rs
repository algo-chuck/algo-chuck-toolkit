use rand::Rng;
/// Mock market data service for paper trading order execution.
/// Provides simulated real-time prices for common symbols.
use std::collections::HashMap;

#[derive(Debug)]
pub enum MarketDataError {
    SymbolNotFound(String),
}

impl std::fmt::Display for MarketDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarketDataError::SymbolNotFound(symbol) => {
                write!(f, "Symbol not found: {}", symbol)
            }
        }
    }
}

impl std::error::Error for MarketDataError {}

/// Mock market data service with simulated prices.
#[derive(Clone)]
pub struct MarketDataService {
    /// Base prices for common symbols (used as starting point for simulation)
    base_prices: HashMap<String, f64>,
}

impl MarketDataService {
    /// Create a new market data service with default base prices.
    pub fn new() -> Self {
        let mut base_prices = HashMap::new();

        // Common stocks with realistic base prices
        base_prices.insert("AAPL".to_string(), 175.0);
        base_prices.insert("GOOGL".to_string(), 140.0);
        base_prices.insert("MSFT".to_string(), 380.0);
        base_prices.insert("AMZN".to_string(), 155.0);
        base_prices.insert("TSLA".to_string(), 245.0);
        base_prices.insert("NVDA".to_string(), 485.0);
        base_prices.insert("META".to_string(), 450.0);
        base_prices.insert("JPM".to_string(), 160.0);
        base_prices.insert("V".to_string(), 270.0);
        base_prices.insert("WMT".to_string(), 68.0);
        base_prices.insert("SPY".to_string(), 470.0); // S&P 500 ETF
        base_prices.insert("QQQ".to_string(), 395.0); // Nasdaq ETF

        Self { base_prices }
    }

    /// Get the current simulated price for a symbol.
    ///
    /// Prices are simulated with small random variations (±1%) from base price
    /// to simulate market movement. In a real implementation, this would
    /// connect to a real-time data feed.
    ///
    /// # Arguments
    /// * `symbol` - The stock symbol (e.g., "AAPL", "GOOGL")
    ///
    /// # Returns
    /// * `Ok(f64)` - The current price
    /// * `Err(MarketDataError::SymbolNotFound)` - If symbol is not in the mock data
    pub fn get_current_price(&self, symbol: &str) -> Result<f64, MarketDataError> {
        let base_price = self
            .base_prices
            .get(symbol)
            .ok_or_else(|| MarketDataError::SymbolNotFound(symbol.to_string()))?;

        // Add small random variation (±1%) to simulate market movement
        let mut rng = rand::thread_rng();
        let variation = rng.gen_range(-0.01..=0.01);
        let current_price = base_price * (1.0 + variation);

        // Round to 2 decimal places
        Ok((current_price * 100.0).round() / 100.0)
    }

    /// Add a custom symbol with a base price.
    /// Useful for testing with specific symbols.
    pub fn add_symbol(&mut self, symbol: String, base_price: f64) {
        self.base_prices.insert(symbol, base_price);
    }

    /// Check if a symbol is available in the mock data.
    pub fn has_symbol(&self, symbol: &str) -> bool {
        self.base_prices.contains_key(symbol)
    }
}

impl Default for MarketDataService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_price_known_symbol() {
        let service = MarketDataService::new();
        let price = service.get_current_price("AAPL").unwrap();

        // Price should be close to base price (within 1%)
        assert!(price > 173.0 && price < 177.0);
    }

    #[test]
    fn test_get_current_price_unknown_symbol() {
        let service = MarketDataService::new();
        let result = service.get_current_price("UNKNOWN");

        assert!(matches!(result, Err(MarketDataError::SymbolNotFound(_))));
    }

    #[test]
    fn test_add_custom_symbol() {
        let mut service = MarketDataService::new();
        service.add_symbol("TEST".to_string(), 100.0);

        let price = service.get_current_price("TEST").unwrap();
        assert!(price > 99.0 && price < 101.0);
    }

    #[test]
    fn test_has_symbol() {
        let service = MarketDataService::new();

        assert!(service.has_symbol("AAPL"));
        assert!(!service.has_symbol("UNKNOWN"));
    }
}
