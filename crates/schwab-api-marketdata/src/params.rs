use http::Method;

use schwab_api_core::RequestParams;

pub trait MarketdataParams {
    // Quotes

    /// Get quotes for multiple symbols
    /// - symbols: Comma-separated list of symbols (e.g., "AAPL,MSFT,TSLA")
    /// - fields: Optional subset of data (e.g., "quote,reference")
    /// - indicative: Include indicative quotes for ETFs
    fn get_quotes_params<'a>(
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
            body: None,
            path: "/quotes".to_string(),
            method: Method::GET,
            query: Some(query),
        }
    }

    /// Get quote for a single symbol
    /// - symbol: Single symbol (e.g., "AAPL")
    /// - fields: Optional subset of data (e.g., "quote,reference")
    fn get_quote_params<'a>(
        access_token: &'a str,
        symbol: &str,
        fields: Option<&str>,
    ) -> RequestParams<'a> {
        let path = format!("/{symbol}/quotes");
        let query = fields.map(|f| format!("fields={f}"));

        RequestParams {
            access_token,
            body: None,
            path,
            method: Method::GET,
            query,
        }
    }

    // Option Chains

    /// Get option chain for an optionable symbol
    /// - symbol: Symbol (e.g., "AAPL")
    /// - contract_type: Type of contracts (CALL, PUT, ALL)
    /// - strike_count: Number of strikes above/below ATM
    /// - include_underlying_quote: Include underlying quote
    /// - strategy: Option chain strategy (SINGLE, ANALYTICAL, COVERED, etc.)
    /// - interval: Strike interval for spread strategies
    /// - strike: Strike price
    /// - range: Range (ITM/NTM/OTM)
    /// - from_date: From date (yyyy-MM-dd)
    /// - to_date: To date (yyyy-MM-dd)
    /// - volatility: Volatility for ANALYTICAL strategy
    /// - underlying_price: Underlying price for ANALYTICAL strategy
    /// - interest_rate: Interest rate for ANALYTICAL strategy
    /// - days_to_expiration: Days to expiration for ANALYTICAL strategy
    /// - exp_month: Expiration month
    /// - option_type: Option type
    fn get_option_chain_params<'a>(
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
            body: None,
            path: "/chains".to_string(),
            method: Method::GET,
            query: Some(query),
        }
    }

    // Options Expiration Chain

    /// Get option expiration chain for an optionable symbol
    /// - symbol: Symbol (e.g., "AAPL")
    fn get_expiration_chain_params<'a>(access_token: &'a str, symbol: &str) -> RequestParams<'a> {
        let query = format!("symbol={symbol}");

        RequestParams {
            access_token,
            body: None,
            path: "/expirationchain".to_string(),
            method: Method::GET,
            query: Some(query),
        }
    }

    // Price History

    /// Get price history for a symbol
    /// - symbol: Symbol (e.g., "AAPL")
    /// - period_type: Period type (day, month, year, ytd)
    /// - period: Number of periods
    /// - frequency_type: Frequency type (minute, daily, weekly, monthly)
    /// - frequency: Frequency (1, 5, 10, 15, 30 for minute; 1 for others)
    /// - start_date: Start date in milliseconds since epoch
    /// - end_date: End date in milliseconds since epoch
    /// - need_extended_hours_data: Include extended hours data
    /// - need_previous_close: Include previous close
    fn get_price_history_params<'a>(
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
        if let Some(ehd) = need_extended_hours_data {
            query_parts.push(format!("needExtendedHoursData={ehd}"));
        }
        if let Some(pc) = need_previous_close {
            query_parts.push(format!("needPreviousClose={pc}"));
        }

        let query = query_parts.join("&");

        RequestParams {
            access_token,
            body: None,
            path: "/pricehistory".to_string(),
            method: Method::GET,
            query: Some(query),
        }
    }

    // Movers

    /// Get movers for a specific index
    /// - symbol: Index symbol ($DJI, $COMPX, $SPX, NYSE, NASDAQ, OTCBB, INDEX_ALL, EQUITY_ALL, OPTION_ALL, OPTION_PUT, OPTION_CALL)
    /// - sort: Sort by attribute (VOLUME, TRADES, PERCENT_CHANGE_UP, PERCENT_CHANGE_DOWN)
    /// - frequency: Frequency (0, 1, 5, 10, 30, 60)
    fn get_movers_params<'a>(
        access_token: &'a str,
        symbol: &str,
        sort: Option<&str>,
        frequency: Option<i32>,
    ) -> RequestParams<'a> {
        let path = format!("/movers/{symbol}");

        let mut query_parts = Vec::new();
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
            body: None,
            path,
            method: Method::GET,
            query,
        }
    }

    // Market Hours

    /// Get market hours for multiple markets
    /// - markets: Comma-separated list of markets (equity, option, bond, future, forex)
    /// - date: Date in YYYY-MM-DD format
    fn get_markets_params<'a>(
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
            body: None,
            path: "/markets".to_string(),
            method: Method::GET,
            query: Some(query),
        }
    }

    /// Get market hours for a single market
    /// - market: Market (equity, option, bond, future, forex)
    /// - date: Date in YYYY-MM-DD format
    fn get_market_params<'a>(
        access_token: &'a str,
        market: &str,
        date: Option<&str>,
    ) -> RequestParams<'a> {
        let path = format!("/markets/{market}");

        let query = date.map(|d| format!("date={d}"));

        RequestParams {
            access_token,
            body: None,
            path,
            method: Method::GET,
            query,
        }
    }

    // Instruments

    /// Get instruments by symbol and projection
    /// - symbol: Symbol to search for
    /// - projection: Search type (symbol-search, symbol-regex, desc-search, desc-regex, search, fundamental)
    fn get_instruments_params<'a>(
        access_token: &'a str,
        symbol: &str,
        projection: &str,
    ) -> RequestParams<'a> {
        let query = format!("symbol={symbol}&projection={projection}");

        RequestParams {
            access_token,
            body: None,
            path: "/instruments".to_string(),
            method: Method::GET,
            query: Some(query),
        }
    }

    /// Get instrument by CUSIP
    /// - cusip: CUSIP identifier
    fn get_instrument_by_cusip_params<'a>(access_token: &'a str, cusip: &str) -> RequestParams<'a> {
        let path = format!("/instruments/{cusip}");

        RequestParams {
            access_token,
            body: None,
            path,
            method: Method::GET,
            query: None,
        }
    }
}
