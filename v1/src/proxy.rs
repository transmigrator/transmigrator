use rand::seq::SliceRandom;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct Proxy {
    address: String,
}

#[derive(Clone)]
pub struct ProxyChain {
    proxies: Vec<Proxy>,
}

pub struct ProxyMesh {
    chains: Mutex<Vec<ProxyChain>>,
    proxy_queue: Mutex<Vec<Proxy>>,
}

impl ProxyMesh {
    pub fn new() -> Self {
        ProxyMesh { 
            chains: Mutex::new(Vec::new()), 
            proxy_queue: Mutex::new(Vec::new()) 
        }
    }

    pub fn construct_chains(&self) {
        let proxies = get_proxies();
        let mut rng = rand::thread_rng();
        let mut proxy_queue = self.proxy_queue.lock().unwrap();
        *proxy_queue = proxies.into_iter().map(|p| Proxy { address: p }).collect();

        let mut chains = self.chains.lock().unwrap();
        while proxy_queue.len() >= 3 {
            let mut chain_proxies = proxy_queue.drain(0..3).collect::<Vec<_>>();
            chain_proxies.shuffle(&mut rng);
            chains.push(ProxyChain { proxies: chain_proxies });
        }
    }

    pub fn get_next_chain(&self) -> Option<ProxyChain> {
        let mut proxy_queue = self.proxy_queue.lock().unwrap();
        if proxy_queue.len() < 3 {
            drop(proxy_queue); // Release the lock before calling construct_chains
            self.construct_chains();
            proxy_queue = self.proxy_queue.lock().unwrap();
        }

        let mut chains = self.chains.lock().unwrap();
        if chains.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let chain = chains.remove(0);
        chains.push(chain.clone());
        let mut shuffled_chain = chain.clone();
        shuffled_chain.proxies.shuffle(&mut rng);
        Some(shuffled_chain)
    }

    pub fn send_packet(&mut self, packet: &mut Packet) -> Result<(), Box<dyn std::error::Error>> {
        // Implement the logic to send the packet through the proxy chain
        Ok(())
    }
}

fn get_proxies() -> Vec<String> {
    // This function should fetch the proxies from the utils module
    // Assuming utils module has a function called get_proxies
    crate::utils::get_proxies()
}
