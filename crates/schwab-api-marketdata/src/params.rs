//! Request parameter builders for Schwab Market Data API endpoints.
//!
//! This module provides simplified parameter construction using the public parameter
//! structs from `schwab-api-types`. Each method accepts a parameter struct and returns
//! a `RequestParams` configured for the specific endpoint.

use http::Method;

use schwab_api_core::RequestParams;
use schwab_api_types::marketdata::*;

/// Parameter builders for all Schwab Market Data API endpoints.
///
/// Function names match OpenAPI operationIds (converted to snake_case).
/// All methods are static and return `RequestParams` configured for the specific endpoint.
pub struct MarketdataParams;

impl MarketdataParams {
    /// Build params for getQuotes operation - Get quotes for multiple symbols
    pub fn get_quotes(params: &GetQuotesParams) -> RequestParams {
        let query = serde_urlencoded::to_string(params).ok();

        RequestParams {
            method: Method::GET,
            path: "/quotes".to_string(),
            query,
            body: None,
        }
    }

    /// Build params for getQuote operation - Get quote for a single symbol
    pub fn get_quote(params: &GetQuoteParams) -> RequestParams {
        let query = serde_urlencoded::to_string(params).ok();
        let path = format!("/{}/quotes", params.symbol);

        RequestParams {
            method: Method::GET,
            path,
            query,
            body: None,
        }
    }

    /// Build params for getChain operation - Get option chain for an optionable symbol
    pub fn get_chain(params: &GetChainParams) -> RequestParams {
        let query = serde_urlencoded::to_string(params).ok();

        RequestParams {
            method: Method::GET,
            path: "/chains".to_string(),
            query,
            body: None,
        }
    }

    /// Build params for getExpirationChain operation - Get option expiration chain
    pub fn get_expiration_chain(params: &GetExpirationChainParams) -> RequestParams {
        let query = serde_urlencoded::to_string(params).ok();

        RequestParams {
            method: Method::GET,
            path: "/expirationchain".to_string(),
            query,
            body: None,
        }
    }

    /// Build params for getPriceHistory operation - Get price history for a symbol
    pub fn get_price_history(params: &GetPriceHistoryParams) -> RequestParams {
        let query = serde_urlencoded::to_string(params).ok();

        RequestParams {
            method: Method::GET,
            path: "/pricehistory".to_string(),
            query,
            body: None,
        }
    }

    /// Build params for getMovers operation - Get movers for a specific index
    pub fn get_movers(params: &GetMoversParams) -> RequestParams {
        let query = serde_urlencoded::to_string(params).ok();

        RequestParams {
            method: Method::GET,
            path: format!("/movers/{}", params.symbol),
            query,
            body: None,
        }
    }

    /// Build params for getMarketHours operation - Get market hours for multiple markets
    pub fn get_market_hours(params: &GetMarketHoursParams) -> RequestParams {
        let query = serde_urlencoded::to_string(params).ok();

        RequestParams {
            method: Method::GET,
            path: "/markets".to_string(),
            query,
            body: None,
        }
    }

    /// Build params for getMarketHour operation - Get market hours for a single market
    pub fn get_market_hour(params: &GetMarketHourParams) -> RequestParams {
        let query = serde_urlencoded::to_string(params).ok();

        RequestParams {
            method: Method::GET,
            path: format!("/markets/{}", params.market),
            query,
            body: None,
        }
    }

    /// Build params for getInstruments operation - Get instruments by symbol and projection
    pub fn get_instruments(params: &GetInstrumentsParams) -> RequestParams {
        let query = serde_urlencoded::to_string(params).ok();

        RequestParams {
            method: Method::GET,
            path: "/instruments".to_string(),
            query,
            body: None,
        }
    }

    /// Build params for getInstrumentsByCusip operation - Get instrument by CUSIP
    pub fn get_instruments_by_cusip(params: &GetInstrumentByCusipParams) -> RequestParams {
        RequestParams {
            method: Method::GET,
            path: format!("/instruments/{}", params.cusip),
            query: None,
            body: None,
        }
    }
}
