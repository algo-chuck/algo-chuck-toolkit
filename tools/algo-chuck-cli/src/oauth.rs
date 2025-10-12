use anyhow::{Context, Result};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

use crate::config::SchwabConfig;

/// Response structure from Schwab OAuth2 token endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct SchwabTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
    pub id_token: String,
}

/// Generate a random state parameter for OAuth2 PKCE
pub fn generate_state() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Build the Schwab OAuth2 authorization URL
pub fn build_schwab_auth_url(config: &SchwabConfig, state: &str) -> Result<String> {
    let mut auth_url =
        Url::parse(&config.api.auth_url).context("Failed to parse Schwab authorization URL")?;

    let client_id = config
        .client
        .client_id
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Client ID not configured"))?;

    let params = vec![
        ("response_type", "code"),
        ("client_id", client_id),
        ("redirect_uri", &config.api.callback_url),
        ("scope", "readonly"),
        ("state", state),
    ];

    for (key, value) in params {
        auth_url.query_pairs_mut().append_pair(key, value);
    }

    Ok(auth_url.to_string())
}

/// Exchange authorization code for access and refresh tokens
pub async fn exchange_code_for_token(
    config: &SchwabConfig,
    code: &str,
) -> Result<SchwabTokenResponse> {
    let client = Client::new();

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

    // Create Basic Auth header
    let auth_string = format!("{}:{}", client_id, client_secret);
    let auth_header = format!("Basic {}", STANDARD.encode(auth_string.as_bytes()));

    let mut form = HashMap::new();
    form.insert("grant_type", "authorization_code");
    form.insert("code", code);
    form.insert("redirect_uri", &config.api.callback_url);

    let response = client
        .post(&config.api.token_url)
        .header("Authorization", auth_header)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form)
        .send()
        .await
        .context("Failed to send token request")?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow::anyhow!(
            "Token request failed with status {}: {}",
            status,
            error_text
        ));
    }

    let token_response: SchwabTokenResponse = response
        .json()
        .await
        .context("Failed to parse token response")?;

    Ok(token_response)
}

/// Refresh an access token using a refresh token
pub async fn refresh_access_token(
    config: &SchwabConfig,
    refresh_token: &str,
) -> Result<SchwabTokenResponse> {
    let client = Client::new();

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

    // Create Basic Auth header
    let auth_string = format!("{}:{}", client_id, client_secret);
    let auth_header = format!("Basic {}", STANDARD.encode(auth_string.as_bytes()));

    let mut form = HashMap::new();
    form.insert("grant_type", "refresh_token");
    form.insert("refresh_token", refresh_token);

    let response = client
        .post(&config.api.token_url)
        .header("Authorization", auth_header)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form)
        .send()
        .await
        .context("Failed to send refresh token request")?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow::anyhow!(
            "Refresh token request failed with status {}: {}",
            status,
            error_text
        ));
    }

    let token_response: SchwabTokenResponse = response
        .json()
        .await
        .context("Failed to parse token refresh response")?;

    Ok(token_response)
}
