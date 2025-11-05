/// OAuth configuration and constants for Schwab API
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    /// OAuth2 authorization endpoint
    pub auth_url: String,
    /// OAuth2 token endpoint
    pub token_url: String,
    /// OAuth2 redirect URI (must match what's registered in Schwab Developer Portal)
    pub redirect_uri: String,
    /// OAuth2 client ID
    pub client_id: String,
    /// OAuth2 client secret
    pub client_secret: String,
}

impl OAuthConfig {
    /// Default Schwab OAuth endpoints
    pub const DEFAULT_AUTH_URL: &'static str = "https://api.schwabapi.com/v1/oauth/authorize";
    pub const DEFAULT_TOKEN_URL: &'static str = "https://api.schwabapi.com/v1/oauth/token";

    /// Create a new OAuthConfig with the specified parameters.
    /// Uses default Schwab API endpoints for auth_url and token_url.
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self {
            auth_url: Self::DEFAULT_AUTH_URL.to_string(),
            token_url: Self::DEFAULT_TOKEN_URL.to_string(),
            redirect_uri: redirect_uri.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }

    /// Create a new OAuth configuration with custom endpoints
    pub fn with_custom_endpoints(
        auth_url: impl Into<String>,
        token_url: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self {
            auth_url: auth_url.into(),
            token_url: token_url.into(),
            redirect_uri: redirect_uri.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }
}
