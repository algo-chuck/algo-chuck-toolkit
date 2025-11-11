//! Request parameter builders for Schwab Market Data API endpoints.
//!
//! This module provides type-safe parameter construction for all Market Data API operations.
//! Each method corresponds to an API endpoint and returns a `RequestParams` struct
//! configured with the appropriate HTTP method, path, and query parameters.

use http::Method;

use schwab_api_core::RequestParams;

/// Parameter builders for all Schwab Market Data API endpoints.
///
/// Function names match OpenAPI operationIds (converted to snake_case).
/// All methods are static and return `RequestParams` configured for the specific endpoint.
pub struct MarketdataParams;

impl MarketdataParams {
    /// Build params for getQuotes operation - Get quotes for multiple symbols
    pub fn get_quotes<'a>(
        access_token: &'a str,
        symbols: &str,
        fields: Option<&str>,
        indicative: Option<bool>,
    ) -> RequestParams<'a> {
        let mut query_parts = vec![format!("symbols={symbols}")];

        if let Some(f) = fields {
            query_parts.push(format!("fields={f}"));
        }

        if let Some(ind) = indicative {
            query_parts.push(format!("indicative={ind}"));
        }

        let query = query_parts.join("&");

        RequestParams {
            access_token,
            method: Method::GET,
            path: "/quotes".to_string(),
            query: Some(query),
            body: None,
        }
    }

    /// Build params for getQuote operation - Get quote for a single symbol
    pub fn get_quote<'a>(
        access_token: &'a str,
        symbol: &str,
        fields: Option<&str>,
    ) -> RequestParams<'a> {
        let path = format!("/{symbol}/quotes");
        let query = fields.map(|f| format!("fields={f}"));

        RequestParams {
            access_token,
            method: Method::GET,
            path,
            query,
            body: None,
        }
    }

    /// Build params for getChain operation - Get option chain for an optionable symbol
    #[allow(clippy::too_many_arguments)]
    pub fn get_chain<'a>(
        access_token: &'a str,
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
    ) -> RequestParams<'a> {
        let mut query_parts = vec![format!("symbol={symbol}")];

        if let Some(ct) = contract_type {
            query_parts.push(format!("contractType={ct}"));
        }
        if let Some(sc) = strike_count {
            query_parts.push(format!("strikeCount={sc}"));
        }
        if let Some(iq) = include_underlying_quote {
            query_parts.push(format!("includeUnderlyingQuote={iq}"));
        }
        if let Some(s) = strategy {
            query_parts.push(format!("strategy={s}"));
        }
        if let Some(i) = interval {
            query_parts.push(format!("interval={i}"));
        }
        if let Some(st) = strike {
            query_parts.push(format!("strike={st}"));
        }
        if let Some(r) = range {
            query_parts.push(format!("range={r}"));
        }
        if let Some(fd) = from_date {
            query_parts.push(format!("fromDate={fd}"));
        }
        if let Some(td) = to_date {
            query_parts.push(format!("toDate={td}"));
        }
        if let Some(v) = volatility {
            query_parts.push(format!("volatility={v}"));
        }
        if let Some(up) = underlying_price {
            query_parts.push(format!("underlyingPrice={up}"));
        }
        if let Some(ir) = interest_rate {
            query_parts.push(format!("interestRate={ir}"));
        }
        if let Some(dte) = days_to_expiration {
            query_parts.push(format!("daysToExpiration={dte}"));
        }
        if let Some(em) = exp_month {
            query_parts.push(format!("expMonth={em}"));
        }
        if let Some(ot) = option_type {
            query_parts.push(format!("optionType={ot}"));
        }

        let query = query_parts.join("&");

        RequestParams {
            access_token,
            method: Method::GET,
            path: "/chains".to_string(),
            query: Some(query),
            body: None,
        }
    }

    /// Build params for getExpirationChain operation - Get option expiration chain
    pub fn get_expiration_chain<'a>(access_token: &'a str, symbol: &str) -> RequestParams<'a> {
        let query = format!("symbol={symbol}");

        RequestParams {
            access_token,
            method: Method::GET,
            path: "/expirationchain".to_string(),
            query: Some(query),
            body: None,
        }
    }

    /// Build params for getPriceHistory operation - Get price history for a symbol
    #[allow(clippy::too_many_arguments)]
    pub fn get_price_history<'a>(
        access_token: &'a str,
        symbol: &str,
        period_type: Option<&str>,
        period: Option<i32>,
        frequency_type: Option<&str>,
        frequency: Option<i32>,
        start_date: Option<i64>,
        end_date: Option<i64>,
        need_extended_hours_data: Option<bool>,
        need_previous_close: Option<bool>,
    ) -> RequestParams<'a> {
        let mut query_parts = vec![format!("symbol={symbol}")];

        if let Some(pt) = period_type {
            query_parts.push(format!("periodType={pt}"));
        }
        if let Some(p) = period {
            query_parts.push(format!("period={p}"));
        }
        if let Some(ft) = frequency_type {
            query_parts.push(format!("frequencyType={ft}"));
        }
        if let Some(f) = frequency {
            query_parts.push(format!("frequency={f}"));
        }
        if let Some(sd) = start_date {
            query_parts.push(format!("startDate={sd}"));
        }
        if let Some(ed) = end_date {
            query_parts.push(format!("endDate={ed}"));
        }
        if let Some(nehd) = need_extended_hours_data {
            query_parts.push(format!("needExtendedHoursData={nehd}"));
        }
        if let Some(npc) = need_previous_close {
            query_parts.push(format!("needPreviousClose={npc}"));
        }

        let query = query_parts.join("&");

        RequestParams {
            access_token,
            method: Method::GET,
            path: "/pricehistory".to_string(),
            query: Some(query),
            body: None,
        }
    }

    /// Build params for getMovers operation - Get movers for a specific index
    pub fn get_movers<'a>(
        access_token: &'a str,
        symbol: &str,
        sort: Option<&str>,
        frequency: Option<i32>,
    ) -> RequestParams<'a> {
        let mut query_parts = vec![];

        if let Some(s) = sort {
            query_parts.push(format!("sort={s}"));
        }
        if let Some(f) = frequency {
            query_parts.push(format!("frequency={f}"));
        }

        let query = if query_parts.is_empty() {
            None
        } else {
            Some(query_parts.join("&"))
        };

        RequestParams {
            access_token,
            method: Method::GET,
            path: format!("/movers/{symbol}"),
            query,
            body: None,
        }
    }

    /// Build params for getMarketHours operation - Get market hours for multiple markets
    pub fn get_market_hours<'a>(
        access_token: &'a str,
        markets: &str,
        date: Option<&str>,
    ) -> RequestParams<'a> {
        let mut query_parts = vec![format!("markets={markets}")];

        if let Some(d) = date {
            query_parts.push(format!("date={d}"));
        }

        let query = query_parts.join("&");

        RequestParams {
            access_token,
            method: Method::GET,
            path: "/markets".to_string(),
            query: Some(query),
            body: None,
        }
    }

    /// Build params for getMarketHour operation - Get market hours for a single market
    pub fn get_market_hour<'a>(
        access_token: &'a str,
        market: &str,
        date: Option<&str>,
    ) -> RequestParams<'a> {
        let query = date.map(|d| format!("date={d}"));

        RequestParams {
            access_token,
            method: Method::GET,
            path: format!("/markets/{market}"),
            query,
            body: None,
        }
    }

    /// Build params for getInstruments operation - Get instruments by symbol and projection
    pub fn get_instruments<'a>(
        access_token: &'a str,
        symbol: &str,
        projection: &str,
    ) -> RequestParams<'a> {
        let query = format!("symbol={symbol}&projection={projection}");

        RequestParams {
            access_token,
            method: Method::GET,
            path: "/instruments".to_string(),
            query: Some(query),
            body: None,
        }
    }

    /// Build params for getInstrumentsByCusip operation - Get instrument by CUSIP
    pub fn get_instruments_by_cusip<'a>(access_token: &'a str, cusip: &str) -> RequestParams<'a> {
        RequestParams {
            access_token,
            method: Method::GET,
            path: format!("/instruments/{cusip}"),
            query: None,
            body: None,
        }
    }
}
