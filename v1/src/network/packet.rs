// network/packet.rs

use crate::network::proxy_mesh::ProxyMesh;

pub struct Packet {
    data: Vec<u8>,
    // Other fields
}

impl Packet {
    pub fn new(data: Vec<u8>) -> Self {
        Packet { data }
    }

    pub fn encrypt(&self, key: &[u8]) -> Vec<u8> {
        let proxy_mesh = ProxyMesh {};
        proxy_mesh.encrypt_packet(&self.data, key)
    }

    pub fn decrypt(encrypted_data: &[u8], key: &[u8]) -> Vec<u8> {
        let proxy_mesh = ProxyMesh {};
        proxy_mesh.decrypt_packet(encrypted_data, key)
    }
}

    pub fn decrypt(encrypted_data: &[u8], key: &[u8]) -> Vec<u8> {
        let proxy_mesh = ProxyMesh {};
        proxy_mesh.decrypt_packet(encrypted_data, key)
    }
}

