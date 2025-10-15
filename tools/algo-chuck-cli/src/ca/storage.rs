//! Storage utilities for CA certificates and metadata

use std::path::Path;
use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use base64::{Engine as _, engine::general_purpose};

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
    let der_bytes = general_purpose::STANDARD.decode(&cert_data)
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

/// Validate that a certificate file exists and is readable
pub fn validate_certificate_file(cert_path: &Path) -> Result<()> {
    if !cert_path.exists() {
        return Err(anyhow::anyhow!("Certificate file does not exist: {}", cert_path.display()));
    }
    
    let cert_pem = std::fs::read_to_string(cert_path)
        .with_context(|| format!("Failed to read certificate file: {}", cert_path.display()))?;
    
    // Basic validation - check for PEM markers
    if !cert_pem.contains("-----BEGIN CERTIFICATE-----") || 
       !cert_pem.contains("-----END CERTIFICATE-----") {
        return Err(anyhow::anyhow!("Invalid certificate format in: {}", cert_path.display()));
    }
    
    Ok(())
}

/// Validate that a private key file exists and is readable
pub fn validate_private_key_file(key_path: &Path) -> Result<()> {
    if !key_path.exists() {
        return Err(anyhow::anyhow!("Private key file does not exist: {}", key_path.display()));
    }
    
    let key_pem = std::fs::read_to_string(key_path)
        .with_context(|| format!("Failed to read private key file: {}", key_path.display()))?;
    
    // Basic validation - check for PEM markers
    if !key_pem.contains("-----BEGIN") || !key_pem.contains("-----END") {
        return Err(anyhow::anyhow!("Invalid private key format in: {}", key_path.display()));
    }
    
    Ok(())
}

/// Securely delete a file by overwriting it before removal
pub fn secure_delete_file(file_path: &Path) -> Result<()> {
    if !file_path.exists() {
        return Ok(()); // Already deleted
    }
    
    // Get file size
    let metadata = std::fs::metadata(file_path)
        .with_context(|| format!("Failed to get metadata for: {}", file_path.display()))?;
    let file_size = metadata.len() as usize;
    
    if file_size > 0 {
        // Overwrite with random data
        let random_data: Vec<u8> = (0..file_size).map(|_| rand::random::<u8>()).collect();
        std::fs::write(file_path, random_data)
            .with_context(|| format!("Failed to overwrite file: {}", file_path.display()))?;
        
        // Overwrite with zeros
        let zero_data = vec![0u8; file_size];
        std::fs::write(file_path, zero_data)
            .with_context(|| format!("Failed to zero file: {}", file_path.display()))?;
    }
    
    // Finally remove the file
    std::fs::remove_file(file_path)
        .with_context(|| format!("Failed to remove file: {}", file_path.display()))?;
    
    Ok(())
}

/// Create directory with proper permissions for CA storage
pub fn create_secure_directory(dir_path: &Path) -> Result<()> {
    std::fs::create_dir_all(dir_path)
        .with_context(|| format!("Failed to create directory: {}", dir_path.display()))?;
    
    // Set proper permissions on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o700); // rwx------
        std::fs::set_permissions(dir_path, perms)
            .with_context(|| format!("Failed to set directory permissions: {}", dir_path.display()))?;
    }
    
    Ok(())
}

/// Check if a certificate is expiring soon
pub fn is_certificate_expiring_soon(cert_path: &Path, days_threshold: i64) -> Result<bool> {
    // This is a simplified check - in a real implementation you'd parse the certificate
    // and check the actual expiration date
    let metadata = std::fs::metadata(cert_path)
        .with_context(|| format!("Failed to get certificate metadata: {}", cert_path.display()))?;
    
    let created = metadata.created()
        .or_else(|_| metadata.modified())
        .with_context(|| "Failed to get certificate creation time")?;
    
    let created_chrono = chrono::DateTime::<chrono::Utc>::from(created);
    let expiry_estimate = created_chrono + chrono::Duration::days(365); // Assume 1 year validity
    let threshold = chrono::Utc::now() + chrono::Duration::days(days_threshold);
    
    Ok(expiry_estimate < threshold)
}

/// Backup existing certificates before regeneration
pub fn backup_certificates(ca_dir: &Path) -> Result<()> {
    let backup_dir = ca_dir.join("backup");
    std::fs::create_dir_all(&backup_dir)
        .with_context(|| format!("Failed to create backup directory: {}", backup_dir.display()))?;
    
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
            std::fs::copy(&src, &dst)
                .with_context(|| format!("Failed to backup {} to {}", src.display(), dst.display()))?;
        }
    }
    
    println!("📦 Certificates backed up to: {}", backup_dir.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_pem_data() {
        let pem = r#"-----BEGIN CERTIFICATE-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA7SAMPLE/DATA/HERE+
ADDITIONAL/LINE/OF/DATA==
-----END CERTIFICATE-----"#;
        
        let result = extract_pem_data(pem).unwrap();
        assert_eq!(result, "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA7SAMPLE/DATA/HERE+ADDITIONAL/LINE/OF/DATA==");
    }
}