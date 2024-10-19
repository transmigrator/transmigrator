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
        // Re-add proxies to the end of the queue to ensure they are reused only after all have been used
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
            drop(success_rate); // Release the lock before calling refresh_proxies
            self.refresh_proxies(new_proxies);
        }
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

    #[test]
    fn test_get_three_proxies() {
        let manager = ProxyManager::new();
        manager.add_proxy("proxy1".to_string());
        manager.add_proxy("proxy2".to_string());
        manager.add_proxy("proxy3".to_string());
        manager.add_proxy("proxy4".to_string());

        let proxies = manager.get_three_proxies().unwrap();
        assert_eq!(proxies.len(), 3);
        assert!(proxies.contains(&"proxy1".to_string()));
        assert!(proxies.contains(&"proxy2".to_string()));
        assert!(proxies.contains(&"proxy3".to_string()));
    }

    #[test]
    fn test_update_success_rate() {
        let manager = ProxyManager::new();
        manager.update_success_rate(true);
        manager.update_success_rate(false);
        let success_rate = manager.success_rate.lock().unwrap();
        assert!(*success_rate < 1.0);
    }

    #[test]
    fn test_check_and_refetch_proxies() {
        let manager = ProxyManager::new();
        manager.update_success_rate(false);
        manager.update_success_rate(false);
        manager.update_success_rate(false);
        manager.update_success_rate(false);
        manager.update_success_rate(false);
        manager.check_and_refetch_proxies(vec!["proxy5".to_string(), "proxy6".to_string()]);

        assert_eq!(manager.get_next_proxy(), Some("proxy5".to_string()));
        assert_eq!(manager.get_next_proxy(), Some("proxy6".to_string()));
    }
}
