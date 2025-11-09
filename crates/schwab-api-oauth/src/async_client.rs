use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use std::collections::HashMap;

use crate::TokenResponse;
use crate::config::OAuthConfig;
use crate::error::{OAuthError, Result};

/// Async OAuth client for Schwab API authentication
pub struct AsyncOAuthClient {
    client: reqwest::Client,
    config: OAuthConfig,
}

impl AsyncOAuthClient {
    /// Create a new async OAuth client
    pub fn new(client: reqwest::Client, config: OAuthConfig) -> Self {
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
    pub async fn exchange_code_for_token(&self, code: &str) -> Result<TokenResponse> {
        // Create Basic Auth header
        let auth_string = format!("{}:{}", self.config.client_id, self.config.client_secret);
        let auth_header = format!("Basic {}", STANDARD.encode(auth_string.as_bytes()));

        let mut form = HashMap::new();
        form.insert("grant_type", "authorization_code");
        form.insert("code", code);
        form.insert("redirect_uri", &self.config.redirect_uri);

        let response = self
            .client
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
            .client
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
