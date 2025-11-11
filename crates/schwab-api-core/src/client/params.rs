//! Request parameter types for API calls.

/// Parameters for building and executing HTTP requests to the Schwab API.
///
/// This struct encapsulates the information needed to make an API request,
/// including HTTP method, path, query parameters, and optional body.
/// The access token is now stored in the `ApiClient` itself.
///
/// # Type Parameters
///
/// * `B` - The type of the request body (defaults to `()` for no body)
///
/// # Examples
///
/// ```ignore
/// use schwab_api_core::RequestParams;
/// use http::Method;
///
/// let params = RequestParams {
///     body: None,
///     method: Method::GET,
///     path: "/accounts".to_string(),
///     query: None,
/// };
/// ```
#[derive(Debug)]
pub struct RequestParams<B = ()> {
    /// Optional request body (for POST, PUT, PATCH requests)
    pub body: Option<B>,

    /// HTTP method for the request (GET, POST, PUT, DELETE, etc.)
    pub method: http::Method,

    /// The API path (e.g., "/accounts", "/orders")
    pub path: String,

    /// Optional query string parameters (without the leading '?')
    pub query: Option<String>,
}
