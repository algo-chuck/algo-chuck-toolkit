use anyhow::{Context, Result};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use super::encryption::EncryptionManager;
use super::manager::ConfigManager;
use super::types::EncryptedCredentials;

pub struct CredentialsManager {
    encryption_manager: EncryptionManager,
    credentials_path: PathBuf,
}

impl CredentialsManager {
    pub fn new(config_manager: &ConfigManager) -> Result<Self> {
        let key_file = config_manager.key_file_path();
        let credentials_path = config_manager.credentials_file_path();

        let encryption_manager = EncryptionManager::new(&key_file)?;

        Ok(Self {
            encryption_manager,
            credentials_path,
        })
    }

    pub fn load_credentials(&self) -> Result<EncryptedCredentials> {
        if !self.credentials_path.exists() {
            return Ok(EncryptedCredentials::default());
        }

        let encrypted_data = fs::read(&self.credentials_path)?;
        if encrypted_data.is_empty() {
            return Ok(EncryptedCredentials::default());
        }

        let decrypted = self.encryption_manager.decrypt(&encrypted_data)?;
        let credentials: EncryptedCredentials = serde_json::from_slice(&decrypted)?;
        Ok(credentials)
    }

    pub fn save_credentials(&self, credentials: &EncryptedCredentials) -> Result<()> {
        let json_data = serde_json::to_vec(credentials)?;
        let encrypted_data = self.encryption_manager.encrypt(&json_data)?;

        fs::write(&self.credentials_path, encrypted_data)?;

        #[cfg(unix)]
        {
            let mut perms = fs::metadata(&self.credentials_path)?.permissions();
            perms.set_mode(0o600); // Owner read/write only
            fs::set_permissions(&self.credentials_path, perms)?;
        }

        Ok(())
    }

    pub fn get_client_secret(&self) -> Result<Option<String>> {
        let credentials = self.load_credentials()?;
        Ok(credentials.client_secret)
    }

    pub fn set_client_secret(&self, secret: &str) -> Result<()> {
        let mut credentials = self.load_credentials()?;
        credentials.client_secret = Some(secret.to_string());
        credentials.created_at = Some(Utc::now().timestamp_millis());
        credentials.version = 1; // Version 1 of the credentials format

        self.save_credentials(&credentials)
    }

    pub fn clear_credentials(&self) -> Result<()> {
        if self.credentials_path.exists() {
            std::fs::remove_file(&self.credentials_path)
                .context("Failed to remove credentials file")?;
        }
        Ok(())
    }

    pub fn has_credentials(&self) -> bool {
        self.credentials_path.exists()
            && self
                .credentials_path
                .metadata()
                .map(|m| m.len() > 0)
                .unwrap_or(false)
    }
}
