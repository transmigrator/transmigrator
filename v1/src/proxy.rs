use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

pub struct ProxyManager {
    proxy_queue: Arc<Mutex<VecDeque<String>>>,
}

impl ProxyManager {
    pub fn new() -> Self {
        ProxyManager {
            proxy_queue: Arc::new(Mutex::new(VecDeque::new())),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_proxy() {
        let manager = ProxyManager::new();
        manager.add_proxy("proxy1".to_string());
        manager.add_proxy("proxy2".to_string());

        assert_eq!(manager.get_next_proxy(), Some("proxy1".to_string()));
        assert_eq!(manager.get_next_proxy(), Some("proxy2".to_string()));
        assert_eq!(manager.get_next_proxy(), None);
    }

    #[test]
    fn test_refresh_proxies() {
        let manager = ProxyManager::new();
        manager.add_proxy("proxy1".to_string());
        manager.add_proxy("proxy2".to_string());

        manager.refresh_proxies(vec!["proxy3".to_string(), "proxy4".to_string()]);

        assert_eq!(manager.get_next_proxy(), Some("proxy3".to_string()));
        assert_eq!(manager.get_next_proxy(), Some("proxy4".to_string()));
        assert_eq!(manager.get_next_proxy(), None);
    }
}
