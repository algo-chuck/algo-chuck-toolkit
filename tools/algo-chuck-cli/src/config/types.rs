use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchwabConfig {
    pub client: ClientConfig,
    pub preferences: PreferencesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub client_id: Option<String>,
    #[serde(skip)] // Never serialize to config file - only loaded from encrypted storage
    pub client_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferencesConfig {
    pub auto_refresh: bool,
    pub refresh_buffer: u32,
    pub browser_timeout: u32,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptedTokens {
    pub access_token: Option<String>,
    pub access_token_expiry: Option<i64>,
    pub refresh_token: Option<String>,
    pub refresh_token_expiry: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptedCredentials {
    pub client_secret: Option<String>,
    pub created_at: Option<i64>,
    pub version: u32,
}

impl Default for SchwabConfig {
    fn default() -> Self {
        Self {
            client: ClientConfig {
                client_id: None,
                client_secret: None, // Will be populated from encrypted storage
            },
            preferences: PreferencesConfig {
                auto_refresh: true,
                refresh_buffer: 300,
                browser_timeout: 300,
                log_level: "info".to_string(),
            },
        }
    }
}

impl SchwabConfig {
    /// OAuth2 callback URL used by the application
    /// This is the URL that should be registered in the Schwab Developer Portal
    pub const CALLBACK_URL: &str = "https://127.0.0.1:8443/oauth/schwab/callback";

    /// Callback server bind address
    pub const CALLBACK_ADDRESS: &str = "127.0.0.1:8443";

    /// Callback path for OAuth2 responses
    pub const CALLBACK_PATH: &str = "/oauth/schwab/callback";

    /// Schwab authorization url to start the oath precess
    pub const SCHWAB_AUTH_URL: &str = "https://api.schwabapi.com/v1/oauth/authorize";

    /// Schwab token url to exchange authorization code for tokens
    pub const SCHWAB_TOKEN_URL: &str = "https://api.schwabapi.com/v1/oauth/token";
}
