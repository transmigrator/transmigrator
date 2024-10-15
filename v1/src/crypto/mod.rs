// crypto/mod.rs

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::RngCore;

pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

pub fn encrypt_packet(packet: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(&[0u8; 12]); // In practice, use a unique nonce for each encryption

    cipher.encrypt(nonce, packet)
        .map_err(|e| format!("Encryption failed: {:?}", e))
}

pub fn decrypt_packet(encrypted_packet: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(&[0u8; 12]); // Should match the nonce used for encryption

    cipher.decrypt(nonce, encrypted_packet)
        .map_err(|e| format!("Decryption failed: {:?}", e))
}
