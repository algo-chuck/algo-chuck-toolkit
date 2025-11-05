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

    // Options Expiration Chain

    // Price History

    // Movers

    // Market Hours

    // Instruments
}
