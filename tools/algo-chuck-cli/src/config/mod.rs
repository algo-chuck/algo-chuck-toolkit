// Configuration module with separated concerns
mod credentials;
mod encryption;
mod manager;
mod storage;
mod types;

use anyhow::{Context, Result};
use std::path::PathBuf;

// Re-export public API
pub use credentials::CredentialsManager;
pub use manager::ConfigManager;
pub use storage::TokenManager;
pub use types::SchwabConfig;

/// Get the configuration directory for the application
pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
        .join("Algo Chuck CLI");

    std::fs::create_dir_all(&config_dir).with_context(|| {
        format!(
            "Failed to create config directory: {}",
            config_dir.display()
        )
    })?;

    Ok(config_dir)
}
