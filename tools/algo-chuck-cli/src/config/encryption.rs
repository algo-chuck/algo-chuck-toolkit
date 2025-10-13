use anyhow::Result;
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, KeyInit},
};
use rand::Rng;
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub struct EncryptionManager {
    encryption_key: [u8; 32],
}

impl EncryptionManager {
    pub fn new(key_file: &PathBuf) -> Result<Self> {
        let encryption_key = if key_file.exists() {
            Self::load_key(key_file)?
        } else {
            let key = Self::generate_key()?;
            Self::save_key(key_file, &key)?;
            key
        };

        Ok(Self { encryption_key })
    }

    fn generate_key() -> Result<[u8; 32]> {
        let mut key = [0u8; 32];
        rand::rng().fill(&mut key);
        Ok(key)
    }

    fn save_key(key_file: &PathBuf, key: &[u8; 32]) -> Result<()> {
        fs::write(key_file, key)?;

        #[cfg(unix)]
        {
            let mut perms = fs::metadata(key_file)?.permissions();
            perms.set_mode(0o600); // Owner read/write only
            fs::set_permissions(key_file, perms)?;
        }

        Ok(())
    }

    fn load_key(key_file: &PathBuf) -> Result<[u8; 32]> {
        let key_data = fs::read(key_file)?;
        if key_data.len() != 32 {
            return Err(anyhow::anyhow!("Invalid key file length"));
        }

        let mut key = [0u8; 32];
        key.copy_from_slice(&key_data);
        Ok(key)
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new_from_slice(&self.encryption_key)
            .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;
        let mut nonce_bytes = [0u8; 12];
        rand::rng().fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 12 {
            return Err(anyhow::anyhow!("Invalid encrypted data length"));
        }

        let nonce = Nonce::from_slice(&data[..12]);
        let ciphertext = &data[12..];

        let cipher = ChaCha20Poly1305::new_from_slice(&self.encryption_key)
            .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        Ok(plaintext)
    }
}
