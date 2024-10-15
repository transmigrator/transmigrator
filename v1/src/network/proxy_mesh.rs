// network/proxy_mesh.rs

use std::collections::VecDeque;
use rand::seq::SliceRandom;
use crate::network::packet::Packet;
use crate::crypto;

pub struct ProxyMesh {
    proxies: VecDeque<String>,
    chains: Vec<ProxyChain>,
}

pub struct ProxyChain {
    proxies: [String; 3],
}

impl ProxyMesh {
    pub fn new(proxy_list: Vec<String>) -> Self {
        ProxyMesh {
            proxies: VecDeque::from(proxy_list),
            chains: Vec::new(),
        }
    }

    pub fn create_proxy_chain(&mut self) -> Option<ProxyChain> {
        if self.proxies.len() < 3 {
            return None;
        }

        let mut chain_proxies = [String::new(), String::new(), String::new()];
        for i in 0..3 {
            if let Some(proxy) = self.proxies.pop_front() {
                chain_proxies[i] = proxy;
                self.proxies.push_back(chain_proxies[i].clone());
            }
        }

        chain_proxies.shuffle(&mut rand::thread_rng());

        let chain = ProxyChain { proxies: chain_proxies };
        self.chains.push(chain.clone());
        Some(chain)
    }

    pub fn get_next_chain(&mut self) -> Option<&ProxyChain> {
        if self.chains.is_empty() {
            self.create_proxy_chain()?;
        }
        self.chains.rotate_left(1);
        self.chains.last()
    }

    pub fn route_packet(&mut self, packet: Packet) -> Result<Packet, String> {
        let chain = self.get_next_chain().ok_or("No proxy chain available")?;
        
        let key = crypto::generate_key();
        let encrypted_data = crypto::encrypt_packet(packet.data(), &key)?;
        
        let routed_data = self.route_through_chain(&encrypted_data, chain)?;
        
        Ok(Packet::new(routed_data))
    }

    fn route_through_chain(&self, data: &[u8], chain: &ProxyChain) -> Result<Vec<u8>, String> {
        // TODO: Implement actual routing through the proxy chain
        // For now, we'll just return the input data
        Ok(data.to_vec())
    }
}

impl ProxyChain {
    pub fn get_proxies(&self) -> &[String; 3] {
        &self.proxies
    }
}
