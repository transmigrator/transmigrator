// network/mod.rs

use std::collections::VecDeque;
use rand::seq::SliceRandom;

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
}
