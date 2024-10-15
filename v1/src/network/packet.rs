// network/packet.rs

use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

const PACKET_SIZE: usize = 1280;

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
}
