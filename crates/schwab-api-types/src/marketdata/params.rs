//! Parameter types for Schwab Market Data API endpoints.
//!
//! These structs provide type-safe parameter construction for Market Data API operations.
//! All structs implement `Serialize` for URL encoding.

use serde::Serialize;

/// Parameters for fetching quotes for multiple symbols.
#[derive(Debug, Clone, Serialize)]
pub struct GetQuotesParams<'a> {
    /// Comma-separated list of symbols
    pub symbols: &'a str,
    /// Fields to include in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<&'a str>,
    /// Include indicative symbol quotes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicative: Option<bool>,
}

/// Parameters for fetching a quote for a single symbol.
#[derive(Debug, Clone, Serialize)]
pub struct GetQuoteParams<'a> {
    /// The symbol to get a quote for
    pub symbol: &'a str,
    /// Fields to include in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<&'a str>,
}

/// Parameters for fetching an option chain.
#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct GetChainParams<'a> {
    /// The underlying symbol
    pub symbol: &'a str,
    /// Type of contracts to return (CALL, PUT, ALL)
    #[serde(rename = "contractType", skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<&'a str>,
    /// Number of strikes to return
    #[serde(rename = "strikeCount", skip_serializing_if = "Option::is_none")]
    pub strike_count: Option<i32>,
    /// Include underlying quote data
    #[serde(
        rename = "includeUnderlyingQuote",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_underlying_quote: Option<bool>,
    /// Strategy chain (e.g., SINGLE, ANALYTICAL, COVERED, VERTICAL, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<&'a str>,
    /// Strike interval for spread strategy chains
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<f64>,
    /// Strike price to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike: Option<f64>,
    /// Range of strikes (ITM, OTM, NTM, ALL, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<&'a str>,
    /// Start date for expiration filtering (yyyy-MM-dd)
    #[serde(rename = "fromDate", skip_serializing_if = "Option::is_none")]
    pub from_date: Option<&'a str>,
    /// End date for expiration filtering (yyyy-MM-dd)
    #[serde(rename = "toDate", skip_serializing_if = "Option::is_none")]
    pub to_date: Option<&'a str>,
    /// Volatility to use in calculations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volatility: Option<f64>,
    /// Underlying price for calculations
    #[serde(rename = "underlyingPrice", skip_serializing_if = "Option::is_none")]
    pub underlying_price: Option<f64>,
    /// Interest rate to use in calculations
    #[serde(rename = "interestRate", skip_serializing_if = "Option::is_none")]
    pub interest_rate: Option<f64>,
    /// Days to expiration
    #[serde(rename = "daysToExpiration", skip_serializing_if = "Option::is_none")]
    pub days_to_expiration: Option<i32>,
    /// Return only options expiring in the specified month (JAN, FEB, etc.)
    #[serde(rename = "expMonth", skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<&'a str>,
    /// Type of contracts to return (CALL, PUT, ALL)
    #[serde(rename = "optionType", skip_serializing_if = "Option::is_none")]
    pub option_type: Option<&'a str>,
}

/// Parameters for fetching option expiration chain.
#[derive(Debug, Clone, Serialize)]
pub struct GetExpirationChainParams<'a> {
    /// The underlying symbol
    pub symbol: &'a str,
}

/// Parameters for fetching price history.
#[derive(Debug, Clone, Serialize)]
pub struct GetPriceHistoryParams<'a> {
    /// The symbol
    pub symbol: &'a str,
    /// Period type (day, month, year, ytd)
    #[serde(rename = "periodType", skip_serializing_if = "Option::is_none")]
    pub period_type: Option<&'a str>,
    /// Number of periods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    /// Frequency type (minute, daily, weekly, monthly)
    #[serde(rename = "frequencyType", skip_serializing_if = "Option::is_none")]
    pub frequency_type: Option<&'a str>,
    /// Frequency (1, 5, 10, 15, 30 for minute, 1 for daily/weekly/monthly)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i32>,
    /// Start date as epoch milliseconds
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i64>,
    /// End date as epoch milliseconds
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i64>,
    /// Include extended hours data
    #[serde(
        rename = "needExtendedHoursData",
        skip_serializing_if = "Option::is_none"
    )]
    pub need_extended_hours_data: Option<bool>,
    /// Include previous close
    #[serde(rename = "needPreviousClose", skip_serializing_if = "Option::is_none")]
    pub need_previous_close: Option<bool>,
}

/// Parameters for fetching market movers.
#[derive(Debug, Clone, Serialize)]
pub struct GetMoversParams<'a> {
    /// The index symbol ($DJI, $COMPX, $SPX, etc.)
    pub symbol: &'a str,
    /// Sort order (VOLUME, TRADES, PERCENT_CHANGE_UP, PERCENT_CHANGE_DOWN)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<&'a str>,
    /// Frequency in minutes (0, 1, 5, 10, 30, 60)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i32>,
}

/// Parameters for fetching market hours for multiple markets.
#[derive(Debug, Clone, Serialize)]
pub struct GetMarketHoursParams<'a> {
    /// Comma-separated list of markets (equity, option, bond, future, forex)
    pub markets: &'a str,
    /// Date in yyyy-MM-dd format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<&'a str>,
}

/// Parameters for fetching market hours for a single market.
#[derive(Debug, Clone, Serialize)]
pub struct GetMarketHourParams<'a> {
    /// The market (equity, option, bond, future, forex)
    pub market: &'a str,
    /// Date in yyyy-MM-dd format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<&'a str>,
}

/// Parameters for searching instruments.
#[derive(Debug, Clone, Serialize)]
pub struct GetInstrumentsParams<'a> {
    /// The symbol or partial symbol to search for
    pub symbol: &'a str,
    /// The projection type (symbol-search, symbol-regex, desc-search, desc-regex, search, fundamental)
    pub projection: &'a str,
}

/// Parameters for getting an instrument by CUSIP.
#[derive(Debug, Clone, Serialize)]
pub struct GetInstrumentByCusipParams<'a> {
    /// The CUSIP identifier
    pub cusip: &'a str,
}
