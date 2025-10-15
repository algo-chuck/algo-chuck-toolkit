//! Certificate generation functionality for CA and server certificates

use super::{CaInfo, CaManager, CaMetadata, ServerCertMetadata, ServerCertificate};
use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use rcgen::{CertificateParams, DistinguishedName, DnType, KeyPair, Issuer};
use sha2::{Digest, Sha256};
use time;

/// Generate a new Certificate Authority
pub async fn generate_ca(ca_manager: &CaManager) -> Result<CaInfo> {
    println!("🔐 Generating Certificate Authority...");

    // Create proper certificate parameters for CA
    let mut params = CertificateParams::default();

    // Set the subject distinguished name
    let mut distinguished_name = DistinguishedName::new();
    distinguished_name.push(DnType::CommonName, "Algo Chuck Local CA");
    distinguished_name.push(DnType::OrganizationName, "Algo Chuck");
    params.distinguished_name = distinguished_name;

    // Set validity period (10 years for CA)
    let now = Utc::now();
    let expires_at = now + Duration::days(365 * 10);
    params.not_before = time::OffsetDateTime::from_unix_timestamp(now.timestamp()).unwrap();
    params.not_after = time::OffsetDateTime::from_unix_timestamp(expires_at.timestamp()).unwrap();

    // Mark as CA certificate
    params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);

    // Generate key pair and certificate
    let key_pair = KeyPair::generate()?;
    let cert = params.self_signed(&key_pair)?;

    // Get certificate and key in PEM format
    let ca_cert_pem = cert.pem();
    let ca_key_pem = key_pair.serialize_pem();

    // Calculate certificate fingerprint
    let fingerprint = calculate_cert_fingerprint(&ca_cert_pem)?;

    // Save certificate and key to disk
    save_ca_files(ca_manager, &ca_cert_pem, &ca_key_pem).await?;

    // Set validity times
    let now = Utc::now();
    let expires_at = now + Duration::days(365 * 10); // 10 years

    // Create CA metadata
    let ca_metadata = CaMetadata {
        created_at: now,
        expires_at,
        subject: "CN=Algo Chuck Local CA".to_string(),
        serial_number: generate_serial_number().to_string(),
        installed_in_system: false,
        installation_method: None,
        fingerprint,
    };

    let ca_info = CaInfo {
        ca: ca_metadata,
        server: None,
    };

    // Save CA info
    ca_manager.save_ca_info(&ca_info)?;

    println!("✅ Certificate Authority generated successfully");
    println!("   Fingerprint: {}", ca_info.ca.fingerprint);

    Ok(ca_info)
}

/// Generate a server certificate signed by the CA
pub async fn generate_server_certificate(ca_manager: &CaManager) -> Result<ServerCertificate> {
    println!("📜 Generating server certificate...");

    // Load CA certificate and private key
    let ca_cert_pem = std::fs::read_to_string(ca_manager.ca_cert_path())
        .with_context(|| "Failed to read CA certificate")?;
    let ca_key_pem = std::fs::read_to_string(ca_manager.ca_key_path())
        .with_context(|| "Failed to read CA private key")?;

    // Load CA key pair from PEM
    let ca_key_pair = KeyPair::from_pem(&ca_key_pem)
        .with_context(|| "Failed to load CA private key")?;

    // Recreate the CA certificate from stored parameters
    let mut ca_params = CertificateParams::default();
    let mut ca_distinguished_name = DistinguishedName::new();
    ca_distinguished_name.push(DnType::CommonName, "Algo Chuck Local CA");
    ca_distinguished_name.push(DnType::OrganizationName, "Algo Chuck");
    ca_params.distinguished_name = ca_distinguished_name;
    ca_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    
    // Create an Issuer from the CA parameters and key pair
    let ca_issuer = rcgen::Issuer::new(ca_params, ca_key_pair);

    // Create proper certificate parameters for server
    let mut params = CertificateParams::default();

    // Set the subject distinguished name
    let mut distinguished_name = DistinguishedName::new();
    distinguished_name.push(DnType::CommonName, "Algo Chuck HTTPS Server");
    params.distinguished_name = distinguished_name;

    // Add Subject Alternative Names for localhost
    params.subject_alt_names = vec![
        rcgen::SanType::DnsName("localhost".try_into().unwrap()),
        rcgen::SanType::DnsName("127.0.0.1".try_into().unwrap()),
        rcgen::SanType::IpAddress(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))),
    ];

    // Set validity period (1 year for server certificates)
    let now = Utc::now();
    let expires_at = now + Duration::days(365);
    params.not_before = time::OffsetDateTime::from_unix_timestamp(now.timestamp()).unwrap();
    params.not_after = time::OffsetDateTime::from_unix_timestamp(expires_at.timestamp()).unwrap();

    // Generate key pair for server certificate
    let server_key_pair = KeyPair::generate()?;
    
    // Sign the server certificate with the CA
    let cert = params.signed_by(&server_key_pair, &ca_issuer)?;

    // Get certificate and key in PEM format
    let server_cert_pem = cert.pem();
    let server_key_pem = server_key_pair.serialize_pem();

    // Save server certificate and key to disk
    save_server_files(ca_manager, &server_cert_pem, &server_key_pem).await?;

    // Update CA info with server certificate metadata
    let mut ca_info = ca_manager.load_ca_info()?;
    let now = Utc::now();
    let subject_alt_names = vec![
        "localhost".to_string(),
        "127.0.0.1".to_string(),
        "IP:127.0.0.1".to_string(),
    ];
    ca_info.server = Some(ServerCertMetadata {
        last_generated: now,
        expires_at: now + Duration::days(365), // 1 year
        subject_alt_names,
    });
    ca_manager.save_ca_info(&ca_info)?;

    println!("✅ Server certificate generated successfully");

    Ok(ServerCertificate {
        cert_pem: server_cert_pem,
        key_pem: server_key_pem,
        cert_chain: vec![ca_cert_pem],
    })
}

/// Save CA certificate and private key to disk with proper permissions
async fn save_ca_files(ca_manager: &CaManager, cert_pem: &str, key_pem: &str) -> Result<()> {
    // Save certificate (readable by others)
    std::fs::write(ca_manager.ca_cert_path(), cert_pem)
        .with_context(|| "Failed to write CA certificate")?;

    // Save private key (readable only by owner)
    std::fs::write(ca_manager.ca_key_path(), key_pem)
        .with_context(|| "Failed to write CA private key")?;

    // Set proper permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        // CA certificate: 644 (owner read/write, group/others read)
        let cert_perms = std::fs::Permissions::from_mode(0o644);
        std::fs::set_permissions(ca_manager.ca_cert_path(), cert_perms)
            .with_context(|| "Failed to set CA certificate permissions")?;

        // CA private key: 600 (owner read/write only)
        let key_perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(ca_manager.ca_key_path(), key_perms)
            .with_context(|| "Failed to set CA private key permissions")?;
    }

    Ok(())
}

/// Save server certificate and private key to disk with proper permissions
async fn save_server_files(ca_manager: &CaManager, cert_pem: &str, key_pem: &str) -> Result<()> {
    // Save certificate (readable by others)
    std::fs::write(ca_manager.server_cert_path(), cert_pem)
        .with_context(|| "Failed to write server certificate")?;

    // Save private key (readable only by owner)
    std::fs::write(ca_manager.server_key_path(), key_pem)
        .with_context(|| "Failed to write server private key")?;

    // Set proper permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        // Server certificate: 644 (owner read/write, group/others read)
        let cert_perms = std::fs::Permissions::from_mode(0o644);
        std::fs::set_permissions(ca_manager.server_cert_path(), cert_perms)
            .with_context(|| "Failed to set server certificate permissions")?;

        // Server private key: 600 (owner read/write only)
        let key_perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(ca_manager.server_key_path(), key_perms)
            .with_context(|| "Failed to set server private key permissions")?;
    }

    Ok(())
}

/// Generate a random serial number for certificates
fn generate_serial_number() -> u64 {
    use rand::Rng;
    let mut rng = rand::rng();
    rng.random()
}

/// Calculate SHA-256 fingerprint of a certificate
fn calculate_cert_fingerprint(cert_pem: &str) -> Result<String> {
    // Create a simple hash of the PEM content
    let mut hasher = Sha256::new();
    hasher.update(cert_pem.as_bytes());
    let result = hasher.finalize();

    // Format as colon-separated hex
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
