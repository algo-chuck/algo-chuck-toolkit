/// OAuth configuration and constants for Schwab API
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    /// OAuth2 authorization endpoint
    pub auth_url: String,
    /// OAuth2 token endpoint
    pub token_url: String,
    /// OAuth2 redirect URI (must match what's registered in Schwab Developer Portal)
    pub redirect_uri: String,
}

impl OAuthConfig {
    /// Default Schwab OAuth endpoints
    pub const DEFAULT_AUTH_URL: &'static str = "https://api.schwabapi.com/v1/oauth/authorize";
    pub const DEFAULT_TOKEN_URL: &'static str = "https://api.schwabapi.com/v1/oauth/token";

    /// Create a new OAuth configuration with custom settings
    pub fn new(auth_url: String, token_url: String, redirect_uri: String) -> Self {
        Self {
            auth_url,
            token_url,
            redirect_uri,
        }
    }

    /// Create OAuth configuration with default Schwab endpoints
    pub fn with_redirect_uri(redirect_uri: impl Into<String>) -> Self {
        Self {
            auth_url: Self::DEFAULT_AUTH_URL.to_string(),
            token_url: Self::DEFAULT_TOKEN_URL.to_string(),
            redirect_uri: redirect_uri.into(),
        }
    }
}

impl Default for OAuthConfig {
    fn default() -> Self {
        Self {
            auth_url: Self::DEFAULT_AUTH_URL.to_string(),
            token_url: Self::DEFAULT_TOKEN_URL.to_string(),
            redirect_uri: "https://127.0.0.1:8443/oauth/schwab/callback".to_string(),
        }
    }
}
