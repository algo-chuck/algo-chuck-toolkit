//! Configuration traits for API-specific clients.

/// Configuration trait for API-specific clients (Trader, Market Data, etc.)
///
/// This trait is implemented by each API's configuration type to provide
/// the base URL for that particular API endpoint.
///
/// # Examples
///
/// ```ignore
/// use schwab_api_core::ApiConfig;
///
/// pub struct TraderConfig;
///
/// impl ApiConfig for TraderConfig {
///     fn base_url() -> &'static str {
///         "https://api.schwabapi.com/trader/v1"
///     }
/// }
/// ```
pub trait ApiConfig {
    /// Returns the base URL for this API endpoint.
    ///
    /// This should include the protocol, host, and any version prefix,
    /// but should not include trailing slashes or specific resource paths.
    ///
    /// # Example return values
    ///
    /// - `"https://api.schwabapi.com/trader/v1"`
    /// - `"https://api.schwabapi.com/marketdata/v1"`
    fn base_url() -> &'static str;

    /// Returns the API name for error context.
    ///
    /// Used to properly classify API errors as belonging to Trader or Marketdata APIs.
    ///
    /// # Example return values
    ///
    /// - `"trader"`
    /// - `"marketdata"`
    fn api_name() -> &'static str;
}
