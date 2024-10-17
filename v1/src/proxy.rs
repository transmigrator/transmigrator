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

    pub fn get_next_chain(&mut self) -> Option<ProxyChain> {
        if self.proxies.len() < 3 {
            return None;
        }

        let mut rng = thread_rng();
        let mut selected_proxies = Vec::new();
        for _ in 0..3 {
            if let Some(proxy) = self.proxies.pop_front() {
                selected_proxies.push(proxy);
            }
        }

        selected_proxies.shuffle(&mut rng);
        Some(ProxyChain::new(selected_proxies))
    }

    pub fn add_proxy(&mut self, proxy: String) {
        self.proxies.push_back(proxy);
    }

    pub fn recycle_proxies(&mut self, proxies: Vec<String>) {
        for proxy in proxies {
            self.proxies.push_back(proxy);
        }
    }
}
