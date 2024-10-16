// network/mod.rs

use std::collections::VecDeque;
use rand::seq::SliceRandom;
use crate::network::packet::Packet;

pub mod proxy_mesh;
pub mod packet;

pub struct Proxy {
    ip: String,
    port: u16,
}

#[derive(Clone)]
pub struct ProxyChain {
    proxies: Vec<Proxy>,
}

pub struct ProxyMesh {
    proxy_pool: VecDeque<Proxy>,
    active_chains: Vec<ProxyChain>,
}

impl Proxy {
    pub fn new(ip: String, port: u16) -> Self {
        Self { ip, port }
    }
}

impl ProxyChain {
    pub fn new(proxies: Vec<Proxy>) -> Self {
        assert_eq!(proxies.len(), 3, "ProxyChain must contain exactly 3 proxies");
        Self { proxies }
    }
}

impl ProxyMesh {
    pub fn new() -> Self {
        Self {
            proxy_pool: VecDeque::new(),
            active_chains: Vec::new(),
        }
    }

    pub fn add_proxy(&mut self, ip: String, port: u16) {
        let proxy = Proxy::new(ip, port);
        self.proxy_pool.push_back(proxy);
    }

    pub fn create_proxy_chain(&mut self) -> Result<ProxyChain, &'static str> {
        if self.proxy_pool.len() < 3 {
            return Err("Not enough proxies to create a chain");
        }

        let mut rng = rand::thread_rng();
        let mut proxies = Vec::new();
        for _ in 0..3 {
            if let Some(proxy) = self.proxy_pool.pop_front() {
                proxies.push(proxy);
            }
        }
        proxies.shuffle(&mut rng);
        let chain = ProxyChain::new(proxies);
        self.active_chains.push(chain.clone());
        Ok(chain)
    }

    pub fn encrypt_packet(&self, packet: &Packet) -> Vec<u8> {
        // Implement packet encryption logic
        packet.data().clone()
    }

    pub fn decrypt_packet(&self, encrypted_data: &[u8]) -> Packet {
        // Implement packet decryption logic
        Packet::new(encrypted_data.to_vec()) // Assuming default constructor for simplicity
    }

    pub fn tunnel_packet(&self, packet: &Packet, chain: &ProxyChain) -> Vec<u8> {
        // Implement SSH-like tunneling logic
        let encrypted_data = self.encrypt_packet(packet);
        // Simulate tunneling through proxies
        for proxy in &chain.proxies {
            // Simulate sending data to proxy
            web_sys::console::log_1(&format!("Tunneling through proxy: {}:{}", proxy.ip, proxy.port).into());
        }
        encrypted_data
    }
}

pub struct Network {
    proxy_mesh: ProxyMesh,
}

impl Network {
    pub fn new() -> Network {
        Network {
            proxy_mesh: ProxyMesh::new(),
        }
    }

    pub fn add_proxy(&mut self, ip: String, port: u16) {
        self.proxy_mesh.add_proxy(ip, port);
    }

    pub fn create_proxy_chain(&mut self) -> Result<JsValue, JsValue> {
        match self.proxy_mesh.create_proxy_chain() {
            Ok(chain) => Ok(JsValue::from_serde(&chain).unwrap()),
            Err(e) => Err(JsValue::from_str(e)),
        }
    }

    pub fn encrypt_packet(&self, data: &[u8]) -> Vec<u8> {
        let packet = match Packet::new(data.to_vec()) {
            Ok(packet) => packet,
            Err(_) => return vec![], // Handle the error appropriately
        };
        self.proxy_mesh.encrypt_packet(&packet)
    }

    pub fn decrypt_packet(&self, encrypted_data: &[u8]) -> Vec<u8> {
        let packet = self.proxy_mesh.decrypt_packet(encrypted_data);
        packet.data().clone()
    }

    pub fn tunnel_packet(&self, data: &[u8], chain: JsValue) -> Vec<u8> {
        let packet = match Packet::new(data.to_vec()) {
            Ok(packet) => packet,
            Err(_) => return vec![], // Handle the error appropriately
        };
        self.proxy_mesh.tunnel_packet(&packet, &chain)
    }
}
