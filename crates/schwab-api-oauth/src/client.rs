use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use schwab_api_core::{ApiClient, ApiConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::config::OAuthConfig;
use crate::error::{OAuthError, Result};

/// Response structure from Schwab OAuth2 token endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
    pub id_token: String,
}

// OAuthConfig already exists, but we need it to implement ApiConfig
// We'll use a marker type for the ApiConfig implementation
pub struct OAuthApiConfig;

impl ApiConfig for OAuthApiConfig {
    fn base_url() -> &'static str {
        // OAuth doesn't use a base URL in the same way, but we need to satisfy the trait
        // We'll override the URL construction in OAuthClient methods
        ""
    }
}

/// OAuth client for Schwab API authentication
pub struct OAuthClient<C> {
    inner: ApiClient<C, OAuthApiConfig>,
    config: OAuthConfig,
}

impl<C> OAuthClient<C> {
    /// Create a new OAuth client with the given HTTP client and configuration
    pub fn new(client: C, config: OAuthConfig) -> Self {
        Self {
            inner: ApiClient::new(client),
            config,
        }
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

    /// Access the inner ApiClient for direct operations
    pub fn inner(&self) -> &ApiClient<C, OAuthApiConfig> {
        &self.inner
    }

    /// Access the inner ApiClient mutably
    pub fn inner_mut(&mut self) -> &mut ApiClient<C, OAuthApiConfig> {
        &mut self.inner
    }
}

// Implement Deref to allow calling ApiClient methods directly
impl<C> std::ops::Deref for OAuthClient<C> {
    type Target = ApiClient<C, OAuthApiConfig>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Implement DerefMut to allow mutable access to ApiClient methods
impl<C> std::ops::DerefMut for OAuthClient<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// OAuth-specific methods that require reqwest::Client
impl OAuthClient<reqwest::Client> {
    /// Exchange authorization code for access and refresh tokens
    pub async fn exchange_code_for_token(&self, code: &str) -> Result<TokenResponse> {
        // Create Basic Auth header
        let auth_string = format!("{}:{}", self.config.client_id, self.config.client_secret);
        let auth_header = format!("Basic {}", STANDARD.encode(auth_string.as_bytes()));

        let mut form = HashMap::new();
        form.insert("grant_type", "authorization_code");
        form.insert("code", code);
        form.insert("redirect_uri", &self.config.redirect_uri);

        let response = self
            .inner
            .client
            .inner()
            .post(&self.config.token_url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form)
            .send()
            .await
            .map_err(|e| OAuthError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OAuthError::TokenRequestFailed {
                status: status.as_u16(),
                body,
            });
        }

        let token_response: TokenResponse = response.json().await.map_err(|e| {
            OAuthError::NetworkError(format!("Failed to parse token response: {}", e))
        })?;

        Ok(token_response)
    }

    /// Refresh an access token using a refresh token
    pub async fn refresh_access_token(&self, refresh_token: &str) -> Result<TokenResponse> {
        // Create Basic Auth header
        let auth_string = format!("{}:{}", self.config.client_id, self.config.client_secret);
        let auth_header = format!("Basic {}", STANDARD.encode(auth_string.as_bytes()));

        let mut form = HashMap::new();
        form.insert("grant_type", "refresh_token");
        form.insert("refresh_token", refresh_token);

        let response = self
            .inner
            .client
            .inner()
            .post(&self.config.token_url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form)
            .send()
            .await
            .map_err(|e| OAuthError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OAuthError::TokenRequestFailed {
                status: status.as_u16(),
                body,
            });
        }

        let token_response: TokenResponse = response.json().await.map_err(|e| {
            OAuthError::NetworkError(format!("Failed to parse refresh response: {}", e))
        })?;

        Ok(token_response)
    }
}
