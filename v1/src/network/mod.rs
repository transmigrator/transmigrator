// network/mod.rs

use std::collections::VecDeque;
use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;

pub mod proxy_mesh;
pub mod packet;
use packet::Packet;

pub struct Proxy {
    ip: String,
    port: u16,
}

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

    pub fn add_proxy(&mut self, proxy: Proxy) {
        self.proxy_pool.push_back(proxy);
    }

    pub fn create_proxy_chain(&mut self) -> Option<ProxyChain> {
        if self.proxy_pool.len() < 3 {
            return None;
        }

        let mut proxies = Vec::new();
        for _ in 0..3 {
            if let Some(proxy) = self.proxy_pool.pop_front() {
                proxies.push(proxy);
            }
        }

        // Randomly permute the proxies
        proxies.shuffle(&mut rand::thread_rng());

        let chain = ProxyChain::new(proxies);
        self.active_chains.push(chain.clone());
        Some(chain)
    }

    pub fn recycle_chain(&mut self, chain: ProxyChain) {
        for proxy in chain.proxies {
            self.proxy_pool.push_back(proxy);
        }
        // Remove the chain from active_chains
        self.active_chains.retain(|c| c.proxies != chain.proxies);
    }

    pub fn route_packet(&mut self, packet: Packet) -> Result<(), String> {
        let chain = self.create_proxy_chain().ok_or("No available proxy chain")?;
        
        // Here we would implement the actual routing logic
        // For now, let's just print some debug information
        println!("Routing packet {} through chain:", packet.get_id());
        for (i, proxy) in chain.proxies.iter().enumerate() {
            println!("  Hop {}: {}:{}", i+1, proxy.ip, proxy.port);
        }

        // After routing, recycle the chain
        self.recycle_chain(chain);

        Ok(())
    }
}

#[wasm_bindgen]
pub struct Network {
    proxy_mesh: ProxyMesh,
}

#[wasm_bindgen]
impl Network {
    #[wasm_bindgen(constructor)]
    pub fn new(proxy_list: Vec<String>) -> Network {
        Network {
            proxy_mesh: ProxyMesh::new(),
        }
    }

    pub fn create_proxy_chain(&mut self) -> Vec<String> {
        self.proxy_mesh.create_proxy_chain()
    }

    pub fn encrypt_packet(&self, packet: &[u8]) -> Vec<u8> {
        self.proxy_mesh.encrypt_packet(packet)
    }

    pub fn decrypt_packet(&self, packet: &[u8]) -> Vec<u8> {
        self.proxy_mesh.decrypt_packet(packet)
    }

    pub fn tunnel_packet(&self, packet: &[u8], chain: Vec<String>) -> Vec<u8> {
        self.proxy_mesh.tunnel_packet(packet, chain)
    }

    pub fn doh_query(&self, domain: &str) -> Result<String, JsValue> {
        self.proxy_mesh.doh_query(domain)
    }
}
