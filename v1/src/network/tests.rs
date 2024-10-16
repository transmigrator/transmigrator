// network/tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use crate::network::proxy_mesh::ProxyMesh;
    use crate::network::packet::Packet;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_add_proxy() {
        let mut proxy_mesh = ProxyMesh::new();
        proxy_mesh.add_proxy("127.0.0.1".to_string(), 8080);
        assert_eq!(proxy_mesh.proxy_pool.len(), 1);
    }

    #[wasm_bindgen_test]
    fn test_create_proxy_chain() {
        let mut proxy_mesh = ProxyMesh::new();
        proxy_mesh.add_proxy("127.0.0.1".to_string(), 8080);
        proxy_mesh.add_proxy("127.0.0.2".to_string(), 8081);
        proxy_mesh.add_proxy("127.0.0.3".to_string(), 8082);
        let chain = proxy_mesh.create_proxy_chain().unwrap();
        assert_eq!(chain.proxies.len(), 3);
    }

    #[wasm_bindgen_test]
    fn test_encrypt_decrypt_packet() {
        let proxy_mesh = ProxyMesh::new();
        let packet = Packet::new(vec![1, 2, 3, 4]);
        let encrypted_data = proxy_mesh.encrypt_packet(&packet);
        let decrypted_packet = proxy_mesh.decrypt_packet(&encrypted_data);
        assert_eq!(decrypted_packet.data(), &vec![1, 2, 3, 4]);
    }

    #[wasm_bindgen_test]
    fn test_tunnel_packet() {
        let mut proxy_mesh = ProxyMesh::new();
        proxy_mesh.add_proxy("127.0.0.1".to_string(), 8080);
        proxy_mesh.add_proxy("127.0.0.2".to_string(), 8081);
        proxy_mesh.add_proxy("127.0.0.3".to_string(), 8082);
        let chain = proxy_mesh.create_proxy_chain().unwrap();
        let packet = Packet::new(vec![1, 2, 3, 4]);
        let tunneled_data = proxy_mesh.tunnel_packet(&packet, &chain);
        assert_eq!(tunneled_data, vec![1, 2, 3, 4]);
    }
}