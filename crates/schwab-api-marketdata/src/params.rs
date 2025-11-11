//! Request parameter builders for Schwab Market Data API endpoints.
//!
//! This module provides type-safe parameter construction for all Market Data API operations.
//! Each method corresponds to an API endpoint and returns a `RequestParams` struct
//! configured with the appropriate HTTP method, path, and query parameters.
//!
//! Query parameters are serialized using `serde_urlencoded` for proper URL encoding
//! and consistent handling of optional parameters.

use http::Method;
use serde::Serialize;

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
        #[derive(Serialize)]
        struct Query<'a> {
            symbols: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            fields: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            indicative: Option<bool>,
        }

        let query = serde_urlencoded::to_string(&Query {
            symbols,
            fields,
            indicative,
        })
        .ok();

        RequestParams {
            access_token,
            method: Method::GET,
            path: "/quotes".to_string(),
            query,
            body: None,
        }
    }

    /// Build params for getQuote operation - Get quote for a single symbol
    pub fn get_quote<'a>(
        access_token: &'a str,
        symbol: &str,
        fields: Option<&str>,
    ) -> RequestParams<'a> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            fields: Option<&'a str>,
        }

        let path = format!("/{symbol}/quotes");
        let query = serde_urlencoded::to_string(&Query { fields }).ok();

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
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            #[serde(rename = "contractType")]
            #[serde(skip_serializing_if = "Option::is_none")]
            contract_type: Option<&'a str>,
            #[serde(rename = "strikeCount")]
            #[serde(skip_serializing_if = "Option::is_none")]
            strike_count: Option<i32>,
            #[serde(rename = "includeUnderlyingQuote")]
            #[serde(skip_serializing_if = "Option::is_none")]
            include_underlying_quote: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            strategy: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            interval: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            strike: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            range: Option<&'a str>,
            #[serde(rename = "fromDate")]
            #[serde(skip_serializing_if = "Option::is_none")]
            from_date: Option<&'a str>,
            #[serde(rename = "toDate")]
            #[serde(skip_serializing_if = "Option::is_none")]
            to_date: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            volatility: Option<f64>,
            #[serde(rename = "underlyingPrice")]
            #[serde(skip_serializing_if = "Option::is_none")]
            underlying_price: Option<f64>,
            #[serde(rename = "interestRate")]
            #[serde(skip_serializing_if = "Option::is_none")]
            interest_rate: Option<f64>,
            #[serde(rename = "daysToExpiration")]
            #[serde(skip_serializing_if = "Option::is_none")]
            days_to_expiration: Option<i32>,
            #[serde(rename = "expMonth")]
            #[serde(skip_serializing_if = "Option::is_none")]
            exp_month: Option<&'a str>,
            #[serde(rename = "optionType")]
            #[serde(skip_serializing_if = "Option::is_none")]
            option_type: Option<&'a str>,
        }

        let query = serde_urlencoded::to_string(&Query {
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
        })
        .ok();

        RequestParams {
            access_token,
            method: Method::GET,
            path: "/chains".to_string(),
            query,
            body: None,
        }
    }

    /// Build params for getExpirationChain operation - Get option expiration chain
    pub fn get_expiration_chain<'a>(access_token: &'a str, symbol: &str) -> RequestParams<'a> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
        }

        let query = serde_urlencoded::to_string(&Query { symbol }).ok();

        RequestParams {
            access_token,
            method: Method::GET,
            path: "/expirationchain".to_string(),
            query,
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
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            #[serde(rename = "periodType")]
            #[serde(skip_serializing_if = "Option::is_none")]
            period_type: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            period: Option<i32>,
            #[serde(rename = "frequencyType")]
            #[serde(skip_serializing_if = "Option::is_none")]
            frequency_type: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            frequency: Option<i32>,
            #[serde(rename = "startDate")]
            #[serde(skip_serializing_if = "Option::is_none")]
            start_date: Option<i64>,
            #[serde(rename = "endDate")]
            #[serde(skip_serializing_if = "Option::is_none")]
            end_date: Option<i64>,
            #[serde(rename = "needExtendedHoursData")]
            #[serde(skip_serializing_if = "Option::is_none")]
            need_extended_hours_data: Option<bool>,
            #[serde(rename = "needPreviousClose")]
            #[serde(skip_serializing_if = "Option::is_none")]
            need_previous_close: Option<bool>,
        }

        let query = serde_urlencoded::to_string(&Query {
            symbol,
            period_type,
            period,
            frequency_type,
            frequency,
            start_date,
            end_date,
            need_extended_hours_data,
            need_previous_close,
        })
        .ok();

        RequestParams {
            access_token,
            method: Method::GET,
            path: "/pricehistory".to_string(),
            query,
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
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            sort: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            frequency: Option<i32>,
        }

        let query = serde_urlencoded::to_string(&Query { sort, frequency }).ok();

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
        #[derive(Serialize)]
        struct Query<'a> {
            markets: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            date: Option<&'a str>,
        }

        let query = serde_urlencoded::to_string(&Query { markets, date }).ok();

        RequestParams {
            access_token,
            method: Method::GET,
            path: "/markets".to_string(),
            query,
            body: None,
        }
    }

    /// Build params for getMarketHour operation - Get market hours for a single market
    pub fn get_market_hour<'a>(
        access_token: &'a str,
        market: &str,
        date: Option<&str>,
    ) -> RequestParams<'a> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            date: Option<&'a str>,
        }

        let query = serde_urlencoded::to_string(&Query { date }).ok();

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
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            projection: &'a str,
        }

        let query = serde_urlencoded::to_string(&Query { symbol, projection }).ok();

        RequestParams {
            access_token,
            method: Method::GET,
            path: "/instruments".to_string(),
            query,
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
