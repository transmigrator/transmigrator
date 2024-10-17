use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;
use reqwest::Error;

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

    pub async fn fetch_proxies(url: &str) -> Result<Vec<String>, Error> {
        let response = reqwest::get(url).await?;
        let proxies = response.text().await?;
        Ok(proxies.lines().map(|line| line.to_string()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_proxies() {
        let url = "https://example.com/proxies.txt";
        let proxies = ProxyMesh::fetch_proxies(url).await.unwrap();
        assert!(!proxies.is_empty());
    }

    #[test]
    fn test_proxy_mesh() {
        let proxies = vec![
            "192.168.1.1:1080".to_string(),
            "192.168.1.2:1080".to_string(),
            "192.168.1.3:1080".to_string(),
        ];

        let mut proxy_mesh = ProxyMesh::new(proxies);

        let proxy_chain = proxy_mesh.get_next_chain();
        assert_eq!(proxy_chain.proxies.len(), 3);
    }
}
