// src/network/packet.rs

use crate::crypto::{encrypt_packet, decrypt_packet};

const PACKET_SIZE: usize = 1280;

pub struct Packet {
    data: [u8; PACKET_SIZE],
    // Other fields
}

impl Packet {
    pub fn new(data: Vec<u8>) -> Result<Self, &'static str> {
        if data.len() != PACKET_SIZE {
            return Err("Data must be exactly 1280 bytes");
        }
        let mut fixed_data = [0u8; PACKET_SIZE];
        fixed_data.copy_from_slice(&data);
        Ok(Packet { data: fixed_data })
    }

    pub fn encrypt(&self, key: &[u8]) -> Vec<u8> {
        encrypt_packet(&self.data, key)
    }

    pub fn decrypt(encrypted_data: &[u8], key: &[u8]) -> Vec<u8> {
        decrypt_packet(encrypted_data, key)
    }
}


