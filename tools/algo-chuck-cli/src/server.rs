use anyhow::{Context, Result};
use chrono::Utc;
use rcgen::generate_simple_self_signed;
use rustls::ServerConfig;
use rustls::ServerConnection;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Duration;
use url::Url;

use crate::ca::CaManager;
use crate::config::SchwabConfig;

const SUCCESS_HTML_TEMPLATE: &str = include_str!("../templates/oauth_success.html");
const NOT_FOUND_HTML_TEMPLATE: &str = include_str!("../templates/oauth_404.html");

/// Create TLS configuration with CA-generated certificate
pub fn create_tls_config() -> Result<ServerConfig> {
    // Install the default crypto provider (ring) for rustls
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Try to use CA-generated certificate first
    let ca_manager = CaManager::new()?;

    if ca_manager.ca_exists() {
        // Use CA-generated server certificate
        match ca_manager.get_or_create_server_cert_sync() {
            Ok(server_cert) => {
                return create_tls_config_from_pem(&server_cert.full_chain, &server_cert.key_pem);
            }
            Err(e) => {
                eprintln!("⚠️  Failed to use CA certificate, falling back to self-signed: {e}");
            }
        }
    }

    // Fallback to self-signed certificate
    create_self_signed_tls_config()
}

/// Create TLS configuration from PEM certificate and key
fn create_tls_config_from_pem(cert_pem: &str, key_pem: &str) -> Result<ServerConfig> {
    // Parse certificate chain
    let cert_chain: Vec<CertificateDer> = rustls_pemfile::certs(&mut cert_pem.as_bytes())
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse certificate chain")?;

    if cert_chain.is_empty() {
        return Err(anyhow::anyhow!("No certificates found in PEM data"));
    }

    // Parse private key
    let mut key_reader = key_pem.as_bytes();
    let private_key = rustls_pemfile::private_key(&mut key_reader)
        .context("Failed to parse private key")?
        .ok_or_else(|| anyhow::anyhow!("No private key found in PEM data"))?;

    // Create TLS configuration
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
        .context("Failed to build TLS config")?;

    Ok(config)
}

/// Create TLS configuration with self-signed certificate (fallback)
fn create_self_signed_tls_config() -> Result<ServerConfig> {
    let subject_alt_names = vec!["localhost".to_string(), "127.0.0.1".to_string()];
    let cert = generate_simple_self_signed(subject_alt_names)
        .context("Failed to generate self-signed certificate")?;

    let cert_der = cert.cert.der().to_vec();
    let private_key_der = cert.signing_key.serialize_der();

    let cert_chain = vec![CertificateDer::from(cert_der)];
    let private_key = PrivateKeyDer::try_from(private_key_der)
        .map_err(|e| anyhow::anyhow!("Failed to convert private key: {}", e))?;

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
        .context("Failed to build TLS config")?;

    Ok(config)
}

/// Create success response with embedded HTML template and dynamic content
fn create_success_response(session_id: &str) -> String {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();

    let html_content = SUCCESS_HTML_TEMPLATE
        .replace("{{SESSION_ID}}", session_id)
        .replace("{{TIMESTAMP}}", &timestamp);

    format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         \r\n{}",
        html_content.len(),
        html_content
    )
}

/// Create 404 not found response with embedded HTML template and dynamic content
fn create_not_found_response(requested_path: &str) -> String {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();

    let html_content = NOT_FOUND_HTML_TEMPLATE
        .replace("{{REQUESTED_PATH}}", requested_path)
        .replace("{{EXPECTED_PATH}}", SchwabConfig::CALLBACK_PATH)
        .replace("{{TIMESTAMP}}", &timestamp);

    format!(
        "HTTP/1.1 404 Not Found\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         \r\n{}",
        html_content.len(),
        html_content
    )
}

/// Start HTTPS callback server and wait for OAuth2 callback
pub fn start_callback_server(config: &SchwabConfig) -> Result<(String, String)> {
    let tls_config = create_tls_config()?;
    let tls_config = Arc::new(tls_config);

    let listener = TcpListener::bind(SchwabConfig::CALLBACK_ADDRESS).context(format!(
        "Failed to bind to {}",
        SchwabConfig::CALLBACK_ADDRESS
    ))?;

    // Set socket to blocking mode (default)
    listener.set_nonblocking(false)?;

    println!(
        "🌐 HTTPS callback server listening on: {}",
        SchwabConfig::CALLBACK_ADDRESS
    );
    println!("📡 Waiting for OAuth2 callback...");
    println!("⏰ Timeout: {} seconds", config.preferences.browser_timeout);

    let timeout_duration = Duration::from_secs(config.preferences.browser_timeout as u64);
    let start_time = std::time::Instant::now();

    // Loop to accept connections with timeout
    loop {
        // Check if we've exceeded the timeout
        if start_time.elapsed() > timeout_duration {
            return Err(anyhow::anyhow!(
                "OAuth2 callback timeout after {} seconds. Please try again or check your browser.",
                config.preferences.browser_timeout
            ));
        }

        // Accept connections with a timeout by using SO_RCVTIMEO
        match listener.accept() {
            Ok((stream, _addr)) => match handle_callback(stream, &tls_config, config) {
                Ok((code, state)) => return Ok((code, state)),
                Err(e) => {
                    eprintln!("Connection handling failed: {}", e);
                    continue;
                }
            },
            Err(e) => {
                // On timeout or would block, check if overall timeout exceeded
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut
                {
                    continue;
                }
                return Err(anyhow::anyhow!("Failed to accept connection: {}", e));
            }
        }
    }
}

/// Handle individual HTTPS callback request
fn handle_callback(
    stream: TcpStream,
    tls_config: &Arc<ServerConfig>,
    _config: &SchwabConfig,
) -> Result<(String, String)> {
    // Perform TLS handshake
    let conn =
        ServerConnection::new(tls_config.clone()).context("Failed to create TLS connection")?;

    // Set socket timeout
    stream.set_read_timeout(Some(Duration::from_secs(10)))?;
    stream.set_write_timeout(Some(Duration::from_secs(10)))?;

    // Create rustls stream (note: this is a bit tricky with blocking I/O)
    // We need to handle the TLS handshake and data transfer manually
    let mut tls_stream = rustls::StreamOwned::new(conn, stream);

    // Read the HTTP request line
    let mut reader = BufReader::new(&mut tls_stream);
    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .context("Failed to read request line")?;

    println!("📨 Received callback: {}", request_line.trim());

    // Parse the request line to extract the path
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(anyhow::anyhow!("Invalid HTTP request format"));
    }

    let path = parts[1];

    // Validate that the request is for the correct OAuth callback path
    if path != SchwabConfig::CALLBACK_PATH
        && !path.starts_with(&format!("{}?", SchwabConfig::CALLBACK_PATH))
    {
        eprintln!(
            "❌ Invalid callback path received: {path} (expected: {})",
            SchwabConfig::CALLBACK_PATH
        );

        // Send 404 response for wrong paths
        let not_found_response = create_not_found_response(path);

        // Write through the reader's inner stream
        let tls_stream = reader.into_inner();
        let _ = tls_stream.write_all(not_found_response.as_bytes());
        let _ = tls_stream.flush();

        return Err(anyhow::anyhow!("Invalid callback path: {}", path));
    }

    let url = format!("http://localhost{}", path);
    let parsed_url = Url::parse(&url).context("Failed to parse callback URL")?;

    let query_pairs: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();

    let code = query_pairs
        .get("code")
        .ok_or_else(|| anyhow::anyhow!("No authorization code found in callback"))?
        .clone();

    let state = query_pairs
        .get("state")
        .ok_or_else(|| anyhow::anyhow!("No state parameter found in callback"))?
        .clone();

    let session_id = query_pairs
        .get("session")
        .unwrap_or(&"unknown".to_string())
        .clone();

    // Send success response
    let response = create_success_response(&session_id);

    // Write through the reader's inner stream
    let tls_stream = reader.into_inner();
    tls_stream
        .write_all(response.as_bytes())
        .context("Failed to write response")?;
    tls_stream.flush().context("Failed to flush response")?;

    println!("✅ Authorization code received successfully!");

    Ok((code, state))
}
