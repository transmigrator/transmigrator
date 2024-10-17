use rand::seq::SliceRandom;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct Proxy {
    address: String,
}

pub struct ProxyChain {
    proxies: Vec<Proxy>,
}

pub struct ProxyMesh {
    chains: Vec<ProxyChain>,
    proxy_queue: Vec<Proxy>,
}

impl ProxyMesh {
    pub fn new() -> Self {
        ProxyMesh { chains: Vec::new(), proxy_queue: Vec::new() }
    }

    pub fn construct_chains(&mut self) {
        let proxies = get_proxies();
        let mut rng = rand::thread_rng();
        self.proxy_queue = proxies.into_iter().map(|p| Proxy { address: p }).collect();

        while self.proxy_queue.len() >= 3 {
            let mut chain_proxies = self.proxy_queue.drain(0..3).collect::<Vec<_>>();
            chain_proxies.shuffle(&mut rng);
            self.chains.push(ProxyChain { proxies: chain_proxies });
        }
    }

    pub fn get_next_chain(&mut self) -> Option<ProxyChain> {
        if self.proxy_queue.len() < 3 {
            self.construct_chains();
        }

        if self.chains.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let chain = self.chains.remove(0);
        self.chains.push(chain.clone());
        let mut shuffled_chain = chain.clone();
        shuffled_chain.proxies.shuffle(&mut rng);
        Some(shuffled_chain)
    }
}

fn get_proxies() -> Vec<String> {
    // This function should fetch the proxies from the utils module
    // Assuming utils module has a function called get_proxies
    crate::utils::get_proxies()
}
