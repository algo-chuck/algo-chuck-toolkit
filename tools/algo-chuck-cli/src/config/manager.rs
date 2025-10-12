use anyhow::{Context, Result};
use clap::ArgMatches;
use config::{Config, File};
use dirs;
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use super::credentials::CredentialsManager;
use super::types::{ApiConfig, ClientConfig, PreferencesConfig, SchwabConfig};

pub struct ConfigManager {
    config_dir: PathBuf,
    data_dir: PathBuf,
    cache_dir: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .context("Could not determine config directory")?
            .join("com.algochuck.cli");

        let data_dir = dirs::data_dir()
            .context("Could not determine data directory")?
            .join("com.algochuck.cli");

        let cache_dir = dirs::cache_dir()
            .context("Could not determine cache directory")?
            .join("com.algochuck.cli");

        // Create directories if they don't exist
        fs::create_dir_all(&config_dir)?;
        fs::create_dir_all(&data_dir)?;
        fs::create_dir_all(&cache_dir)?;

        Ok(Self {
            config_dir,
            data_dir,
            cache_dir,
        })
    }

    pub fn config_file_path(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    pub fn tokens_file_path(&self) -> PathBuf {
        self.data_dir.join("tokens.enc")
    }

    pub fn key_file_path(&self) -> PathBuf {
        self.config_dir.join(".algo_chuck_key")
    }

    pub fn credentials_file_path(&self) -> PathBuf {
        self.data_dir.join("credentials.enc")
    }

    pub fn load_config(&self, matches: &ArgMatches) -> Result<SchwabConfig> {
        // 1. Start with default configuration
        let mut config = SchwabConfig::default();

        // 2. Load from config file if it exists
        let config_file = self.config_file_path();
        if config_file.exists() {
            let file_config = Config::builder()
                .add_source(File::from(config_file).required(false))
                .build()?;

            if let Ok(api) = file_config.get::<ApiConfig>("api") {
                config.api = api;
            }
            if let Ok(client) = file_config.get::<ClientConfig>("client") {
                config.client = client;
            }
            if let Ok(preferences) = file_config.get::<PreferencesConfig>("preferences") {
                config.preferences = preferences;
            }
        }

        // 2.5. Load encrypted credentials (client_secret from encrypted storage)
        if let Ok(credentials_manager) = CredentialsManager::new(self) {
            if let Ok(Some(encrypted_client_secret)) = credentials_manager.get_client_secret() {
                config.client.client_secret = Some(encrypted_client_secret);
            }
        }

        // 3. Override with environment variables
        self.apply_env_overrides(&mut config)?;

        // 4. Override with CLI arguments
        self.apply_cli_overrides(&mut config, matches)?;

        // 5. Parse callback URL to extract address and path
        config.parse_callback_url()?;

        // 6. Validate required fields
        self.validate_config(&config)?;

        Ok(config)
    }

    fn apply_env_overrides(&self, config: &mut SchwabConfig) -> Result<()> {
        if let Ok(auth_url) = std::env::var("SCHWAB_AUTH_URL") {
            config.api.auth_url = auth_url;
        }
        if let Ok(token_url) = std::env::var("SCHWAB_TOKEN_URL") {
            config.api.token_url = token_url;
        }
        if let Ok(callback_url) = std::env::var("SCHWAB_CALLBACK_URL") {
            config.api.callback_url = callback_url;
        }
        if let Ok(client_id) = std::env::var("SCHWAB_CLIENT_ID") {
            config.client.client_id = Some(client_id);
        }
        if let Ok(client_secret) = std::env::var("SCHWAB_CLIENT_SECRET") {
            config.client.client_secret = Some(client_secret);
        }

        Ok(())
    }

    fn apply_cli_overrides(&self, _config: &mut SchwabConfig, _matches: &ArgMatches) -> Result<()> {
        // Currently no CLI overrides for credentials - they must be set via env vars or config file
        // This function is ready for future CLI arguments if needed

        Ok(())
    }

    fn validate_config(&self, config: &SchwabConfig) -> Result<()> {
        if config.client.client_id.is_none() {
            return Err(anyhow::anyhow!(
                "SCHWAB_CLIENT_ID is required. Set it via environment variable, config file, or --client-id"
            ));
        }
        if config.client.client_secret.is_none() {
            return Err(anyhow::anyhow!(
                "SCHWAB_CLIENT_SECRET is required. Set it via environment variable, config file, or --client-secret"
            ));
        }

        Ok(())
    }

    pub fn save_config(&self, config: &SchwabConfig) -> Result<()> {
        let config_content = toml::to_string_pretty(config)?;
        let config_file = self.config_file_path();

        fs::write(&config_file, config_content)?;

        #[cfg(unix)]
        {
            let mut perms = fs::metadata(&config_file)?.permissions();
            perms.set_mode(0o644); // User read/write, group/other read
            fs::set_permissions(&config_file, perms)?;
        }

        Ok(())
    }
}
