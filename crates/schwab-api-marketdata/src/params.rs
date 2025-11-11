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
        // Pre-calculate capacity for query string
        let mut capacity = "symbols=".len() + symbols.len();

        if let Some(f) = fields {
            capacity += "&fields=".len() + f.len();
        }
        if indicative.is_some() {
            capacity += "&indicative=false".len(); // worst case
        }

        let mut query = String::with_capacity(capacity);
        use std::fmt::Write;
        let _ = write!(query, "symbols={}", symbols);

        if let Some(f) = fields {
            let _ = write!(query, "&fields={}", f);
        }
        if let Some(ind) = indicative {
            let _ = write!(query, "&indicative={}", ind);
        }

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
        // Pre-calculate rough capacity (symbol + up to 16 optional params with ~20 chars each)
        let capacity = "symbol=".len() + symbol.len() + 400;

        let mut query = String::with_capacity(capacity);
        use std::fmt::Write;
        let _ = write!(query, "symbol={}", symbol);

        if let Some(ct) = contract_type {
            let _ = write!(query, "&contractType={}", ct);
        }
        if let Some(sc) = strike_count {
            let _ = write!(query, "&strikeCount={}", sc);
        }
        if let Some(iq) = include_underlying_quote {
            let _ = write!(query, "&includeUnderlyingQuote={}", iq);
        }
        if let Some(s) = strategy {
            let _ = write!(query, "&strategy={}", s);
        }
        if let Some(i) = interval {
            let _ = write!(query, "&interval={}", i);
        }
        if let Some(st) = strike {
            let _ = write!(query, "&strike={}", st);
        }
        if let Some(r) = range {
            let _ = write!(query, "&range={}", r);
        }
        if let Some(fd) = from_date {
            let _ = write!(query, "&fromDate={}", fd);
        }
        if let Some(td) = to_date {
            let _ = write!(query, "&toDate={}", td);
        }
        if let Some(v) = volatility {
            let _ = write!(query, "&volatility={}", v);
        }
        if let Some(up) = underlying_price {
            let _ = write!(query, "&underlyingPrice={}", up);
        }
        if let Some(ir) = interest_rate {
            let _ = write!(query, "&interestRate={}", ir);
        }
        if let Some(dte) = days_to_expiration {
            let _ = write!(query, "&daysToExpiration={}", dte);
        }
        if let Some(em) = exp_month {
            let _ = write!(query, "&expMonth={}", em);
        }
        if let Some(ot) = option_type {
            let _ = write!(query, "&optionType={}", ot);
        }

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
        // Pre-calculate rough capacity
        let capacity = "symbol=".len() + symbol.len() + 200;

        let mut query = String::with_capacity(capacity);
        use std::fmt::Write;
        let _ = write!(query, "symbol={}", symbol);

        if let Some(pt) = period_type {
            let _ = write!(query, "&periodType={}", pt);
        }
        if let Some(p) = period {
            let _ = write!(query, "&period={}", p);
        }
        if let Some(ft) = frequency_type {
            let _ = write!(query, "&frequencyType={}", ft);
        }
        if let Some(f) = frequency {
            let _ = write!(query, "&frequency={}", f);
        }
        if let Some(sd) = start_date {
            let _ = write!(query, "&startDate={}", sd);
        }
        if let Some(ed) = end_date {
            let _ = write!(query, "&endDate={}", ed);
        }
        if let Some(nehd) = need_extended_hours_data {
            let _ = write!(query, "&needExtendedHoursData={}", nehd);
        }
        if let Some(npc) = need_previous_close {
            let _ = write!(query, "&needPreviousClose={}", npc);
        }

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
        let query = match (sort, frequency) {
            (None, None) => None,
            (Some(s), None) => Some(format!("sort={}", s)),
            (None, Some(f)) => Some(format!("frequency={}", f)),
            (Some(s), Some(f)) => Some(format!("sort={}&frequency={}", s, f)),
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
        let query = match date {
            Some(d) => format!("markets={}&date={}", markets, d),
            None => format!("markets={}", markets),
        };

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
