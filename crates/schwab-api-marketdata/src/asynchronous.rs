use schwab_api_core::{AsyncHttpClient, HttpError};
use schwab_api_types::{
    CandleList, ExpirationChain, GetMovers200Response, Hours, InstrumentResponse, OptionChain,
    QuoteResponseObject,
};
use std::collections::HashMap;

use crate::client::MarketdataClient;
use crate::params::MarketdataParams;

impl<C> MarketdataClient<C>
where
    C: AsyncHttpClient,
    HttpError: From<C::Error>,
{
    // Quotes

    /// Get quotes for multiple symbols
    pub async fn get_quotes(
        &self,
        access_token: &str,
        symbols: &str,
        fields: Option<&str>,
        indicative: Option<bool>,
    ) -> Result<HashMap<String, QuoteResponseObject>, HttpError> {
        let params =
            MarketdataClient::<C>::get_quotes_params(access_token, symbols, fields, indicative);
        self.fetch(&params).await
    }

    /// Get quote for a single symbol
    pub async fn get_quote(
        &self,
        access_token: &str,
        symbol: &str,
        fields: Option<&str>,
    ) -> Result<HashMap<String, QuoteResponseObject>, HttpError> {
        let params = MarketdataClient::<C>::get_quote_params(access_token, symbol, fields);
        self.fetch(&params).await
    }

    // Option Chains

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
        let params = MarketdataClient::<C>::get_chain_params(
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
        self.fetch(&params).await
    }

    // Options Expiration Chain

    /// Get option expiration chain for an optionable symbol
    pub async fn get_expiration_chain(
        &self,
        access_token: &str,
        symbol: &str,
    ) -> Result<ExpirationChain, HttpError> {
        let params = MarketdataClient::<C>::get_expiration_chain_params(access_token, symbol);
        self.fetch(&params).await
    }

    // Price History

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
        let params = MarketdataClient::<C>::get_price_history_params(
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
        self.fetch(&params).await
    }

    // Movers

    /// Get movers for a specific index
    pub async fn get_movers(
        &self,
        access_token: &str,
        symbol: &str,
        sort: Option<&str>,
        frequency: Option<i32>,
    ) -> Result<GetMovers200Response, HttpError> {
        let params =
            MarketdataClient::<C>::get_movers_params(access_token, symbol, sort, frequency);
        self.fetch(&params).await
    }

    // Market Hours

    /// Get market hours for multiple markets
    pub async fn get_market_hours(
        &self,
        access_token: &str,
        markets: &str,
        date: Option<&str>,
    ) -> Result<HashMap<String, HashMap<String, Hours>>, HttpError> {
        let params = MarketdataClient::<C>::get_market_hours_params(access_token, markets, date);
        self.fetch(&params).await
    }

    /// Get market hours for a single market
    pub async fn get_market_hour(
        &self,
        access_token: &str,
        market: &str,
        date: Option<&str>,
    ) -> Result<HashMap<String, HashMap<String, Hours>>, HttpError> {
        let params = MarketdataClient::<C>::get_market_hour_params(access_token, market, date);
        self.fetch(&params).await
    }

    // Instruments

    /// Get instruments by symbol and projection
    pub async fn get_instruments(
        &self,
        access_token: &str,
        symbol: &str,
        projection: &str,
    ) -> Result<HashMap<String, Vec<InstrumentResponse>>, HttpError> {
        let params =
            MarketdataClient::<C>::get_instruments_params(access_token, symbol, projection);
        self.fetch(&params).await
    }

    /// Get instrument by CUSIP
    pub async fn get_instrument_by_cusip(
        &self,
        access_token: &str,
        cusip: &str,
    ) -> Result<HashMap<String, Vec<InstrumentResponse>>, HttpError> {
        let params = MarketdataClient::<C>::get_instrument_by_cusip_params(access_token, cusip);
        self.fetch(&params).await
    }
}
