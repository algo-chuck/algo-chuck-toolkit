//! Local Certificate Authority management for seamless HTTPS OAuth callbacks
//!
//! This module implements a local CA approach similar to `mkcert` to eliminate
//! certificate warnings during OAuth flows with Schwab's HTTPS-required callbacks.

pub mod generator;
pub mod installer;
pub mod storage;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Certificate Authority manager for local HTTPS development
pub struct CaManager {
    ca_dir: PathBuf,
}

/// CA metadata and configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct CaInfo {
    pub ca: CaMetadata,
    pub server: Option<ServerCertMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaMetadata {
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub subject: String,
    pub serial_number: String,
    pub installed_in_system: bool,
    pub installation_method: Option<String>,
    pub fingerprint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerCertMetadata {
    pub last_generated: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub subject_alt_names: Vec<String>,
}

/// Server certificate bundle for HTTPS server
#[derive(Debug)]
pub struct ServerCertificate {
    pub key_pem: String,
    pub full_chain: String,
}

impl CaManager {
    /// Create a new CA manager with the default configuration directory
    pub fn new() -> Result<Self> {
        let config_dir = crate::config::get_config_dir()?;
        let ca_dir = config_dir.join("ca");

        // Create CA directory if it doesn't exist
        std::fs::create_dir_all(&ca_dir)
            .with_context(|| format!("Failed to create CA directory: {}", ca_dir.display()))?;

        Ok(Self { ca_dir })
    }

    /// Check if a CA certificate exists
    pub fn ca_exists(&self) -> bool {
        self.ca_cert_path().exists() && self.ca_key_path().exists()
    }

    /// Check if CA is installed in system trust store
    pub fn ca_installed_in_system(&self) -> Result<bool> {
        if let Ok(info) = self.load_ca_info() {
            Ok(info.ca.installed_in_system)
        } else {
            Ok(false)
        }
    }

    /// Get the CA directory path
    pub fn ca_directory(&self) -> &PathBuf {
        &self.ca_dir
    }

    /// Get CA certificate file path
    pub fn ca_cert_path(&self) -> PathBuf {
        self.ca_dir.join("ca-cert.pem")
    }

    /// Get CA private key file path
    pub fn ca_key_path(&self) -> PathBuf {
        self.ca_dir.join("ca-key.pem")
    }

    /// Get server certificate file path
    pub fn server_cert_path(&self) -> PathBuf {
        self.ca_dir.join("server-cert.pem")
    }

    /// Get server private key file path
    pub fn server_key_path(&self) -> PathBuf {
        self.ca_dir.join("server-key.pem")
    }

    /// Get CA info file path
    pub fn ca_info_path(&self) -> PathBuf {
        self.ca_dir.join("ca-info.toml")
    }

    /// Load CA information from disk
    pub fn load_ca_info(&self) -> Result<CaInfo> {
        let info_path = self.ca_info_path();
        let content = std::fs::read_to_string(&info_path)
            .with_context(|| format!("Failed to read CA info: {}", info_path.display()))?;

        toml::from_str(&content).with_context(|| "Failed to parse CA info")
    }

    /// Save CA information to disk
    pub fn save_ca_info(&self, info: &CaInfo) -> Result<()> {
        let info_path = self.ca_info_path();
        let content =
            toml::to_string_pretty(info).with_context(|| "Failed to serialize CA info")?;

        std::fs::write(&info_path, content)
            .with_context(|| format!("Failed to write CA info: {}", info_path.display()))?;

        Ok(())
    }

    /// Generate a new Certificate Authority
    pub async fn generate_ca(&self) -> Result<CaInfo> {
        generator::generate_ca(self).await
    }

    /// Generate a new Certificate Authority (sync version)
    pub fn generate_ca_sync(&self) -> Result<CaInfo> {
        generator::generate_ca_sync(self)
    }

    /// Install CA certificate in system trust store
    pub async fn install_system_ca(&self) -> Result<()> {
        installer::install_ca_in_system(self).await
    }

    /// Install CA certificate in system trust store (sync version)
    pub fn install_system_ca_sync(&self) -> Result<()> {
        installer::install_ca_in_system_sync(self)
    }

    /// Remove CA certificate from system trust store
    pub async fn uninstall_system_ca(&self) -> Result<()> {
        installer::uninstall_ca_from_system(self).await
    }

    /// Generate or retrieve existing server certificate (async version)
    pub async fn get_or_create_server_cert(&self) -> Result<ServerCertificate> {
        // Check if server cert exists and is valid
        if self.server_cert_path().exists() && self.server_key_path().exists() {
            if let Ok(info) = self.load_ca_info() {
                if let Some(server_meta) = &info.server {
                    // Check if certificate is still valid (not expired and not expiring soon)
                    let now = Utc::now();
                    let expires_soon = server_meta.expires_at - chrono::Duration::days(30);

                    if now < expires_soon {
                        // Certificate is still good, load it
                        return self.load_server_certificate();
                    }
                }
            }
        }

        // Generate new server certificate
        self.generate_server_certificate().await
    }

    /// Generate or retrieve existing server certificate (sync version)
    pub fn get_or_create_server_cert_sync(&self) -> Result<ServerCertificate> {
        // Check if server cert exists and is valid
        if self.server_cert_path().exists() && self.server_key_path().exists() {
            if let Ok(info) = self.load_ca_info() {
                if let Some(server_meta) = &info.server {
                    // Check if certificate is still valid (not expired and not expiring soon)
                    let now = Utc::now();
                    let expires_soon = server_meta.expires_at - chrono::Duration::days(30);

                    if now < expires_soon {
                        // Certificate is still good, load it
                        return self.load_server_certificate();
                    }
                }
            }
        }

        // Generate new server certificate
        self.generate_server_certificate_sync()
    }

    /// Generate a new server certificate signed by the CA (async version)
    pub async fn generate_server_certificate(&self) -> Result<ServerCertificate> {
        generator::generate_server_certificate(self).await
    }

    /// Generate a new server certificate signed by the CA (sync version)
    pub fn generate_server_certificate_sync(&self) -> Result<ServerCertificate> {
        generator::generate_server_certificate_sync(self)
    }

    /// Load existing server certificate from disk
    pub fn load_server_certificate(&self) -> Result<ServerCertificate> {
        let cert_pem = std::fs::read_to_string(self.server_cert_path())
            .with_context(|| "Failed to read server certificate")?;

        let key_pem = std::fs::read_to_string(self.server_key_path())
            .with_context(|| "Failed to read server private key")?;

        let ca_cert_pem = std::fs::read_to_string(self.ca_cert_path())
            .with_context(|| "Failed to read CA certificate")?;
        // Create full certificate chain: server cert + CA cert
        let full_chain = format!("{}\n{}", cert_pem, ca_cert_pem);

        Ok(ServerCertificate {
            key_pem,
            full_chain,
        })
    }

    /// Remove all CA files and optionally uninstall from system
    pub async fn clean(&self, uninstall_from_system: bool) -> Result<()> {
        if uninstall_from_system && self.ca_installed_in_system()? {
            self.uninstall_system_ca().await?;
        }

        // Remove all CA files
        let files_to_remove = [
            self.ca_cert_path(),
            self.ca_key_path(),
            self.server_cert_path(),
            self.server_key_path(),
            self.ca_info_path(),
        ];

        for file_path in &files_to_remove {
            if file_path.exists() {
                std::fs::remove_file(file_path)
                    .with_context(|| format!("Failed to remove: {}", file_path.display()))?;
            }
        }

        // Remove backup directory if it exists
        let backup_dir = self.ca_directory().join("backup");
        if backup_dir.exists() {
            std::fs::remove_dir_all(&backup_dir).with_context(|| {
                format!(
                    "Failed to remove backup directory: {}",
                    backup_dir.display()
                )
            })?;
        }

        Ok(())
    }

    /// Get CA certificate fingerprint for verification
    pub fn get_ca_fingerprint(&self) -> Result<String> {
        storage::get_certificate_fingerprint(&self.ca_cert_path())
    }

    /// Display CA status information
    pub fn status(&self) -> Result<()> {
        if !self.ca_exists() {
            // Check if there's a certificate in the system keychain but no local files
            if self.check_cert_in_system_keychain() {
                println!("🔐 CA Status: Installed in system but no local files");
                println!(
                    "   ⚠️  Certificate is installed in system trust store but local CA files are missing"
                );
                println!("   📝 This means you can't generate new server certificates");
                println!();
                println!("💡 Options:");
                println!(
                    "   • Run 'chuck ca regenerate' to create new CA files (will replace system certificate)"
                );
                println!(
                    "   • Run 'chuck ca uninstall' to remove from system, then 'chuck ca install' for fresh setup"
                );
                println!(
                    "   ℹ️  Note: Private keys cannot be recovered from keychain for security reasons"
                );
                return Ok(());
            }

            println!("🔐 CA Status: Not created");
            println!("   Run 'chuck ca install' to set up local Certificate Authority");
            return Ok(());
        }

        let info = self.load_ca_info()?;
        let fingerprint = self.get_ca_fingerprint()?;

        println!("🔐 CA Status: Created");
        println!("   Subject: {}", info.ca.subject);
        println!(
            "   Created: {}",
            info.ca.created_at.format("%Y-%m-%d %H:%M:%S UTC")
        );
        println!(
            "   Expires: {}",
            info.ca.expires_at.format("%Y-%m-%d %H:%M:%S UTC")
        );
        println!("   Fingerprint: {}", fingerprint);
        println!(
            "   System Install: {}",
            if info.ca.installed_in_system {
                "✅ Yes"
            } else {
                "❌ No"
            }
        );

        if let Some(server) = &info.server {
            println!("\n📜 Server Certificate:");
            println!(
                "   Generated: {}",
                server.last_generated.format("%Y-%m-%d %H:%M:%S UTC")
            );
            println!(
                "   Expires: {}",
                server.expires_at.format("%Y-%m-%d %H:%M:%S UTC")
            );
            println!("   SANs: {}", server.subject_alt_names.join(", "));
        } else {
            println!("\n📜 Server Certificate: Not generated");
        }

        Ok(())
    }

    /// Check if CA certificate exists in system keychain/trust store
    fn check_cert_in_system_keychain(&self) -> bool {
        use std::process::Command;

        match std::env::consts::OS {
            "macos" => {
                let output = Command::new("security")
                    .args([
                        "find-certificate",
                        "-c",
                        "Algo Chuck Local CA",
                        "/Library/Keychains/System.keychain",
                    ])
                    .output();

                match output {
                    Ok(result) => result.status.success(),
                    Err(_) => false,
                }
            }
            "windows" => {
                // Check Windows certificate store
                let output = Command::new("powershell")
                    .args([
                        "-Command",
                        "Get-ChildItem -Path Cert:\\LocalMachine\\Root | Where-Object { $_.Subject -like '*Algo Chuck Local CA*' }",
                    ])
                    .output();

                match output {
                    Ok(result) => result.status.success() && !result.stdout.is_empty(),
                    Err(_) => false,
                }
            }
            "linux" => {
                // Check common Linux certificate locations
                std::path::Path::new("/usr/local/share/ca-certificates/algo-chuck-local-ca.crt")
                    .exists()
                    || std::path::Path::new("/etc/ssl/certs/algo-chuck-local-ca.pem").exists()
            }
            _ => false,
        }
    }
}

impl Default for CaManager {
    fn default() -> Self {
        Self::new().expect("Failed to create CA manager")
    }
}
