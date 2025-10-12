# Algo Chuck CLI

A professional-grade Rust CLI tool for secure Schwab Developer API OAuth2 authentication with enterprise-level credential management.

## 🚀 Features

### 🔐 **Enterprise Security**

- **Encrypted Credential Storage** - Client secrets stored with ChaCha20Poly1305 encryption
- **Secure File Permissions** - Sensitive files protected with owner-only access (600)
- **No Plain Text Secrets** - Client secrets never stored in configuration files
- **Defense in Depth** - Multiple security layers (encryption + file permissions + separation)

### 🏗️ **Professional Architecture**

- **Modular Design** - Clean, maintainable code following Rust best practices
- **Configuration Hierarchy** - CLI args → env vars → encrypted storage → config files → defaults
- **Cross-Platform** - XDG Base Directory compliance (macOS, Linux, Windows)
- **Error Resilience** - Comprehensive error handling and graceful fallbacks

### 🛠️ **Developer Experience**

- **Command-Based Interface** - Intuitive subcommands for all operations
- **Transparent Encryption** - Security without complexity
- **Clear Status Display** - Know exactly where your credentials are coming from
- **Comprehensive Help** - Built-in documentation for all features

### 🔄 **OAuth2 Flow**

- **Schwab Compliance** - Tailored for Schwab Developer API requirements
- **HTTPS Callback Server** - Local TLS server with self-signed certificates
- **Browser Integration** - Automatic OAuth URL opening
- **Token Management** - Secure storage and automatic refresh handling

## 📋 **Prerequisites**

### Schwab Developer Account Setup

1. Create a [Schwab Developer](https://developer.schwab.com/) account
2. Create a new application
3. Note your **App Key** (client_id) and **Secret** (client_secret)
4. Set your redirect URI to: `https://127.0.0.1:6309/oauth/callback`

## 🚀 **Installation**

### From Source

```bash
git clone https://github.com/algo-chuck/algo-chuck-toolkit
cd tools/algo-chuck-cli
cargo build --release
# Binary will be in target/release/chuck
```

### Add to PATH (Optional)

```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
export PATH="$PATH:/path/to/algo-chuck-cli/target/release"
```

## ⚙️ **Configuration**

### 🔐 **Secure Setup (Recommended)**

```bash
# Set your Schwab credentials securely
chuck config set --client-id "YOUR_APP_KEY"
chuck config set --client-secret "YOUR_APP_SECRET"

# Verify configuration
chuck config show
```

### 🌍 **Environment Variables (Alternative)**

```bash
export SCHWAB_CLIENT_ID="YOUR_APP_KEY"
export SCHWAB_CLIENT_SECRET="YOUR_APP_SECRET"
export SCHWAB_CALLBACK_URL="https://127.0.0.1:6309/oauth/callback"  # optional
```

### 📁 **Configuration Files**

The tool automatically creates platform-appropriate directories:

**macOS:**

```
~/Library/Application Support/Algo Chuck CLI/
├── config.toml         # Non-sensitive settings
├── credentials.enc     # 🔐 Encrypted client secret
├── tokens.enc          # 🔐 Encrypted OAuth2 tokens
└── .algo_chuck_key     # 🔐 Encryption key
```

**Linux:**

```
~/.config/Algo Chuck CLI/       # or $XDG_CONFIG_HOME/Algo Chuck CLI/
~/.local/share/Algo Chuck CLI/  # or $XDG_DATA_HOME/Algo Chuck CLI/
```

**Windows:**

```
%APPDATA%\Algo Chuck CLI\
%LOCALAPPDATA%\Algo Chuck CLI\
```

## 🎯 **Usage**

### 🔑 **Initial Authentication**

```bash
# Start OAuth2 flow
chuck login
```

This will:

1. Open your browser to Schwab's OAuth page
2. Start a local HTTPS server
3. Capture the authorization code
4. Exchange it for access/refresh tokens
5. Store tokens securely with encryption

### 🔄 **Token Refresh**

```bash
# Refresh expired access token
chuck refresh
```

### 📊 **Token Information**

```bash
# View current token status
chuck info
```

### ⚙️ **Configuration Management**

```bash
# View current configuration
chuck config show

# Set credentials securely
chuck config set --client-id "YOUR_APP_KEY"
chuck config set --client-secret "YOUR_SECRET"

# Set custom callback URL
chuck config set --callback-url "https://localhost:8080/callback"

# Reset everything (clears all data)
chuck config reset
```

### 🆘 **Help System**

```bash
# Main help
chuck --help

# Command-specific help
chuck config --help
chuck config set --help
```

## 🔐 **Security Features**

### **Encrypted Credential Storage**

- Client secrets encrypted with **ChaCha20Poly1305**
- Unique encryption key per installation
- Secure key derivation and storage

### **File Security**

- Sensitive files: `600` permissions (owner read/write only)
- Config files: `644` permissions (owner read/write, others read)
- Automatic directory creation with proper permissions

### **Configuration Priority**

1. **CLI Arguments** (highest priority)
2. **Environment Variables**
3. **Encrypted Storage** 🔐
4. **Config Files** (non-sensitive only)
5. **Defaults** (lowest priority)

### **What's Encrypted vs Plain Text**

| Data Type          | Storage           | Security                            |
| ------------------ | ----------------- | ----------------------------------- |
| Client Secret      | `credentials.enc` | 🔐 **Encrypted**                    |
| OAuth2 Tokens      | `tokens.enc`      | 🔐 **Encrypted**                    |
| Encryption Key     | `.algo_chuck_key` | 🔒 **Secure permissions**           |
| Client ID          | `config.toml`     | 📄 **Plain text** (low sensitivity) |
| URLs & Preferences | `config.toml`     | 📄 **Plain text** (non-sensitive)   |

## 🔍 **Troubleshooting**

### **Common Issues**

**"Client ID/Secret not configured"**

```bash
# Check configuration
chuck config show

# Set missing credentials
chuck config set --client-id "YOUR_APP_KEY"
chuck config set --client-secret "YOUR_SECRET"
```

**"Browser doesn't open"**

- Copy the displayed URL manually
- Ensure your system has a default browser configured

**"Callback server errors"**

- Check if port 6309 is available
- Verify firewall settings allow localhost connections
- Try a different callback URL: `--callback-url "https://localhost:8080/callback"`

**"Permission denied" errors**

- Ensure you have write access to config directories
- Check file permissions: `ls -la ~/Library/Application\ Support/Algo\ Chuck\ CLI/`

### **Reset Everything**

```bash
# Nuclear option - clears all data
chuck config reset

# Or manually remove files
rm -rf ~/Library/Application\ Support/Algo\ Chuck\ CLI/  # macOS
rm -rf ~/.config/Algo\ Chuck\ CLI/ ~/.local/share/Algo\ Chuck\ CLI/  # Linux
```

## 🏗️ **Architecture**

### **Modular Design**

```
src/
├── main.rs              # Entry point
├── cli.rs               # CLI definition
├── oauth.rs             # OAuth2 flow
├── server.rs            # HTTPS callback server
├── display.rs           # Display utilities
├── commands/            # Command handlers
│   ├── login.rs
│   ├── refresh.rs
│   ├── info.rs
│   └── config.rs
└── config/              # Configuration system
    ├── types.rs         # Data structures
    ├── manager.rs       # Config file management
    ├── credentials.rs   # Encrypted credentials
    ├── encryption.rs    # Crypto utilities
    └── storage.rs       # Token storage
```

### **Security Architecture**

- **Separation of Concerns** - Sensitive vs non-sensitive data
- **Encrypted Storage** - ChaCha20Poly1305 for all secrets
- **Secure Defaults** - Safe configuration out of the box
- **Privilege Separation** - Minimal file permissions

## 🤝 **Contributing**

### **Development Setup**

```bash
git clone https://github.com/algo-chuck/algo-chuck-toolkit
cd tools/algo-chuck-cli
cargo build
cargo test
cargo run -- --help
```

### **Code Style**

- Follow standard Rust formatting: `cargo fmt`
- Ensure clippy compliance: `cargo clippy`
- Maintain test coverage: `cargo test`

## 📄 **License**

[License information here]

## 🙋 **Support**

For issues, feature requests, or questions:

- Create an issue in the repository
- Check existing documentation
- Review troubleshooting section

---

**⚡ Built with Rust for security, performance, and reliability.**
