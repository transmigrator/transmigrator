use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

pub struct ProxyChain {
    proxies: Vec<String>,
}

impl ProxyChain {
    pub fn new(proxies: Vec<String>) -> Self {
        ProxyChain { proxies }
    }

    pub fn add_proxy(&mut self, proxy: String) {
        self.proxies.push(proxy);
    }

    pub fn get_proxies(&self) -> &Vec<String> {
        &self.proxies
    }
}

pub struct ProxyMesh {
    proxies: VecDeque<String>,
}

impl ProxyMesh {
    pub fn new(proxies: Vec<String>) -> Self {
        ProxyMesh {
            proxies: VecDeque::from(proxies),
        }
    }

    pub fn get_next_chain(&mut self) -> ProxyChain {
        let mut chain_proxies = Vec::new();
        for _ in 0..3 {
            if let Some(proxy) = self.proxies.pop_front() {
                chain_proxies.push(proxy.clone());
                self.proxies.push_back(proxy);
            }
        }
        chain_proxies.shuffle(&mut thread_rng());
        ProxyChain { proxies: chain_proxies }
    }

    pub fn get_proxies(&self) -> Vec<String> {
        self.proxies.iter().cloned().collect()
    }
}
