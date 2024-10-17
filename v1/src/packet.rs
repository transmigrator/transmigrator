pub struct Packet {
    data: Vec<u8>,
    key: Vec<u8>,
}

impl Packet {
    pub fn new(data: Vec<u8>, key: Vec<u8>) -> Self {
        Packet { data, key }
    }

    pub fn encrypt(&mut self) {
        // Implement encryption logic here
        // For example, XOR encryption (replace with a proper encryption algorithm)
        for i in 0..self.data.len() {
            self.data[i] ^= self.key[i % self.key.len()];
        }
    }

    pub fn decrypt(&mut self) {
        // Implement decryption logic here (same as encryption for XOR)
        for i in 0..self.data.len() {
            self.data[i] ^= self.key[i % self.key.len()];
        }
    }
}