use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;

use crate::TokenResponse;
use crate::config::OAuthConfig;
use crate::error::{OAuthError, Result};

/// Sync OAuth client for Schwab API authentication
pub struct SyncOAuthClient {
    client: ureq::Agent,
    config: OAuthConfig,
}

impl SyncOAuthClient {
    /// Create a new sync OAuth client
    pub fn new(client: ureq::Agent, config: OAuthConfig) -> Self {
        Self { client, config }
    }

    /// Build the OAuth2 authorization URL
    pub fn build_auth_url(&self, state: &str) -> Result<String> {
        self.config.build_auth_url(state)
    }

    /// Get the redirect URI configured for this client
    pub fn redirect_uri(&self) -> &str {
        &self.config.redirect_uri
    }

    /// Get the client ID
    pub fn client_id(&self) -> &str {
        &self.config.client_id
    }

    /// Exchange authorization code for access and refresh tokens
    pub fn exchange_code_for_token(&self, code: &str) -> Result<TokenResponse> {
        // Create Basic Auth header
        let auth_string = format!("{}:{}", self.config.client_id, self.config.client_secret);
        let auth_header = format!("Basic {}", STANDARD.encode(auth_string.as_bytes()));

        let form_data = format!(
            "grant_type=authorization_code&code={}&redirect_uri={}",
            urlencoding::encode(code),
            urlencoding::encode(&self.config.redirect_uri)
        );

        let response = self
            .client
            .post(&self.config.token_url)
            .set("Authorization", &auth_header)
            .set("Content-Type", "application/x-www-form-urlencoded")
            .send_string(&form_data)
            .map_err(|e| OAuthError::NetworkError(e.to_string()))?;

        let status = response.status();
        if status < 200 || status >= 300 {
            let body = response
                .into_string()
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OAuthError::TokenRequestFailed { status, body });
        }

        let body = response
            .into_string()
            .map_err(|e| OAuthError::NetworkError(format!("Failed to read response: {}", e)))?;

        let token_response: TokenResponse = serde_json::from_str(&body).map_err(|e| {
            OAuthError::NetworkError(format!("Failed to parse token response: {}", e))
        })?;

        Ok(token_response)
    }

    /// Refresh an access token using a refresh token
    pub fn refresh_access_token(&self, refresh_token: &str) -> Result<TokenResponse> {
        // Create Basic Auth header
        let auth_string = format!("{}:{}", self.config.client_id, self.config.client_secret);
        let auth_header = format!("Basic {}", STANDARD.encode(auth_string.as_bytes()));

        let form_data = format!(
            "grant_type=refresh_token&refresh_token={}",
            urlencoding::encode(refresh_token)
        );

        let response = self
            .client
            .post(&self.config.token_url)
            .set("Authorization", &auth_header)
            .set("Content-Type", "application/x-www-form-urlencoded")
            .send_string(&form_data)
            .map_err(|e| OAuthError::NetworkError(e.to_string()))?;

        let status = response.status();
        if status < 200 || status >= 300 {
            let body = response
                .into_string()
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OAuthError::TokenRequestFailed { status, body });
        }

        let body = response
            .into_string()
            .map_err(|e| OAuthError::NetworkError(format!("Failed to read response: {}", e)))?;

        let token_response: TokenResponse = serde_json::from_str(&body).map_err(|e| {
            OAuthError::NetworkError(format!("Failed to parse refresh response: {}", e))
        })?;

        Ok(token_response)
    }
}
