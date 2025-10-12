use anyhow::Result;

use crate::config::{ConfigManager, SchwabConfig, TokenManager};
use crate::oauth::refresh_access_token;

/// Handles automatic token refresh based on configuration
pub struct AutoRefresher {
    config: SchwabConfig,
    token_manager: TokenManager,
}

impl AutoRefresher {
    pub fn new(config: SchwabConfig, config_manager: &ConfigManager) -> Result<Self> {
        let token_manager = TokenManager::new(config_manager)?;
        Ok(Self {
            config,
            token_manager,
        })
    }

    /// Ensure we have a valid access token, refreshing if necessary
    pub async fn ensure_valid_token(&self) -> Result<()> {
        // Only proceed if auto_refresh is enabled
        if !self.config.preferences.auto_refresh {
            // If auto-refresh disabled, just check and warn if expired
            if self.token_manager.is_access_token_expired()? {
                return Err(anyhow::anyhow!(
                    "Access token has expired. Run 'chuck refresh' or enable auto-refresh."
                ));
            }
            return Ok(());
        }

        // Check if token is expired or expires soon (5 min buffer)
        if self.token_manager.is_access_token_expired()? || self.token_manager.expires_soon(300)? {
            println!("🔄 Access token expired, auto-refreshing...");
            self.refresh_token().await?;
            println!("✅ Token refreshed successfully");
        }

        Ok(())
    }

    /// Internal method to refresh the token
    async fn refresh_token(&self) -> Result<()> {
        // Get refresh token
        let refresh_token = self.token_manager.get_refresh_token()?.ok_or_else(|| {
            anyhow::anyhow!("No refresh token available. Please run 'chuck login' again.")
        })?;

        // Make refresh request to Schwab API
        let token_response = refresh_access_token(&self.config, &refresh_token).await
            .map_err(|e| {
                anyhow::anyhow!("Auto-refresh failed: {}. Try running 'chuck refresh' manually or 'chuck login' if refresh token expired.", e)
            })?;

        // Save new tokens
        self.token_manager
            .update_access_token(&token_response.access_token, token_response.expires_in)?;

        Ok(())
    }

    /// Check if auto-refresh is enabled in config
    pub fn is_enabled(&self) -> bool {
        self.config.preferences.auto_refresh
    }

    /// Get the current token expiration status
    pub fn get_token_status(&self) -> Result<TokenStatus> {
        if !self.token_manager.has_tokens() {
            return Ok(TokenStatus::NoTokens);
        }

        if self.token_manager.is_access_token_expired()? {
            Ok(TokenStatus::Expired)
        } else if self.token_manager.expires_soon(300)? {
            Ok(TokenStatus::ExpiresSoon)
        } else {
            Ok(TokenStatus::Valid)
        }
    }
}

/// Token status for display purposes
#[derive(Debug, PartialEq)]
pub enum TokenStatus {
    NoTokens,
    Expired,
    ExpiresSoon,
    Valid,
}
