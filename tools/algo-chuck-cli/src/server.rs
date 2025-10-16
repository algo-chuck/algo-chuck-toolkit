use anyhow::{Context, Result};
use chrono::Utc;
use rcgen::generate_simple_self_signed;
use rustls::ServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener as TokioTcpListener, TcpStream as TokioTcpStream};
use tokio::time::timeout;
use tokio_rustls::{TlsAcceptor, server::TlsStream};
use url::Url;

use crate::ca::CaManager;
use crate::config::SchwabConfig;

const SUCCESS_HTML_TEMPLATE: &str = include_str!("../templates/oauth_success.html");
const NOT_FOUND_HTML_TEMPLATE: &str = include_str!("../templates/oauth_404.html");

/// Create TLS configuration with CA-generated certificate
pub async fn create_tls_config() -> Result<ServerConfig> {
    // Install the default crypto provider (ring) for rustls
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Try to use CA-generated certificate first
    let ca_manager = CaManager::new()?;

    if ca_manager.ca_exists() {
        // Use CA-generated server certificate
        match ca_manager.get_or_create_server_cert().await {
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
pub async fn start_callback_server(config: &SchwabConfig) -> Result<(String, String)> {
    let tls_config = create_tls_config().await?;
    let acceptor = TlsAcceptor::from(Arc::new(tls_config));

    let listener = TokioTcpListener::bind(SchwabConfig::CALLBACK_ADDRESS)
        .await
        .context(format!(
            "Failed to bind to {}",
            SchwabConfig::CALLBACK_ADDRESS
        ))?;

    println!(
        "🌐 HTTPS callback server listening on: {}",
        SchwabConfig::CALLBACK_ADDRESS
    );
    println!("📡 Waiting for OAuth2 callback...");
    println!("⏰ Timeout: {} seconds", config.preferences.browser_timeout);

    let timeout_duration = Duration::from_secs(config.preferences.browser_timeout as u64);

    let server_future = async {
        loop {
            let (stream, _addr) = listener
                .accept()
                .await
                .context("Failed to accept connection")?;
            let acceptor = acceptor.clone();
            let config = config.clone();

            match acceptor.accept(stream).await {
                Ok(tls_stream) => {
                    if let Ok((code, state)) = handle_callback(tls_stream, &config).await {
                        return Ok((code, state));
                    }
                }
                Err(e) => {
                    eprintln!("TLS handshake failed: {}", e);
                    continue;
                }
            }
        }
    };

    match timeout(timeout_duration, server_future).await {
        Ok(result) => result,
        Err(_) => Err(anyhow::anyhow!(
            "OAuth2 callback timeout after {} seconds. Please try again or check your browser.",
            config.preferences.browser_timeout
        )),
    }
}

/// Handle individual HTTPS callback request
async fn handle_callback(
    mut stream: TlsStream<TokioTcpStream>,
    _config: &SchwabConfig,
) -> Result<(String, String)> {
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).await?;

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

        stream.write_all(not_found_response.as_bytes()).await?;
        stream.flush().await?;

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

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    println!("✅ Authorization code received successfully!");

    Ok((code, state))
}
