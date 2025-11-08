use schwab_api_core::{ApiClient, AsyncHttpClient, HttpError};
use schwab_api_types::{
    CandleList, ExpirationChain, GetMovers200Response, Hours, InstrumentResponse, OptionChain,
    QuoteResponseObject,
};
use std::collections::HashMap;
use std::ops::Deref;

use crate::{MarketdataConfig, MarketdataParams};

/// Async client for Schwab Market Data API
pub struct AsyncMarketdataClient<C: AsyncHttpClient> {
    client: ApiClient<C, MarketdataConfig>,
}

impl<C: AsyncHttpClient> AsyncMarketdataClient<C> {
    pub fn new(client: C) -> Self {
        Self {
            client: ApiClient::new(client),
        }
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
        access_token: &str,
        symbols: &str,
        fields: Option<&str>,
        indicative: Option<bool>,
    ) -> Result<HashMap<String, QuoteResponseObject>, HttpError> {
        let params = MarketdataParams::get_quotes(access_token, symbols, fields, indicative);
        self.client.fetch(&params).await
    }

    /// Get quote for a single symbol
    pub async fn get_quote(
        &self,
        access_token: &str,
        symbol: &str,
        fields: Option<&str>,
    ) -> Result<HashMap<String, QuoteResponseObject>, HttpError> {
        let params = MarketdataParams::get_quote(access_token, symbol, fields);
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
        access_token: &str,
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
            access_token,
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
    pub async fn get_expiration_chain(
        &self,
        access_token: &str,
        symbol: &str,
    ) -> Result<ExpirationChain, HttpError> {
        let params = MarketdataParams::get_expiration_chain(access_token, symbol);
        self.client.fetch(&params).await
    }

    /// Get price history for a symbol
    #[allow(clippy::too_many_arguments)]
    pub async fn get_price_history(
        &self,
        access_token: &str,
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
            access_token,
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
        access_token: &str,
        symbol: &str,
        sort: Option<&str>,
        frequency: Option<i32>,
    ) -> Result<GetMovers200Response, HttpError> {
        let params = MarketdataParams::get_movers(access_token, symbol, sort, frequency);
        self.client.fetch(&params).await
    }

    /// Get market hours for multiple markets
    pub async fn get_market_hours(
        &self,
        access_token: &str,
        markets: &str,
        date: Option<&str>,
    ) -> Result<HashMap<String, HashMap<String, Hours>>, HttpError> {
        let params = MarketdataParams::get_market_hours(access_token, markets, date);
        self.client.fetch(&params).await
    }

    /// Get market hours for a single market
    pub async fn get_market_hour(
        &self,
        access_token: &str,
        market: &str,
        date: Option<&str>,
    ) -> Result<HashMap<String, HashMap<String, Hours>>, HttpError> {
        let params = MarketdataParams::get_market_hour(access_token, market, date);
        self.client.fetch(&params).await
    }

    /// Get instruments by symbol and projection
    pub async fn get_instruments(
        &self,
        access_token: &str,
        symbol: &str,
        projection: &str,
    ) -> Result<HashMap<String, Vec<InstrumentResponse>>, HttpError> {
        let params = MarketdataParams::get_instruments(access_token, symbol, projection);
        self.client.fetch(&params).await
    }

    /// Get instrument by CUSIP
    pub async fn get_instruments_by_cusip(
        &self,
        access_token: &str,
        cusip: &str,
    ) -> Result<HashMap<String, Vec<InstrumentResponse>>, HttpError> {
        let params = MarketdataParams::get_instruments_by_cusip(access_token, cusip);
        self.client.fetch(&params).await
    }
}
