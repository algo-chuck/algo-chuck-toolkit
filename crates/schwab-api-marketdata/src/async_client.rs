//! Asynchronous client implementation for Schwab Market Data API.
//!
//! This module provides an async client for interacting with the Schwab Market Data API,
//! supporting operations like quotes, option chains, price history, and market hours.

use schwab_api_core::{ApiClient, AsyncHttpClient, HttpError};
use schwab_api_types::{
    CandleList, ExpirationChain, GetMovers200Response, Hours, InstrumentResponse, OptionChain,
    QuoteResponseObject,
};
use std::collections::HashMap;
use std::ops::Deref;

use crate::{MarketdataConfig, MarketdataParams};

/// Asynchronous client for Schwab Market Data API.
///
/// This client provides async methods for all Market Data API operations, including:
/// - Real-time and delayed quotes
/// - Option chains and expirations
/// - Price history and candlestick data
/// - Market movers
/// - Market hours
/// - Instrument searches
///
/// # Examples
///
/// ```ignore
/// use schwab_api_marketdata::AsyncMarketdataClient;
///
/// let http_client = reqwest::Client::new();
/// let client = AsyncMarketdataClient::new(http_client);
///
/// let quotes = client.get_quote("your_token", "AAPL", None).await?;
/// ```
pub struct AsyncMarketdataClient<C: AsyncHttpClient> {
    client: ApiClient<C, MarketdataConfig>,
}

impl<C: AsyncHttpClient> AsyncMarketdataClient<C> {
    pub fn new(client: C, access_token: impl Into<String>) -> Self {
        Self {
            client: ApiClient::new(client, access_token),
        }
    }

    /// Update the access token (e.g., after refresh)
    pub fn set_access_token(&self, new_token: impl Into<String>) {
        self.client.set_access_token(new_token);
    }

    /// Get the current access token
    pub fn get_access_token(&self) -> String {
        self.client.get_access_token()
    }
}

impl<C: AsyncHttpClient> Deref for AsyncMarketdataClient<C> {
    type Target = ApiClient<C, MarketdataConfig>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl<C> AsyncMarketdataClient<C>
where
    C: AsyncHttpClient,
    HttpError: From<C::Error>,
{
    /// Get quotes for multiple symbols
    pub async fn get_quotes(
        &self,
        symbols: &str,
        fields: Option<&str>,
        indicative: Option<bool>,
    ) -> Result<HashMap<String, QuoteResponseObject>, HttpError> {
        let params = MarketdataParams::get_quotes(symbols, fields, indicative);
        self.client.fetch(&params).await
    }

    /// Get quote for a single symbol
    pub async fn get_quote(
        &self,
        symbol: &str,
        fields: Option<&str>,
    ) -> Result<HashMap<String, QuoteResponseObject>, HttpError> {
        let params = MarketdataParams::get_quote(symbol, fields);
        self.client.fetch(&params).await
    }

    /// Get option chain for an optionable symbol
    ///
    /// **KNOWN ISSUE**: The generated `OptionChain` type from schwab-api-types has structural
    /// mismatches with the actual Schwab API response:
    ///
    /// 1. **Strike prices map to arrays**: The API returns
    ///    `HashMap<String, HashMap<String, Vec<OptionContract>>>` but the generated type expects
    ///    `HashMap<String, HashMap<String, OptionContract>>` (no Vec wrapper).
    ///
    /// 2. **Missing fields**: The API returns `assetMainType` and `assetSubType` fields that
    ///    aren't defined in the generated OptionChain struct.
    ///
    /// 3. **Nested structure issues**: Some nested fields in OptionContract or its sub-types
    ///    may not match the actual API response.
    ///
    /// **Result**: This method will trigger a deserialization warning and return an error.
    /// The warning system will show the raw JSON response for debugging, but the data cannot
    /// be used in a type-safe manner until the OptionChain type is corrected to match the
    /// actual API structure.
    ///
    /// **Workaround**: Access the raw JSON from the error/warning output, or update the
    /// schwab-api-types crate with the corrected structure.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_chain(
        &self,
        symbol: &str,
        contract_type: Option<&str>,
        strike_count: Option<i32>,
        include_underlying_quote: Option<bool>,
        strategy: Option<&str>,
        interval: Option<f64>,
        strike: Option<f64>,
        range: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
        volatility: Option<f64>,
        underlying_price: Option<f64>,
        interest_rate: Option<f64>,
        days_to_expiration: Option<i32>,
        exp_month: Option<&str>,
        option_type: Option<&str>,
    ) -> Result<OptionChain, HttpError> {
        let params = MarketdataParams::get_chain(
            symbol,
            contract_type,
            strike_count,
            include_underlying_quote,
            strategy,
            interval,
            strike,
            range,
            from_date,
            to_date,
            volatility,
            underlying_price,
            interest_rate,
            days_to_expiration,
            exp_month,
            option_type,
        );
        self.client.fetch(&params).await
    }

    /// Get option expiration chain for an optionable symbol
    pub async fn get_expiration_chain(&self, symbol: &str) -> Result<ExpirationChain, HttpError> {
        let params = MarketdataParams::get_expiration_chain(symbol);
        self.client.fetch(&params).await
    }

    /// Get price history for a symbol
    #[allow(clippy::too_many_arguments)]
    pub async fn get_price_history(
        &self,
        symbol: &str,
        period_type: Option<&str>,
        period: Option<i32>,
        frequency_type: Option<&str>,
        frequency: Option<i32>,
        start_date: Option<i64>,
        end_date: Option<i64>,
        need_extended_hours_data: Option<bool>,
        need_previous_close: Option<bool>,
    ) -> Result<CandleList, HttpError> {
        let params = MarketdataParams::get_price_history(
            symbol,
            period_type,
            period,
            frequency_type,
            frequency,
            start_date,
            end_date,
            need_extended_hours_data,
            need_previous_close,
        );
        self.client.fetch(&params).await
    }

    /// Get movers for a specific index
    pub async fn get_movers(
        &self,
        symbol: &str,
        sort: Option<&str>,
        frequency: Option<i32>,
    ) -> Result<GetMovers200Response, HttpError> {
        let params = MarketdataParams::get_movers(symbol, sort, frequency);
        self.client.fetch(&params).await
    }

    /// Get market hours for multiple markets
    pub async fn get_market_hours(
        &self,
        markets: &str,
        date: Option<&str>,
    ) -> Result<HashMap<String, HashMap<String, Hours>>, HttpError> {
        let params = MarketdataParams::get_market_hours(markets, date);
        self.client.fetch(&params).await
    }

    /// Get market hours for a single market
    pub async fn get_market_hour(
        &self,
        market: &str,
        date: Option<&str>,
    ) -> Result<HashMap<String, HashMap<String, Hours>>, HttpError> {
        let params = MarketdataParams::get_market_hour(market, date);
        self.client.fetch(&params).await
    }

    /// Get instruments by symbol and projection
    pub async fn get_instruments(
        &self,
        symbol: &str,
        projection: &str,
    ) -> Result<HashMap<String, Vec<InstrumentResponse>>, HttpError> {
        let params = MarketdataParams::get_instruments(symbol, projection);
        self.client.fetch(&params).await
    }

    /// Get instrument by CUSIP
    pub async fn get_instruments_by_cusip(
        &self,
        cusip: &str,
    ) -> Result<HashMap<String, Vec<InstrumentResponse>>, HttpError> {
        let params = MarketdataParams::get_instruments_by_cusip(cusip);
        self.client.fetch(&params).await
    }
}
