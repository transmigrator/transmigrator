// src/crypto/mod.rs

use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};

const NONCE_SIZE: usize = 12;

pub fn encrypt_packet(packet: &[u8], key: &[u8]) -> Vec<u8> {
    let unbound_key = UnboundKey::new(&AES_256_GCM, key).expect("Invalid key length");
    let nonce = generate_nonce();
    let mut in_out = packet.to_vec();
    let nonce_clone = nonce.clone();
    let nonce = Nonce::assume_unique_for_key(nonce);
    let key = LessSafeKey::new(unbound_key);

    key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .expect("Encryption failed");
    [nonce_clone.as_ref(), in_out.as_slice()].concat()
}

pub fn decrypt_packet(encrypted_packet: &[u8], key: &[u8]) -> Vec<u8> {
    let unbound_key = UnboundKey::new(&AES_256_GCM, key).expect("Invalid key length");
    let (nonce, ciphertext) = encrypted_packet.split_at(NONCE_SIZE);
    let nonce = Nonce::try_assume_unique_for_key(nonce).expect("Invalid nonce length");
    let key = LessSafeKey::new(unbound_key);
    let mut in_out = ciphertext.to_vec();

    key.open_in_place(nonce, Aad::empty(), &mut in_out)
        .expect("Decryption failed");

    in_out
}

fn generate_nonce() -> [u8; NONCE_SIZE] {
    let rng = SystemRandom::new();
    let mut nonce = [0u8; NONCE_SIZE];
    rng.fill(&mut nonce).expect("Failed to generate nonce");
    nonce
}