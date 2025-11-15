//! Synchronous client implementation for Schwab Market Data API.
//!
//! This module provides a blocking/sync client for interacting with the Schwab Market Data API,
//! supporting operations like quotes, option chains, price history, and market hours.

use schwab_api_core::{ApiClient, HttpError, Result, SyncHttpClient};
use schwab_api_types::marketdata::*;
use std::collections::HashMap;
use std::ops::Deref;

use crate::{MarketdataConfig, MarketdataParams};

/// Synchronous/blocking client for Schwab Market Data API.
///
/// This client provides blocking methods for all Market Data API operations, including:
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
/// use schwab_api_marketdata::SyncMarketdataClient;
///
/// let http_client = ureq::Agent::new();
/// let client = SyncMarketdataClient::new(http_client);
///
/// let quotes = client.get_quote("your_token", "AAPL", None)?;
/// ```
pub struct SyncMarketdataClient<C: SyncHttpClient> {
    client: ApiClient<C, MarketdataConfig>,
}

impl<C: SyncHttpClient> SyncMarketdataClient<C> {
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

impl<C: SyncHttpClient> Deref for SyncMarketdataClient<C> {
    type Target = ApiClient<C, MarketdataConfig>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl<C> SyncMarketdataClient<C>
where
    C: SyncHttpClient,
    HttpError: From<C::Error>,
{
    /// Get quotes for multiple symbols
    pub fn get_quotes(
        &self,
        params: &GetQuotesParams<'_>,
    ) -> Result<HashMap<String, QuoteResponseObject>> {
        let params = MarketdataParams::get_quotes(params);
        self.client.fetch_sync(&params)
    }

    /// Get quote for a single symbol
    pub fn get_quote(
        &self,
        params: &GetQuoteParams<'_>,
    ) -> Result<HashMap<String, QuoteResponseObject>> {
        let params = MarketdataParams::get_quote(params);
        self.client.fetch_sync(&params)
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
    pub fn get_chain(&self, params: &GetChainParams<'_>) -> Result<OptionChain> {
        let params = MarketdataParams::get_chain(params);
        self.client.fetch_sync(&params)
    }

    /// Get option expiration chain for an optionable symbol
    pub fn get_expiration_chain(
        &self,
        params: &GetExpirationChainParams<'_>,
    ) -> Result<ExpirationChain> {
        let params = MarketdataParams::get_expiration_chain(params);
        self.client.fetch_sync(&params)
    }

    /// Get price history for a symbol
    pub fn get_price_history(&self, params: &GetPriceHistoryParams<'_>) -> Result<CandleList> {
        let params = MarketdataParams::get_price_history(params);
        self.client.fetch_sync(&params)
    }

    /// Get movers for a specific index
    pub fn get_movers(&self, params: &GetMoversParams<'_>) -> Result<GetMovers200Response> {
        let params = MarketdataParams::get_movers(params);
        self.client.fetch_sync(&params)
    }

    /// Get market hours for multiple markets
    pub fn get_market_hours(
        &self,
        params: &GetMarketHoursParams<'_>,
    ) -> Result<HashMap<String, HashMap<String, Hours>>> {
        let params = MarketdataParams::get_market_hours(params);
        self.client.fetch_sync(&params)
    }

    /// Get market hours for a single market
    pub fn get_market_hour(
        &self,
        params: &GetMarketHourParams<'_>,
    ) -> Result<HashMap<String, HashMap<String, Hours>>> {
        let params = MarketdataParams::get_market_hour(params);
        self.client.fetch_sync(&params)
    }

    /// Get instruments by symbol and projection
    pub fn get_instruments(
        &self,
        params: &GetInstrumentsParams<'_>,
    ) -> Result<HashMap<String, Vec<InstrumentResponse>>> {
        let params = MarketdataParams::get_instruments(params);
        self.client.fetch_sync(&params)
    }

    /// Get instrument by CUSIP
    pub fn get_instruments_by_cusip(
        &self,
        params: &GetInstrumentByCusipParams<'_>,
    ) -> Result<HashMap<String, Vec<InstrumentResponse>>> {
        let params = MarketdataParams::get_instruments_by_cusip(params);
        self.client.fetch_sync(&params)
    }
}
