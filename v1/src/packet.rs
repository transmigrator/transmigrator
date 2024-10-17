use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use std::iter;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub struct Packet {
    data: Vec<u8>,
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl Packet {
    pub fn new(data: Vec<u8>, key: Vec<u8>) -> Self {
        let iv: Vec<u8> = rand::thread_rng().gen_iter().take(16).collect();
        Packet { data, key, iv }
    }

    pub fn encrypt(&mut self) {
        let cipher = Aes256Cbc::new_var(&self.key, &self.iv).unwrap();
        let ciphertext = cipher.encrypt_vec(&self.data);
        self.data = ciphertext;
    }

    pub fn decrypt(&mut self) {
        let cipher = Aes256Cbc::new_var(&self.key, &self.iv).unwrap();
        let decrypted_data = cipher.decrypt_vec(&self.data).unwrap();
        self.data = decrypted_data;
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
}
