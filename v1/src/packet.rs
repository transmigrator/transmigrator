use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub struct Packet {
    data: Vec<u8>,
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl Packet {
    pub fn new(data: Vec<u8>, key: Vec<u8>) -> Self {
        let iv: Vec<u8> = rand::thread_rng().gen::<[u8; 16]>().to_vec();
        Packet { data, key, iv }
    }

    pub fn encrypt(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let cipher = Aes256Cbc::new_from_slices(&self.key, &self.iv)?;
        self.data = cipher.encrypt_vec(&self.data);
        Ok(())
    }

    pub fn decrypt(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let cipher = Aes256Cbc::new_from_slices(&self.key, &self.iv)?;
        self.data = cipher.decrypt_vec(&self.data)?;
        Ok(())
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn set_key(&mut self, key: Vec<u8>) {
        self.key = key;
    }

    pub fn get_key(&self) -> &Vec<u8> {
        &self.key
    }

    pub fn set_iv(&mut self, iv: Vec<u8>) {
        self.iv = iv;
    }

    pub fn get_iv(&self) -> &Vec<u8> {
        &self.iv
    }

    pub fn ensure_size(&mut self) {
        if self.data.len() != 1280 {
            self.data.resize(1280, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_creation() {
        let data = vec![1, 2, 3];
        let key = vec![0; 32];
        let packet = Packet::new(data.clone(), key.clone());

        assert_eq!(packet.get_data(), &data);
        assert_eq!(packet.get_key(), &key);
        assert_eq!(packet.get_iv().len(), 16);
    }

    #[test]
    fn test_packet_encryption_decryption() {
        let data = vec![1, 2, 3];
        let key = vec![0; 32];
        let mut packet = Packet::new(data.clone(), key);

        packet.encrypt().unwrap();
        assert_ne!(packet.get_data(), &data);

        packet.decrypt().unwrap();
        assert_eq!(packet.get_data(), &data);
    }

    #[test]
    fn test_packet_set_get_data() {
        let mut packet = Packet::new(vec![], vec![0; 32]);
        let data = vec![1, 2, 3];
        packet.set_data(data.clone());

        assert_eq!(packet.get_data(), &data);
    }

    #[test]
    fn test_packet_set_get_key() {
        let mut packet = Packet::new(vec![], vec![0; 32]);
        let key = vec![1; 32];
        packet.set_key(key.clone());

        assert_eq!(packet.get_key(), &key);
    }

    #[test]
    fn test_packet_set_get_iv() {
        let mut packet = Packet::new(vec![], vec![0; 32]);
        let iv = vec![1; 16];
        packet.set_iv(iv.clone());

        assert_eq!(packet.get_iv(), &iv);
    }

    #[test]
    fn test_packet_ensure_size() {
        let mut packet = Packet::new(vec![1, 2, 3], vec![0; 32]);
        packet.ensure_size();

        assert_eq!(packet.get_data().len(), 1280);
        assert_eq!(packet.get_data()[0..3], [1, 2, 3]);
        assert_eq!(packet.get_data()[3..], vec![0; 1277]);
    }
}
