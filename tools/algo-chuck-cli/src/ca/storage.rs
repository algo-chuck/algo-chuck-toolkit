//! Storage utilities for CA certificates and metadata

use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose};
use sha2::{Digest, Sha256};
use std::path::Path;

/// Get certificate fingerprint for verification
pub fn get_certificate_fingerprint(cert_path: &Path) -> Result<String> {
    let cert_pem = std::fs::read_to_string(cert_path)
        .with_context(|| format!("Failed to read certificate: {}", cert_path.display()))?;

    calculate_pem_fingerprint(&cert_pem)
}

/// Calculate SHA-256 fingerprint of a PEM certificate
pub fn calculate_pem_fingerprint(cert_pem: &str) -> Result<String> {
    // Extract the certificate data between BEGIN/END markers
    let cert_data = extract_pem_data(cert_pem)?;

    // Decode base64 to get DER bytes
    let der_bytes = general_purpose::STANDARD
        .decode(&cert_data)
        .with_context(|| "Failed to decode certificate base64")?;

    // Calculate SHA-256 hash of DER bytes
    let mut hasher = Sha256::new();
    hasher.update(&der_bytes);
    let result = hasher.finalize();

    // Format as colon-separated hex (standard fingerprint format)
    let hex_string = hex::encode(result);
    let fingerprint = hex_string
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(":");

    Ok(fingerprint.to_uppercase())
}

/// Extract base64 data from PEM certificate
fn extract_pem_data(pem: &str) -> Result<String> {
    let lines: Vec<&str> = pem.lines().collect();
    let mut in_cert = false;
    let mut cert_data = String::new();

    for line in lines {
        if line.starts_with("-----BEGIN CERTIFICATE-----") {
            in_cert = true;
            continue;
        }
        if line.starts_with("-----END CERTIFICATE-----") {
            break;
        }
        if in_cert {
            cert_data.push_str(line.trim());
        }
    }

    if cert_data.is_empty() {
        return Err(anyhow::anyhow!("No certificate data found in PEM"));
    }

    Ok(cert_data)
}

/// Backup existing certificates before regeneration
pub fn backup_certificates(ca_dir: &Path) -> Result<()> {
    let backup_dir = ca_dir.join("backup");
    std::fs::create_dir_all(&backup_dir).with_context(|| {
        format!(
            "Failed to create backup directory: {}",
            backup_dir.display()
        )
    })?;

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");

    let files_to_backup = [
        ("ca-cert.pem", format!("ca-cert_{}.pem", timestamp)),
        ("ca-key.pem", format!("ca-key_{}.pem", timestamp)),
        ("server-cert.pem", format!("server-cert_{}.pem", timestamp)),
        ("server-key.pem", format!("server-key_{}.pem", timestamp)),
        ("ca-info.toml", format!("ca-info_{}.toml", timestamp)),
    ];

    for (original, backup_name) in &files_to_backup {
        let src = ca_dir.join(original);
        let dst = backup_dir.join(backup_name);

        if src.exists() {
            std::fs::copy(&src, &dst).with_context(|| {
                format!("Failed to backup {} to {}", src.display(), dst.display())
            })?;
        }
    }

    println!("📦 Certificates backed up to: {}", backup_dir.display());
    Ok(())
}
