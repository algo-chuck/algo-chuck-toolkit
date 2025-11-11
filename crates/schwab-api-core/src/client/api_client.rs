//! Generic API client implementation for Schwab API interactions.

use http::{Request, Response};
use serde::Serialize;
use serde::de::{DeserializeOwned, Error as DeError};
use std::sync::{Arc, RwLock};

use crate::client::HttpClient;
use crate::client::params::RequestParams;
use crate::client::traits::{AsyncHttpClient, SyncHttpClient};
use crate::config::ApiConfig;
use crate::error::HttpError;
use crate::response::{HttpResponse, SchwabSuccess};

/// Generic API client that works with any API configuration.
///
/// This client provides a unified interface for making HTTP requests to any
/// Schwab API endpoint (Trader, Market Data, etc.) with either async or sync
/// HTTP clients.
///
/// The client stores the access token internally, eliminating the need to pass it
/// with every request. The token can be updated at runtime using `set_access_token()`.
///
/// # Type Parameters
///
/// * `C` - The HTTP client type (must implement `AsyncHttpClient` or `SyncHttpClient`)
/// * `Cfg` - The API configuration type (must implement `ApiConfig`)
///
/// # Examples
///
/// ```ignore
/// use schwab_api_core::{ApiClient, ApiConfig};
///
/// pub struct TraderConfig;
///
/// impl ApiConfig for TraderConfig {
///     fn base_url() -> &'static str {
///         "https://api.schwabapi.com/trader/v1"
///     }
/// }
///
/// let http_client = ureq::Agent::new();
/// let client = ApiClient::<_, TraderConfig>::new(http_client, "your_access_token");
///
/// // Update token when refreshed
/// client.set_access_token("new_access_token");
/// ```
pub struct ApiClient<C, Cfg: ApiConfig> {
    pub client: HttpClient<C>,
    access_token: Arc<RwLock<String>>,
    _config: std::marker::PhantomData<Cfg>,
}

impl<C, Cfg: ApiConfig> ApiClient<C, Cfg> {
    /// Create a new API client with the given HTTP client and access token.
    ///
    /// # Arguments
    ///
    /// * `client` - An HTTP client implementing either `AsyncHttpClient` or `SyncHttpClient`
    /// * `access_token` - The OAuth2 access token for authentication
    pub fn new(client: C, access_token: impl Into<String>) -> Self {
        Self {
            client: HttpClient::new(client),
            access_token: Arc::new(RwLock::new(access_token.into())),
            _config: std::marker::PhantomData,
        }
    }

    /// Update the access token used for authentication.
    ///
    /// This is useful when the token has been refreshed and you want to continue
    /// using the same client instance with the new token.
    ///
    /// # Arguments
    ///
    /// * `new_token` - The new access token to use
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // After refreshing the token
    /// let new_token = oauth_client.refresh_access_token(&refresh_token).await?;
    /// client.set_access_token(&new_token.access_token);
    /// ```
    pub fn set_access_token(&self, new_token: impl Into<String>) {
        let mut token = self.access_token.write().unwrap();
        *token = new_token.into();
    }

    /// Get a copy of the current access token.
    ///
    /// This is useful for debugging or if you need to persist the token externally.
    pub fn get_access_token(&self) -> String {
        self.access_token.read().unwrap().clone()
    }

    /// Build a complete URL from a path and optional query string.
    ///
    /// This method combines the API's base URL with the provided path and
    /// query parameters to create a complete request URL.
    fn build_url(&self, path: &str, query_string_opt: Option<&str>) -> String {
        let base = Cfg::base_url();
        let query_len = query_string_opt
            .filter(|q| !q.is_empty())
            .map_or(0, |q| q.len() + 1); // +1 for '?'

        // Pre-allocate string with exact capacity needed
        let mut url = String::with_capacity(base.len() + path.len() + query_len);
        url.push_str(base);
        url.push_str(path);

        if let Some(query) = query_string_opt.filter(|q| !q.is_empty()) {
            url.push('?');
            url.push_str(query);
        }

        url
    }

    /// Build an HTTP request from request parameters.
    ///
    /// This method constructs a complete HTTP request including:
    /// - URL (base + path + query)
    /// - Authorization header with Bearer token (from stored token)
    /// - Request body (serialized as JSON if present)
    ///
    /// # Arguments
    ///
    /// * `params` - The request parameters
    ///
    /// # Returns
    ///
    /// A `Result` containing the built request or an error
    pub fn build_request<B: Serialize>(
        &self,
        params: &RequestParams<B>,
    ) -> Result<Request<String>, HttpError> {
        let url = self.build_url(&params.path, params.query.as_deref());

        // Use the stored access token
        let token = self.access_token.read().unwrap();
        let bearer_token = format!("Bearer {}", *token);
        drop(token); // Release the read lock

        // Serialize the body if present
        let final_body = match &params.body {
            Some(body) => serde_json::to_string(body)?,
            None => String::new(),
        };

        Request::builder()
            .uri(url)
            .method(&params.method)
            .header("Authorization", bearer_token)
            .body(final_body)
            .map_err(|e| HttpError::RequestFailed(format!("Request builder failed: {}", e)))
    }

    /// Parse a successful HTTP response into the expected type.
    ///
    /// This method performs robust deserialization with fallback handling for
    /// unexpected response structures.
    ///
    /// # Arguments
    ///
    /// * `response` - The HTTP response to parse
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed response data or an error
    pub fn parse_ok_response<R: DeserializeOwned>(
        &self,
        response: &Response<String>,
    ) -> Result<R, HttpError> {
        // Perform robust parsing into the generic wrapper type
        let ok_result: SchwabSuccess<R> = response.json()?;

        // Inspect the result, log the anomaly, and return the final type
        match ok_result {
            SchwabSuccess::Ok(data) => Ok(data),
            SchwabSuccess::MismatchedResponse(value) => {
                // Log the anomaly: API returned 2xx, but structure was mismatched
                eprintln!(
                    "WARNING: API returned status {}, but response body was mismatched:\n {:#?}",
                    response.status(),
                    value
                );

                // Treat the unexpected structure as a serialization failure
                Err(HttpError::SerializationError(DeError::custom(format!(
                    "Received mismatched {} response structure:\n {:#?}",
                    response.status(),
                    value
                ))))
            }
        }
    }
}

// ============================================================================
// Async implementations
// ============================================================================

impl<C, Cfg> ApiClient<C, Cfg>
where
    C: AsyncHttpClient,
    Cfg: ApiConfig,
    HttpError: From<C::Error>,
{
    /// Fetch and deserialize a response asynchronously.
    ///
    /// This is the primary method for making async API requests that return data.
    ///
    /// # Type Parameters
    ///
    /// * `R` - The expected response type
    /// * `B` - The request body type
    ///
    /// # Arguments
    ///
    /// * `params` - The request parameters
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response or an error
    pub async fn fetch<R, B>(&self, params: &RequestParams<B>) -> Result<R, HttpError>
    where
        R: DeserializeOwned,
        B: Serialize,
    {
        let request = self.build_request(params)?;

        let response = self
            .client
            .execute(request)
            .await
            .map_err(HttpError::from)?;

        let typed = self.parse_ok_response(&response)?;
        Ok(typed)
    }

    /// Execute a request without parsing a response body asynchronously.
    ///
    /// This method is useful for API calls that don't return data (e.g., DELETE requests).
    ///
    /// # Type Parameters
    ///
    /// * `B` - The request body type
    ///
    /// # Arguments
    ///
    /// * `params` - The request parameters
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error
    pub async fn execute<B>(&self, params: &RequestParams<B>) -> Result<(), HttpError>
    where
        B: Serialize,
    {
        let request = self.build_request(params)?;

        self.client
            .execute(request)
            .await
            .map_err(HttpError::from)?;

        Ok(())
    }
}

// ============================================================================
// Sync implementations
// ============================================================================

impl<C, Cfg> ApiClient<C, Cfg>
where
    C: SyncHttpClient,
    Cfg: ApiConfig,
    HttpError: From<C::Error>,
{
    /// Fetch and deserialize a response synchronously.
    ///
    /// This is the primary method for making blocking API requests that return data.
    ///
    /// # Type Parameters
    ///
    /// * `R` - The expected response type
    /// * `B` - The request body type
    ///
    /// # Arguments
    ///
    /// * `params` - The request parameters
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response or an error
    pub fn fetch_sync<R, B>(&self, params: &RequestParams<B>) -> Result<R, HttpError>
    where
        R: DeserializeOwned,
        B: Serialize,
    {
        let request = self.build_request(params)?;

        let response = self.client.execute_sync(request).map_err(HttpError::from)?;

        let typed = self.parse_ok_response(&response)?;
        Ok(typed)
    }

    /// Execute a request without parsing a response body synchronously.
    ///
    /// This method is useful for API calls that don't return data (e.g., DELETE requests).
    ///
    /// # Type Parameters
    ///
    /// * `B` - The request body type
    ///
    /// # Arguments
    ///
    /// * `params` - The request parameters
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error
    pub fn execute_sync<B>(&self, params: &RequestParams<B>) -> Result<(), HttpError>
    where
        B: Serialize,
    {
        let request = self.build_request(params)?;

        self.client.execute_sync(request).map_err(HttpError::from)?;

        Ok(())
    }
}
