# digital_signature.py
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives import hashes

def sign(data, private_key):
    signature = private_key.sign(data, ec.ECDSA(hashes.SHA384()))
    return signature