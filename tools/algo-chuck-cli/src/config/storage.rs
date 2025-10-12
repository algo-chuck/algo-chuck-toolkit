use anyhow::{Context, Result};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use super::encryption::EncryptionManager;
use super::manager::ConfigManager;
use super::types::EncryptedTokens;

pub struct TokenManager {
    encryption_manager: EncryptionManager,
    tokens_path: PathBuf,
}

impl TokenManager {
    pub fn new(config_manager: &ConfigManager) -> Result<Self> {
        let key_file = config_manager.key_file_path();
        let tokens_path = config_manager.tokens_file_path();

        let encryption_manager = EncryptionManager::new(&key_file)?;

        Ok(Self {
            encryption_manager,
            tokens_path,
        })
    }

    pub fn load_tokens(&self) -> Result<EncryptedTokens> {
        if !self.tokens_path.exists() {
            return Ok(EncryptedTokens::default());
        }

        let encrypted_data = fs::read(&self.tokens_path)?;
        if encrypted_data.is_empty() {
            return Ok(EncryptedTokens::default());
        }

        let decrypted = self.encryption_manager.decrypt(&encrypted_data)?;
        let tokens: EncryptedTokens = serde_json::from_slice(&decrypted)?;
        Ok(tokens)
    }

    pub fn save_tokens(&self, tokens: &EncryptedTokens) -> Result<()> {
        let json_data = serde_json::to_vec(tokens)?;
        let encrypted_data = self.encryption_manager.encrypt(&json_data)?;

        fs::write(&self.tokens_path, encrypted_data)?;

        #[cfg(unix)]
        {
            let mut perms = fs::metadata(&self.tokens_path)?.permissions();
            perms.set_mode(0o600); // Owner read/write only
            fs::set_permissions(&self.tokens_path, perms)?;
        }

        Ok(())
    }

    pub fn save_oauth_tokens(
        &self,
        access_token: &str,
        refresh_token: &str,
        expires_in: u64,
    ) -> Result<()> {
        let now_ms = Utc::now().timestamp_millis();
        let access_expiry_ms = now_ms + (expires_in as i64 * 1000);
        let refresh_expiry_ms = now_ms + (7 * 24 * 60 * 60 * 1000);

        let tokens = EncryptedTokens {
            access_token: Some(access_token.to_string()),
            access_token_expiry: Some(access_expiry_ms),
            refresh_token: Some(refresh_token.to_string()),
            refresh_token_expiry: Some(refresh_expiry_ms),
        };

        self.save_tokens(&tokens)
    }

    pub fn update_access_token(&self, new_access_token: &str, expires_in: u64) -> Result<()> {
        let mut tokens = self.load_tokens()?;
        let now_ms = Utc::now().timestamp_millis();
        let access_expiry_ms = now_ms + (expires_in as i64 * 1000);

        tokens.access_token = Some(new_access_token.to_string());
        tokens.access_token_expiry = Some(access_expiry_ms);

        self.save_tokens(&tokens)
    }

    pub fn get_refresh_token(&self) -> Result<Option<String>> {
        let tokens = self.load_tokens()?;
        Ok(tokens.refresh_token)
    }

    pub fn get_tokens_info(&self) -> Result<EncryptedTokens> {
        self.load_tokens()
    }

    pub fn has_tokens(&self) -> bool {
        self.tokens_path.exists()
            && self
                .tokens_path
                .metadata()
                .map(|m| m.len() > 0)
                .unwrap_or(false)
    }

    pub fn clear_tokens(&self) -> Result<()> {
        if self.tokens_path.exists() {
            std::fs::remove_file(&self.tokens_path).context("Failed to remove tokens file")?;
        }
        Ok(())
    }
}
