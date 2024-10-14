import os
import hashlib
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.primitives.ciphers.aead import ChaCha20Poly1305


class KeyManager:
    def __init__(self):
        self.clientInputKeys = None
        self.serverOutputKeys = None


    def generateClientInputKeys(self):
        # Generate fresh client-input keys (K1, K2, K3) for a single use
        privateKey = ec.generate_private_key(ec.SECP521R1())
        K1 = privateKey.public_key().public_bytes(
            encoding=serialization.Encoding.X962,
            format=serialization.PublicFormat.SubjectPublicKeyInfo
        )
        K2 = privateKey.sign(b"clientInput", ec.ECDSA(hashlib.sha384()))
        K3 = os.urandom(32)
        self.clientInputKeys = (K1, K2, K3)
        return self.clientInputKeys


    def generateServerOutputKeys(self):
        # Generate fresh server-output keys (K4, K5, K6) for a single use
        privateKey = ec.generate_private_key(ec.SECP521R1())
        K4 = privateKey.public_key().public_bytes(
            encoding=serialization.Encoding.X962,
            format=serialization.PublicFormat.SubjectPublicKeyInfo
        )
        K5 = privateKey.sign(b"serverOutput", ec.ECDSA(hashlib.sha384()))
        K6 = os.urandom(32)
        self.serverOutputKeys = (K4, K5, K6)
        return self.serverOutputKeys


    def getClientInputKeys(self):
        if self.clientInputKeys is None:
            self.generateClientInputKeys()
        keys = self.clientInputKeys
        self.clientInputKeys = None  # Ensure single-use
        return keys


    def getServerOutputKeys(self):
        if self.serverOutputKeys is None:
            self.generateServerOutputKeys()
        keys = self.serverOutputKeys
        self.serverOutputKeys = None  # Ensure single-use
        return keys


def main():
    keyManager = KeyManager()
    clientInputKeys = keyManager.getClientInputKeys()
    serverOutputKeys = keyManager.getServerOutputKeys()


    print("Client Input Keys:")
    print(clientInputKeys)


    print("Server Output Keys:")
    print(serverOutputKeys)


if __name__ == "__main__":
    main()
