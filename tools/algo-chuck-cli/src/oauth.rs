use anyhow::{Context, Result};
use schwab_api_oauth::{OAuthConfig, SyncOAuthClient, TokenResponse};

use crate::config::SchwabConfig;

/// Generate a random state parameter for OAuth2 PKCE
pub fn generate_state() -> String {
    OAuthConfig::generate_state()
}

/// Build the Schwab OAuth2 authorization URL
pub fn build_schwab_auth_url(client_id: &str, state: &str) -> Result<String> {
    let config = OAuthConfig::new(client_id, "", SchwabConfig::CALLBACK_URL);

    config
        .build_auth_url(state)
        .context("Failed to build authorization URL")
}

/// Exchange authorization code for access and refresh tokens
pub fn exchange_code_for_token(config: &SchwabConfig, code: &str) -> Result<TokenResponse> {
    let client_id = config
        .client
        .client_id
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Client ID not configured"))?;

    let client_secret = config
        .client
        .client_secret
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Client secret not configured"))?;

    let http_client = ureq::Agent::new();
    let oauth_config = OAuthConfig::new(client_id, client_secret, SchwabConfig::CALLBACK_URL);
    let oauth_client = SyncOAuthClient::new(http_client, oauth_config);

    oauth_client
        .exchange_code_for_token(code)
        .context("Failed to exchange code for token")
}

/// Refresh an access token using a refresh token
pub fn refresh_access_token(config: &SchwabConfig, refresh_token: &str) -> Result<TokenResponse> {
    let client_id = config
        .client
        .client_id
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Client ID not configured"))?;

    let client_secret = config
        .client
        .client_secret
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Client secret not configured"))?;

    let http_client = ureq::Agent::new();
    let oauth_config = OAuthConfig::new(client_id, client_secret, SchwabConfig::CALLBACK_URL);
    let oauth_client = SyncOAuthClient::new(http_client, oauth_config);

    oauth_client
        .refresh_access_token(refresh_token)
        .context("Failed to refresh access token")
}
