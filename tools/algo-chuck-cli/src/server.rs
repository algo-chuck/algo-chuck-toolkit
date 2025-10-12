use anyhow::{Context, Result};
use rcgen::generate_simple_self_signed;
use rustls::{Certificate, PrivateKey, ServerConfig};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener as TokioTcpListener, TcpStream as TokioTcpStream};
use tokio::time::timeout;
use tokio_rustls::{TlsAcceptor, server::TlsStream};
use url::Url;

use crate::config::SchwabConfig;

/// Create TLS configuration with self-signed certificate
pub fn create_tls_config() -> Result<ServerConfig> {
    let subject_alt_names = vec!["localhost".to_string(), "127.0.0.1".to_string()];
    let cert = generate_simple_self_signed(subject_alt_names)
        .context("Failed to generate self-signed certificate")?;

    let cert_der = cert
        .serialize_der()
        .context("Failed to serialize certificate")?;
    let private_key_der = cert.serialize_private_key_der();

    let cert_chain = vec![Certificate(cert_der)];
    let private_key = PrivateKey(private_key_der);

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
        .context("Failed to build TLS config")?;

    Ok(config)
}

/// Start HTTPS callback server and wait for OAuth2 callback
pub async fn start_callback_server(config: &SchwabConfig) -> Result<(String, String)> {
    let tls_config = create_tls_config()?;
    let acceptor = TlsAcceptor::from(Arc::new(tls_config));

    let listener = TokioTcpListener::bind(&config.api.callback_address)
        .await
        .context(format!("Failed to bind to {}", config.api.callback_address))?;

    println!(
        "🌐 HTTPS callback server listening on: {}",
        config.api.callback_address
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

    // Send success response
    let response = "HTTP/1.1 200 OK\r\n\
                   Content-Type: text/html\r\n\
                   Content-Length: 180\r\n\
                   \r\n\
                   <html><body><h1>Authentication Successful!</h1>\
                   <p>You have successfully authenticated with Schwab.</p>\
                   <p>You can now close this browser window and return to the terminal.</p></body></html>";

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    println!("✅ Authorization code received successfully!");

    Ok((code, state))
}
