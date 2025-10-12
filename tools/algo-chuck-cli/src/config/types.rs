use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchwabConfig {
    pub api: ApiConfig,
    pub client: ClientConfig,
    pub preferences: PreferencesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub auth_url: String,
    pub token_url: String,
    pub callback_url: String,
    pub callback_address: String,
    pub callback_path: String,
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
            api: ApiConfig {
                auth_url: "https://api.schwabapi.com/v1/oauth/authorize".to_string(),
                token_url: "https://api.schwabapi.com/v1/oauth/token".to_string(),
                callback_url: "https://127.0.0.1:6309/oauth/callback".to_string(),
                callback_address: "127.0.0.1:6309".to_string(),
                callback_path: "/oauth/callback".to_string(),
            },
            client: ClientConfig {
                client_id: None,
                client_secret: None, // Will be populated from encrypted storage
            },
            preferences: PreferencesConfig {
                auto_refresh: true,
                browser_timeout: 300,
                log_level: "info".to_string(),
            },
        }
    }
}

impl SchwabConfig {
    pub fn parse_callback_url(&mut self) -> Result<()> {
        let url = url::Url::parse(&self.api.callback_url).context(format!(
            "Invalid callback URL format: {}",
            self.api.callback_url
        ))?;

        let host = url
            .host_str()
            .ok_or_else(|| anyhow::anyhow!("Callback URL must include a host"))?
            .to_string();

        let port = url.port();
        let address = if let Some(port) = port {
            format!("{}:{}", host, port)
        } else {
            host
        };

        let path = url.path().to_string();

        self.api.callback_address = address;
        self.api.callback_path = path;

        Ok(())
    }
}
