//! System trust store integration for installing/uninstalling CA certificates

use super::CaManager;
use anyhow::{Context, Result, anyhow};
use std::process::Command;

/// Install CA certificate in system trust store
pub async fn install_ca_in_system(ca_manager: &CaManager) -> Result<()> {
    println!("🔧 Installing CA certificate in system trust store...");

    let ca_cert_path = ca_manager.ca_cert_path();
    if !ca_cert_path.exists() {
        return Err(anyhow!(
            "CA certificate not found. Run 'chuck ca generate' first."
        ));
    }

    match std::env::consts::OS {
        "macos" => install_macos_ca(ca_manager).await,
        "windows" => install_windows_ca(ca_manager).await,
        "linux" => install_linux_ca(ca_manager).await,
        os => {
            eprintln!("⚠️  Automatic installation not supported on {}", os);
            print_manual_instructions(ca_manager)?;
            Ok(())
        }
    }
}

/// Remove CA certificate from system trust store
pub async fn uninstall_ca_from_system(ca_manager: &CaManager) -> Result<()> {
    println!("🗑️  Removing CA certificate from system trust store...");

    match std::env::consts::OS {
        "macos" => uninstall_macos_ca(ca_manager).await,
        "windows" => uninstall_windows_ca(ca_manager).await,
        "linux" => uninstall_linux_ca(ca_manager).await,
        os => {
            eprintln!("⚠️  Automatic removal not supported on {}", os);
            print_manual_removal_instructions(ca_manager)?;
            Ok(())
        }
    }
}

/// Install CA certificate on macOS
async fn install_macos_ca(ca_manager: &CaManager) -> Result<()> {
    let ca_cert_path = ca_manager.ca_cert_path();

    println!("📋 Installing CA certificate in macOS keychain...");
    println!("   This requires administrator privileges and will prompt for your password.");

    // Use sudo with security command to add certificate to system keychain with broader trust
    let output = Command::new("sudo")
        .args([
            "security",
            "add-trusted-cert",
            "-d", // Add to admin trust settings
            "-r",
            "trustRoot", // Trust as root CA
            "-k",
            "/Library/Keychains/System.keychain",
            ca_cert_path.to_str().unwrap(),
        ])
        .output()
        .with_context(|| "Failed to execute sudo security command")?;

    if output.status.success() {
        println!("✅ CA certificate installed successfully in macOS keychain");
        update_ca_installation_status(ca_manager, true, "macos-security").await?;
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to install CA certificate: {}", error));
    }

    Ok(())
}

/// Install CA certificate on Windows
async fn install_windows_ca(ca_manager: &CaManager) -> Result<()> {
    let ca_cert_path = ca_manager.ca_cert_path();

    println!("📋 Installing CA certificate in Windows certificate store...");
    println!("   This requires administrator privileges.");

    // Use certlm.exe or PowerShell to install certificate
    let output = Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "Import-Certificate -FilePath '{}' -CertStoreLocation Cert:\\LocalMachine\\Root",
                ca_cert_path.display()
            ),
        ])
        .output()
        .with_context(|| "Failed to execute PowerShell command")?;

    if output.status.success() {
        println!("✅ CA certificate installed successfully in Windows certificate store");
        update_ca_installation_status(ca_manager, true, "windows-powershell").await?;
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to install CA certificate: {}", error));
    }

    Ok(())
}

/// Install CA certificate on Linux
async fn install_linux_ca(ca_manager: &CaManager) -> Result<()> {
    let ca_cert_path = ca_manager.ca_cert_path();

    println!("📋 Installing CA certificate in Linux certificate store...");
    println!("   This requires administrator privileges.");

    // Copy certificate to system CA directory
    let system_ca_path = "/usr/local/share/ca-certificates/algo-chuck-ca.crt";

    let copy_output = Command::new("sudo")
        .args(["cp", ca_cert_path.to_str().unwrap(), system_ca_path])
        .output()
        .with_context(|| "Failed to copy CA certificate")?;

    if !copy_output.status.success() {
        let error = String::from_utf8_lossy(&copy_output.stderr);
        return Err(anyhow!("Failed to copy CA certificate: {}", error));
    }

    // Update CA certificates
    let update_output = Command::new("sudo")
        .args(["update-ca-certificates"])
        .output()
        .with_context(|| "Failed to update CA certificates")?;

    if update_output.status.success() {
        println!("✅ CA certificate installed successfully in Linux certificate store");
        update_ca_installation_status(ca_manager, true, "linux-update-ca-certificates").await?;
    } else {
        let error = String::from_utf8_lossy(&update_output.stderr);
        return Err(anyhow!("Failed to update CA certificates: {}", error));
    }

    Ok(())
}

/// Remove CA certificate from macOS keychain
async fn uninstall_macos_ca(ca_manager: &CaManager) -> Result<()> {
    println!("📋 Removing CA certificate from macOS keychain...");
    println!("   This requires administrator privileges and will prompt for your password.");

    // First, find all certificates with our common name and get their SHA-1 hashes
    let find_output = Command::new("security")
        .args([
            "find-certificate",
            "-a",
            "-Z",
            "-c",
            "Algo Chuck Local CA",
            "/Library/Keychains/System.keychain",
        ])
        .output()
        .with_context(|| "Failed to find certificates")?;

    if find_output.status.success() {
        let output_str = String::from_utf8_lossy(&find_output.stdout);

        // Extract SHA-1 hashes from the output
        let mut hashes = Vec::new();
        for line in output_str.lines() {
            if line.starts_with("SHA-1 hash: ") {
                if let Some(hash) = line.strip_prefix("SHA-1 hash: ") {
                    hashes.push(hash.to_string());
                }
            }
        }

        if hashes.is_empty() {
            println!("ℹ️  No certificates found to remove");
            update_ca_installation_status(ca_manager, false, "").await?;
            return Ok(());
        }

        println!("   Found {} certificate(s) to remove", hashes.len());

        // Delete each certificate by its SHA-1 hash
        let mut removed_count = 0;
        for hash in &hashes {
            let delete_output = Command::new("sudo")
                .args([
                    "security",
                    "delete-certificate",
                    "-Z",
                    hash,
                    "/Library/Keychains/System.keychain",
                ])
                .output()
                .with_context(|| format!("Failed to delete certificate with hash {}", hash))?;

            if delete_output.status.success() {
                removed_count += 1;
            } else {
                let error = String::from_utf8_lossy(&delete_output.stderr);
                eprintln!("⚠️  Warning removing certificate {}: {}", hash, error);
            }
        }

        if removed_count > 0 {
            println!(
                "✅ CA certificate(s) removed successfully from macOS keychain ({} removed)",
                removed_count
            );
        } else {
            eprintln!("⚠️  No certificates were successfully removed");
        }

        update_ca_installation_status(ca_manager, false, "").await?;
    } else {
        let error = String::from_utf8_lossy(&find_output.stderr);
        eprintln!("⚠️  Warning: Failed to find certificates: {}", error);
        // Don't fail here, certificate might not have been installed
        update_ca_installation_status(ca_manager, false, "").await?;
    }

    Ok(())
}

/// Remove CA certificate from Windows certificate store
async fn uninstall_windows_ca(ca_manager: &CaManager) -> Result<()> {
    println!("📋 Removing CA certificate from Windows certificate store...");

    // Use PowerShell to remove certificate
    let output = Command::new("powershell")
        .args([
            "-Command",
            "Get-ChildItem -Path Cert:\\LocalMachine\\Root | Where-Object {$_.Subject -like '*Algo Chuck Local CA*'} | Remove-Item",
        ])
        .output()
        .with_context(|| "Failed to execute PowerShell command")?;

    if output.status.success() {
        println!("✅ CA certificate removed successfully from Windows certificate store");
        update_ca_installation_status(ca_manager, false, "").await?;
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("⚠️  Warning: {}", error);
        // Don't fail here, certificate might not have been installed
        update_ca_installation_status(ca_manager, false, "").await?;
    }

    Ok(())
}

/// Remove CA certificate from Linux certificate store
async fn uninstall_linux_ca(ca_manager: &CaManager) -> Result<()> {
    println!("📋 Removing CA certificate from Linux certificate store...");

    // Remove certificate file
    let remove_output = Command::new("sudo")
        .args([
            "rm",
            "-f",
            "/usr/local/share/ca-certificates/algo-chuck-ca.crt",
        ])
        .output()
        .with_context(|| "Failed to remove CA certificate")?;

    if !remove_output.status.success() {
        let error = String::from_utf8_lossy(&remove_output.stderr);
        eprintln!("⚠️  Warning: {}", error);
    }

    // Update CA certificates
    let update_output = Command::new("sudo")
        .args(["update-ca-certificates"])
        .output()
        .with_context(|| "Failed to update CA certificates")?;

    if update_output.status.success() {
        println!("✅ CA certificate removed successfully from Linux certificate store");
        update_ca_installation_status(ca_manager, false, "").await?;
    } else {
        let error = String::from_utf8_lossy(&update_output.stderr);
        eprintln!("⚠️  Warning: {}", error);
        // Don't fail here, update might have succeeded partially
        update_ca_installation_status(ca_manager, false, "").await?;
    }

    Ok(())
}

/// Update CA installation status in metadata
async fn update_ca_installation_status(
    ca_manager: &CaManager,
    installed: bool,
    method: &str,
) -> Result<()> {
    // Only update if CA files exist, otherwise silently succeed
    if ca_manager.ca_exists() {
        let mut ca_info = ca_manager.load_ca_info()?;
        ca_info.ca.installed_in_system = installed;
        ca_info.ca.installation_method = if installed {
            Some(method.to_string())
        } else {
            None
        };
        ca_manager.save_ca_info(&ca_info)?;
    }
    Ok(())
}

/// Print manual installation instructions for unsupported platforms
fn print_manual_instructions(ca_manager: &CaManager) -> Result<()> {
    let ca_cert_path = ca_manager.ca_cert_path();
    let fingerprint = ca_manager.get_ca_fingerprint()?;

    println!("\n📋 Manual Installation Instructions:");
    println!("   1. Open your system's certificate management tool");
    println!("   2. Import the CA certificate as a trusted root authority:");
    println!("      Certificate: {}", ca_cert_path.display());
    println!("   3. Verify the certificate fingerprint:");
    println!("      SHA-256: {}", fingerprint);
    println!("\n⚠️  Only install this certificate if you trust this application!");

    Ok(())
}

/// Print manual removal instructions for unsupported platforms
fn print_manual_removal_instructions(ca_manager: &CaManager) -> Result<()> {
    let fingerprint = ca_manager
        .get_ca_fingerprint()
        .unwrap_or_else(|_| "Unknown".to_string());

    println!("\n📋 Manual Removal Instructions:");
    println!("   1. Open your system's certificate management tool");
    println!("   2. Find the certificate:");
    println!("      Subject: CN=Algo Chuck Local CA, O=Algo Chuck");
    println!("      Fingerprint: {}", fingerprint);
    println!("   3. Remove/delete the certificate from trusted root authorities");

    Ok(())
}

/// Check if user has necessary privileges for system installation
#[allow(dead_code)]
pub fn has_admin_privileges() -> bool {
    match std::env::consts::OS {
        "macos" | "linux" => {
            // Check if running as root or can sudo
            std::env::var("USER").map(|u| u == "root").unwrap_or(false)
                || Command::new("sudo")
                    .args(["-n", "true"])
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
        }
        "windows" => {
            // Check if running as administrator
            // This is a simplified check - in practice you'd use Windows APIs
            Command::new("net")
                .args(["session"])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        }
        _ => false,
    }
}

/// Prompt user for CA installation with clear explanation
pub fn prompt_ca_installation() -> Result<bool> {
    println!("\n🔐 Certificate Authority Setup");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("To eliminate HTTPS certificate warnings during OAuth login, this tool");
    println!("can install a local Certificate Authority in your system's trust store.");
    println!();
    println!("📋 What this does:");
    println!("   • Creates a local CA certificate for this application only");
    println!("   • Installs it in your system's trusted root certificates");
    println!("   • Enables seamless HTTPS for OAuth callbacks");
    println!("   • Can be easily removed later with 'chuck ca uninstall'");
    println!();
    println!("🔒 Security notes:");
    println!("   • CA is only used for localhost (127.0.0.1) connections");
    println!("   • Private key stays on your machine and is not shared");
    println!("   • Only affects this application's OAuth flow");
    println!("   • Similar to development tools like mkcert");
    println!();
    println!("⚠️  This requires administrator privileges (will prompt for password)");
    println!();

    loop {
        print!("Install Certificate Authority? [Y/n/more info]: ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "" | "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            "?" | "more" | "info" | "more info" => {
                print_detailed_ca_info();
                continue;
            }
            _ => {
                println!("Please enter 'y' for yes, 'n' for no, or '?' for more information.");
                continue;
            }
        }
    }
}

/// Print detailed information about CA installation
fn print_detailed_ca_info() {
    println!("\n📚 Detailed Information:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("The Certificate Authority (CA) approach is a standard technique used by");
    println!("development tools to enable trusted HTTPS on localhost without warnings.");
    println!();
    println!("🔧 Technical details:");
    println!("   • Generates a self-signed root CA certificate");
    println!("   • Installs CA in system trust store (requires admin password)");
    println!("   • Creates server certificates signed by this trusted CA");
    println!("   • Browser recognizes certificates as valid (no warnings)");
    println!();
    println!("🛡️  Security model:");
    println!("   • CA can only issue certificates for localhost/127.0.0.1");
    println!("   • Private key is stored securely with 600 permissions");
    println!("   • CA certificate expires in 10 years");
    println!("   • Server certificates expire in 1 year (auto-renewed)");
    println!();
    println!("🗂️  Files created:");
    println!("   • ~/.config/algo-chuck/ca/ca-cert.pem (CA certificate)");
    println!("   • ~/.config/algo-chuck/ca/ca-key.pem (CA private key)");
    println!("   • ~/.config/algo-chuck/ca/server-*.pem (server certificates)");
    println!();
    println!("🔄 Removal:");
    println!("   • Run 'chuck ca uninstall' to remove from system trust store");
    println!("   • Run 'chuck ca clean' to delete all files");
    println!("   • Completely reversible operation");
    println!();
    println!("📖 Similar tools:");
    println!("   • mkcert (popular development CA tool)");
    println!("   • dotnet dev-certs (ASP.NET Core)");
    println!("   • Many development frameworks use this approach");
    println!();
}
