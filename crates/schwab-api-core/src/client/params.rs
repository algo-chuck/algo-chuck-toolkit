//! Request parameter types for API calls.

/// Parameters for building and executing HTTP requests to the Schwab API.
///
/// This struct encapsulates all the information needed to make an API request,
/// including authentication, HTTP method, path, query parameters, and optional body.
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
///     access_token: "your_token_here",
///     body: None,
///     method: Method::GET,
///     path: "/accounts".to_string(),
///     query: None,
/// };
/// ```
#[derive(Debug)]
pub struct RequestParams<'a, B = ()> {
    /// The OAuth2 access token for authenticating the request
    pub access_token: &'a str,

    /// Optional request body (for POST, PUT, PATCH requests)
    pub body: Option<B>,

    /// HTTP method for the request (GET, POST, PUT, DELETE, etc.)
    pub method: http::Method,

    /// The API path (e.g., "/accounts", "/orders")
    pub path: String,

    /// Optional query string parameters (without the leading '?')
    pub query: Option<String>,
}
