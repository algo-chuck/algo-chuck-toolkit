//! Certificate Authority management commands

use crate::ca::{CaManager, installer};
use crate::server::create_tls_config;
use anyhow::Result;
use clap::ArgMatches;
use std::process::Command;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio_rustls::{TlsAcceptor, server::TlsStream};

const CA_TEST_TEMPLATE: &str = include_str!("../../templates/ca_test.html");

/// Handle CA subcommands
pub async fn handle_ca_command(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("status", _)) => handle_ca_status().await,
        Some(("install", _)) => handle_ca_install().await,
        Some(("uninstall", _)) => handle_ca_uninstall().await,
        Some(("regenerate", _)) => handle_ca_regenerate().await,
        Some(("show", _)) => handle_ca_show().await,
        Some(("clean", sub_matches)) => {
            let uninstall = sub_matches.get_flag("uninstall");
            handle_ca_clean(uninstall).await
        }
        Some(("test", sub_matches)) => {
            let port = sub_matches.get_one::<u16>("port").copied().unwrap_or(8443);
            handle_ca_test_server(port).await
        }
        _ => unreachable!("CA subcommand is required"),
    }
}

/// Show CA status
async fn handle_ca_status() -> Result<()> {
    let ca_manager = CaManager::new()?;
    ca_manager.status()?;
    Ok(())
}

/// Install CA certificate in system trust store
async fn handle_ca_install() -> Result<()> {
    let ca_manager = CaManager::new()?;

    // Check if CA exists
    if !ca_manager.ca_exists() {
        println!("🔐 No Certificate Authority found. Creating new CA...");
        ca_manager.generate_ca().await?;
    }

    // Check if already installed
    if ca_manager.ca_installed_in_system()? {
        println!("✅ CA certificate is already installed in system trust store");
        return Ok(());
    }

    // Prompt user for installation
    if installer::prompt_ca_installation()? {
        ca_manager.install_system_ca().await?;
        println!("🎉 Certificate Authority installed successfully!");
        println!("   Future OAuth logins will not show certificate warnings.");
    } else {
        println!("ℹ️  CA installation cancelled by user");
        println!("   You can run 'chuck ca install' again anytime");
    }

    Ok(())
}

/// Uninstall CA certificate from system trust store
async fn handle_ca_uninstall() -> Result<()> {
    let ca_manager = CaManager::new()?;

    // Check if certificate exists in system keychain (even if local files are gone)
    let cert_in_system = check_cert_in_system_keychain();

    if !ca_manager.ca_exists() && !cert_in_system {
        println!("ℹ️  No Certificate Authority found locally or in system trust store");
        return Ok(());
    }

    if ca_manager.ca_exists() && !ca_manager.ca_installed_in_system()? {
        println!("ℹ️  CA certificate is not installed in system trust store");
        return Ok(());
    }

    if cert_in_system {
        println!("🗑️  Removing Certificate Authority from system trust store...");
        ca_manager.uninstall_system_ca().await?;
        println!("✅ CA certificate removed successfully from system");
    } else {
        println!("ℹ️  No certificate found in system trust store");
    }

    Ok(())
}

/// Regenerate CA certificate
async fn handle_ca_regenerate() -> Result<()> {
    let ca_manager = CaManager::new()?;

    if ca_manager.ca_exists() {
        println!(
            "⚠️  This will regenerate the Certificate Authority and invalidate existing certificates."
        );
        println!("   All server certificates will need to be regenerated.");

        if !confirm_regeneration()? {
            println!("ℹ️  CA regeneration cancelled");
            return Ok(());
        }

        // Backup existing certificates
        println!("📦 Backing up existing certificates...");
        crate::ca::storage::backup_certificates(ca_manager.ca_directory())?;

        // Uninstall from system if installed
        if ca_manager.ca_installed_in_system()? {
            println!("🗑️  Removing old CA from system trust store...");
            ca_manager.uninstall_system_ca().await?;
        }
    }

    // Generate new CA
    println!("🔐 Generating new Certificate Authority...");
    ca_manager.generate_ca().await?;

    // Prompt for installation
    if installer::prompt_ca_installation()? {
        ca_manager.install_system_ca().await?;
        println!("🎉 New Certificate Authority installed successfully!");
    }

    Ok(())
}

/// Show CA certificate for manual installation
async fn handle_ca_show() -> Result<()> {
    let ca_manager = CaManager::new()?;

    if !ca_manager.ca_exists() {
        println!("❌ No Certificate Authority found");
        println!("   Run 'chuck ca install' to create and install a CA");
        return Ok(());
    }

    let ca_cert_path = ca_manager.ca_cert_path();
    let fingerprint = ca_manager.get_ca_fingerprint()?;

    println!("🔐 Certificate Authority Information");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Certificate Path: {}", ca_cert_path.display());
    println!("SHA-256 Fingerprint: {}", fingerprint);
    println!();
    println!("📋 Manual Installation Instructions:");
    println!("   1. Open your system's certificate management tool");
    println!("   2. Import the certificate as a trusted root authority");
    println!("   3. Verify the fingerprint matches the one shown above");
    println!();

    // Show certificate content
    let cert_content = std::fs::read_to_string(&ca_cert_path)?;
    println!("📜 Certificate Content:");
    println!("{}", cert_content);

    Ok(())
}

/// Clean all CA certificates
async fn handle_ca_clean(uninstall: bool) -> Result<()> {
    let ca_manager = CaManager::new()?;

    if !ca_manager.ca_exists() {
        println!("ℹ️  No Certificate Authority found");
        return Ok(());
    }

    println!("🗑️  Cleaning Certificate Authority...");

    if uninstall {
        println!("   This will remove CA certificates from the system trust store");
    }
    println!("   This will delete all CA files and certificates");

    if !confirm_clean()? {
        println!("ℹ️  CA clean cancelled");
        return Ok(());
    }

    // Clean CA files and optionally uninstall
    ca_manager.clean(uninstall).await?;

    if uninstall {
        println!("✅ CA certificate removed from system and all files deleted");
    } else {
        println!("✅ All CA files deleted");
        println!("   Note: CA may still be installed in system trust store");
        println!("   Run 'chuck ca uninstall' to remove it from the system");
    }

    Ok(())
}

/// Confirm CA regeneration with user
fn confirm_regeneration() -> Result<bool> {
    loop {
        print!("Are you sure you want to regenerate the Certificate Authority? [y/N]: ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "y" | "yes" => return Ok(true),
            "" | "n" | "no" => return Ok(false),
            _ => {
                println!("Please enter 'y' for yes or 'n' for no.");
                continue;
            }
        }
    }
}

/// Confirm CA clean with user
fn confirm_clean() -> Result<bool> {
    loop {
        print!("Are you sure you want to delete all CA certificates? [y/N]: ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "y" | "yes" => return Ok(true),
            "" | "n" | "no" => return Ok(false),
            _ => {
                println!("Please enter 'y' for yes or 'n' for no.");
                continue;
            }
        }
    }
}
/// Check if certificate exists in system keychain (without requiring local CA files)
fn check_cert_in_system_keychain() -> bool {
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

/// Start a test HTTPS server to verify certificate functionality
async fn handle_ca_test_server(port: u16) -> Result<()> {
    let ca_manager = CaManager::new()?;

    // Check if CA exists
    if !ca_manager.ca_exists() {
        eprintln!("❌ No Certificate Authority found");
        eprintln!("   Run 'chuck ca install' to create and install a CA");
        return Ok(());
    }

    println!("🔒 HTTPS Certificate Test Server");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // Check CA installation status
    if ca_manager.ca_installed_in_system()? {
        println!("✅ CA Status: Installed in system trust store");
        println!("   🔒 Browser should show 'Secure' with no warnings");
    } else {
        println!("⚠️  CA Status: NOT installed in system trust store");
        println!("   🔒 Browser will show certificate warnings");
        println!("   💡 Run 'chuck ca install' to eliminate warnings");
    }
    println!();

    // Get or create server certificate
    let _server_cert = ca_manager.get_or_create_server_cert().await?;

    println!("🌐 Test Server Starting...");
    println!("   Server will run on: https://127.0.0.1:{}", port);
    println!("   Also accessible at: https://localhost:{}", port);
    println!();
    println!("📋 What to Test:");
    println!("   1. Open https://127.0.0.1:{} in your browser", port);
    println!("   2. Check for lock icon 🔒 in address bar");
    println!("   3. If secure: ✅ Certificate working correctly");
    println!("   4. If warning: ⚠️  Run 'chuck ca install' first");
    println!();
    println!("🛑 Press Ctrl+C to stop the server");
    println!();

    // Start a simple HTTPS server using the existing OAuth server but with a test page
    start_test_server(port).await?;

    Ok(())
}

/// Start a simple test server on the specified port
async fn start_test_server(port: u16) -> Result<()> {
    let tls_config = create_tls_config().await?;
    let acceptor = TlsAcceptor::from(Arc::new(tls_config));
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address).await?;

    println!("🔒 Test server listening on port {} with HTTPS", port);

    loop {
        let (stream, _addr) = listener.accept().await?;
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            match acceptor.accept(stream).await {
                Ok(tls_stream) => {
                    let _ = handle_test_connection_https(tls_stream).await;
                }
                Err(e) => {
                    eprintln!("TLS handshake failed: {}", e);
                }
            }
        });
    }
}

/// Handle a test HTTPS connection
async fn handle_test_connection_https(mut stream: TlsStream<tokio::net::TcpStream>) -> Result<()> {
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).await?;

    let response = create_test_response();
    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

/// Create a simple HTTP response with test content
fn create_test_response() -> String {
    let html_content = CA_TEST_TEMPLATE;

    format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         \r\n{}",
        html_content.len(),
        html_content
    )
}
