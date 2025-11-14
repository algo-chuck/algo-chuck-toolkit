# Distribution Strategy for `chuck` CLI

## Overview

The `chuck` CLI tool is a professional-grade OAuth2 client for Schwab API with enterprise security features. This document outlines distribution strategies for maximum reach and ease of installation.

## Distribution Channels

### 1. Cargo (Rust Package Manager)

**Status**: Ready to implement
**Installation**: `cargo install chuck-cli`

**Setup Required**:

- Create crates.io account and publish
- Consider package name: `chuck-cli` (since `chuck` might be taken)
- Add metadata to Cargo.toml for better discoverability

Note: For non-Rust end-users this is optional. If your primary audience installs prebuilt binaries (GitHub Releases, Homebrew, Scoop), you can skip publishing to crates.io for now. Keeping Cargo metadata up-to-date is still useful for developers and future integration.

**Pros**:

- Zero setup required for Rust developers
- Automatic dependency management
- Built-in security scanning
- Version management

**Cons**:

- Requires Rust toolchain
- Limited to Rust ecosystem

### 2. Homebrew (macOS/Linux)

**Status**: Future implementation
**Installation**: `brew install chuck`

**Setup Required**:

- Create Homebrew formula
- Host binary releases on GitHub
- Maintain formula updates

**Pros**:

- Native package manager for macOS
- No Rust toolchain required
- Automatic updates via `brew upgrade`
- Large user base

**Cons**:

- macOS/Linux only
- Requires maintaining separate build pipeline

### 3. Scoop (Windows)

**Status**: Future implementation  
**Installation**: `scoop install chuck`

**Setup Required**:

- Create Scoop manifest
- Host binary releases
- Submit to community bucket

**Pros**:

- Native Windows package manager
- No dependencies required
- Simple JSON-based manifests

**Cons**:

- Windows only
- Smaller user base than Chocolatey

### 4. GitHub Releases

**Status**: Ready to implement
**Installation**: Download binary from releases page

**Setup Required**:

- Automated cross-compilation CI/CD
- Generate release binaries for major platforms
- Checksums and signatures

**Pros**:

- Universal compatibility
- Direct distribution control
- Works with other package managers as source

**Cons**:

- Manual installation process
- No automatic updates
- Security verification burden on users

### 5. Docker Hub

**Status**: Future consideration
**Installation**: `docker run --rm -it algochuck/chuck`

**Setup Required**:

- Multi-stage Dockerfile
- Automated builds
- Volume mounting for config persistence

**Pros**:

- Platform agnostic
- Sandboxed execution
- Consistent environment

**Cons**:

- Docker overhead
- Complex config/credential management
- Less intuitive for CLI tools

## Recommended Implementation Order

### Phase 1: Immediate (Week 1)

1. **GitHub Releases & Cross-platform CI (priority)**

   - Set up automated cross-compilation CI to build release binaries for Windows, macOS (Intel/ARM), and Linux (x86_64/ARM64).
   - Upload artifacts and checksums to GitHub Releases on tag.
   - This provides the easiest install path for end users who don't have Rust toolchains.

2. **Homebrew Formula / Tap**
   - Create a Homebrew formula referencing the GitHub releases (or bottles) so macOS/Linux users can `brew install chuck`.
   - Maintain a custom tap if you don't want to submit to homebrew-core immediately.

### Phase 3: Windows Support (Month 1)

3. **Scoop Manifest / Chocolatey**
   - Create a Scoop manifest for Windows users and consider Chocolatey packaging.
   - Host Windows exe in GitHub Releases and reference it from manifests.

### Phase 3: Windows Support (Month 2)

4. **Scoop Manifest**
   - Create manifest for Windows users
   - Submit to community bucket

### Phase 4: Advanced Distribution (Future)

5. **Crates.io (optional)**

   - If you later decide to reach Rust developers directly or provide a `cargo install` experience, publish as `chuck-cli`.
   - Maintain Cargo metadata and changelog; `cargo publish --dry-run` is useful for validation.

6. **Other packagers / Docker Images (future)**
   - Snap, Flatpak, Chocolatey, and Docker images are lower priority and can be added later.

## Technical Considerations

### Binary Size Optimization

Current release binary is reasonable for distribution:

```bash
# Check current binary size
ls -lh target/release/chuck
```

### Cross-Compilation Targets

Recommended initial targets:

- `x86_64-unknown-linux-gnu` (Linux Intel/AMD)
- `aarch64-unknown-linux-gnu` (Linux ARM64)
- `x86_64-apple-darwin` (macOS Intel)
- `aarch64-apple-darwin` (macOS Apple Silicon)
- `x86_64-pc-windows-gnu` (Windows)

### Security Considerations

- Code signing for Windows/macOS binaries
- Checksums for all releases
- Supply chain security best practices
- Automated vulnerability scanning

### Update Mechanism

Consider implementing self-update capability:

```rust
// Future: chuck update command
chuck update --check
chuck update --install
```

## Package Metadata for Cargo.toml

```toml
[package]
name = "chuck-cli"
version = "0.1.0"
description = "Professional OAuth2 CLI for Schwab API with enterprise security"
authors = ["Your Name <email@example.com>"]
license = "MIT OR Apache-2.0"
readme = "README.md"
homepage = "https://github.com/yourusername/algo-chuck-toolkit"
repository = "https://github.com/yourusername/algo-chuck-toolkit"
keywords = ["trading", "schwab", "oauth2", "cli", "finance"]
categories = ["command-line-utilities", "api-bindings", "authentication"]
edition = "2021"

[[bin]]
name = "chuck"
path = "src/main.rs"

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

## Marketing Strategy

### Target Audiences

1. **Algorithmic Traders** - Primary audience needing API access
2. **Financial Developers** - Building trading applications
3. **Rust Developers** - Interested in financial tooling
4. **DevOps Engineers** - Automating trading infrastructure

### Distribution Channels

- README with clear installation instructions
- Documentation website with getting started guide
- Trading/finance communities (Reddit, Discord)
- Rust community forums
- Financial technology blogs/articles

### Value Propositions

- **Security First**: Enterprise-grade credential encryption
- **Developer Friendly**: Clean CLI interface with comprehensive help
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Production Ready**: OAuth2 flow with proper error handling
- **Zero Dependencies**: Single binary, no additional setup required
