use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use rand::seq::SliceRandom;

pub struct ProxyManager {
    proxy_queue: Arc<Mutex<VecDeque<String>>>,
    success_rate: Arc<Mutex<f32>>,
}

impl ProxyManager {
    pub fn new() -> Self {
        ProxyManager {
            proxy_queue: Arc::new(Mutex::new(VecDeque::new())),
            success_rate: Arc::new(Mutex::new(1.0)),
        }
    }

    pub fn add_proxy(&self, proxy: String) {
        let mut proxy_queue = self.proxy_queue.lock().unwrap();
        proxy_queue.push_back(proxy);
    }

    pub fn get_next_proxy(&self) -> Option<String> {
        let mut proxy_queue = self.proxy_queue.lock().unwrap();
        proxy_queue.pop_front()
    }

    pub fn refresh_proxies(&self, new_proxies: Vec<String>) {
        let mut proxy_queue = self.proxy_queue.lock().unwrap();
        *proxy_queue = VecDeque::from(new_proxies);
    }

    pub fn get_three_proxies(&self) -> Option<Vec<String>> {
        let mut proxy_queue = self.proxy_queue.lock().unwrap();
        if proxy_queue.len() < 3 {
            return None;
        }
        let mut proxies = vec![];
        for _ in 0..3 {
            if let Some(proxy) = proxy_queue.pop_front() {
                proxies.push(proxy);
            }
        }
        proxies.shuffle(&mut rand::thread_rng());
        for proxy in &proxies {
            proxy_queue.push_back(proxy.clone());
        }
        Some(proxies)
    }

    pub fn update_success_rate(&self, success: bool) {
        let mut success_rate = self.success_rate.lock().unwrap();
        if success {
            *success_rate = (*success_rate * 0.9) + 0.1;
        } else {
            *success_rate = (*success_rate * 0.9);
        }
    }

    pub fn check_and_refetch_proxies(&self, new_proxies: Vec<String>) {
        let success_rate = self.success_rate.lock().unwrap();
        if *success_rate < 0.5 {
            drop(success_rate);
            self.refresh_proxies(new_proxies);
        }
    }

    pub fn create_proxy_chain(&self) -> Option<ProxyChain> {
        if let Some(proxies) = self.get_three_proxies() {
            Some(ProxyChain::new(proxies))
        } else {
            None
        }
    }

    pub fn create_proxy_mesh(&self, packet_count: usize) -> ProxyMesh {
        let mut chains = vec![];
        for _ in 0..packet_count {
            if let Some(chain) = self.create_proxy_chain() {
                chains.push(chain);
            }
        }
        ProxyMesh::new(chains)
    }
}

pub struct ProxyChain {
    proxies: Vec<String>,
}

impl ProxyChain {
    pub fn new(proxies: Vec<String>) -> Self {
        ProxyChain { proxies }
    }

    pub fn get_proxies(&self) -> &Vec<String> {
        &self.proxies
    }
}

pub struct ProxyMesh {
    chains: Vec<ProxyChain>,
}

impl ProxyMesh {
    pub fn new(chains: Vec<ProxyChain>) -> Self {
        ProxyMesh { chains }
    }

    pub fn get_chains(&self) -> &Vec<ProxyChain> {
        &self.chains
    }
}
