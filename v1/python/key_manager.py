import hashlib
import hmac
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.primitives.ciphers.aead import ChaCha20Poly1305

class KeyManager:
    def __init__(self):
        # Initialize the two sets of keys
        self.client_input_keys = (K1, K2, K3)
        self.server_output_keys = (K4, K5, K6)

    def get_client_input_keys(self):
        return self.client_input_keys

    def get_server_output_keys(self):
        return self.server_output_keys

    def key_exchange(self, key):
        # Key exchange logic using the provided key
        # Using ECDH with secp521r1 curve
        private_key = ec.generate_private_key(ec.SECP521R1())
        public_key = private_key.public_key()
        shared_secret = private_key.exchange(ec.ECDH(), key)
        return shared_secret

    def digital_signature(self, key, data):
        # Digital signature logic using the provided key and data
        # Using ECDSA with SHA-384 hash
        private_key = ec.generate_private_key(ec.SECP521R1())
        signature = private_key.sign(data, ec.ECDSA(hashes.SHA384()))
        return signature

    def encrypt(self, key, data):
        # Encryption logic using the provided key and data
        # Using ChaCha20-Poly1305
        cipher = ChaCha20Poly1305(key)
        nonce = os.urandom(12)
        ciphertext = cipher.encrypt(nonce, data, None)
        return nonce + ciphertext

    def decrypt(self, key, data):
        # Decryption logic using the provided key and data
        # Using ChaCha20-Poly1305
        cipher = ChaCha20Poly1305(key)
        nonce = data[:12]
        ciphertext = data[12:]
        plaintext = cipher.decrypt(nonce, ciphertext, None)
        return plaintext

    def client_input_key_exchange(self):
        return self.key_exchange(self.client_input_keys[0])

    def client_input_digital_signature(self, data):
        return self.digital_signature(self.client_input_keys[1], data)

    def client_input_encrypt(self, data):
        return self.encrypt(self.client_input_keys[2], data)

    def server_output_key_exchange(self):
        return self.key_exchange(self.server_output_keys[0])

    def server_output_digital_signature(self, data):
        return self.digital_signature(self.server_output_keys[1], data)

    def server_output_decrypt(self, data):
        return self.decrypt(self.server_output_keys[2], data)