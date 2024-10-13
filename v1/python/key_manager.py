import os
import hashlib
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.primitives.ciphers.aead import ChaCha20Poly1305

class KeyManager:
    def __init__(self):
        self.client_input_keys = None
        self.server_output_keys = None

    def generate_client_input_keys(self):
        # Generate new client-input keys (K1, K2, K3)
        # K1: ECDH with secp521r1 curve
        private_key = ec.generate_private_key(ec.SECP521R1())
        K1 = private_key.public_key().public_bytes(
            encoding=serialization.Encoding.X962,
            format=serialization.PublicFormat.SubjectPublicKeyInfo
        )
        # K2: ECDSA with SHA-384 hash
        K2 = private_key.sign(b"client_input", ec.ECDSA(hashlib.sha384()))
        # K3: ChaCha20-Poly1305
        K3 = ChaCha20Poly1305.generate_key()
        self.client_input_keys = (K1, K2, K3)
        return self.client_input_keys

    def generate_server_output_keys(self):
        # Generate new server-output keys (K4, K5, K6)
        # K4: ECDH with secp521r1 curve
        private_key = ec.generate_private_key(ec.SECP521R1())
        K4 = private_key.public_key().public_bytes(
            encoding=serialization.Encoding.X962,
            format=serialization.PublicFormat.SubjectPublicKeyInfo
        )
        # K5: ECDSA with SHA-384 hash
        K5 = private_key.sign(b"server_output", ec.ECDSA(hashlib.sha384()))
        # K6: ChaCha20-Poly1305
        K6 = ChaCha20Poly1305.generate_key()
        self.server_output_keys = (K4, K5, K6)
        return self.server_output_keys

    def get_client_input_keys(self):
        return self.client_input_keys

    def get_server_output_keys(self):
        return self.server_output_keys

def main():
    key_manager = KeyManager()
    client_input_keys = key_manager.generate_client_input_keys()
    server_output_keys = key_manager.generate_server_output_keys()

    print("Client Input Keys:")
    print(client_input_keys)

    print("Server Output Keys:")
    print(server_output_keys)

if __name__ == "__main__":
    main()
