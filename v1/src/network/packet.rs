// network/packet.rs

use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use ring::{aead, rand as ring_rand};

const PACKET_SIZE: usize = 1280;
const KEY_SIZE: usize = 32; // 256 bits
const NONCE_SIZE: usize = 12; // 96 bits

pub struct Packet {
    id: u64,
    data: [u8; PACKET_SIZE],
    timestamp: u64,
}

impl Packet {
    pub fn new(data: &[u8]) -> Self {
        let mut rng = rand::thread_rng();
        let mut packet_data = [0u8; PACKET_SIZE];
        let len = std::cmp::min(data.len(), PACKET_SIZE);
        packet_data[..len].copy_from_slice(&data[..len]);

        Self {
            id: rng.gen(),
            data: packet_data,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn encrypt(&self, key: &[u8; KEY_SIZE]) -> Vec<u8> {
        let nonce = ring_rand::SystemRandom::new().generate::<[u8; NONCE_SIZE]>().unwrap();
        let aad = aead::Aad::empty();

        let sealing_key = aead::UnboundKey::new(&aead::AES_256_GCM, key).unwrap();
        let mut sealing_key = aead::SealingKey::new(sealing_key, &nonce);

        let mut in_out = self.data.to_vec();
        sealing_key.seal_in_place_append_tag(aad, &mut in_out).unwrap();

        [&nonce[..], &in_out[..]].concat()
    }

    pub fn decrypt(encrypted: &[u8], key: &[u8; KEY_SIZE]) -> Result<Self, ring::error::Unspecified> {
        let nonce = aead::Nonce::try_assume_unique_for_key(&encrypted[..NONCE_SIZE])?;
        let mut in_out = encrypted[NONCE_SIZE..].to_vec();

        let opening_key = aead::UnboundKey::new(&aead::AES_256_GCM, key)?;
        let mut opening_key = aead::OpeningKey::new(opening_key, nonce);
        let aad = aead::Aad::empty();

        opening_key.open_in_place(aad, &mut in_out)?;

        let mut data = [0u8; PACKET_SIZE];
        data.copy_from_slice(&in_out[..PACKET_SIZE]);

        Ok(Self {
            id: rand::random(),
            data,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        })
    }
}
