import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.PrivateKey;
import java.security.PublicKey;
import java.security.SecureRandom;
import java.security.spec.ECGenParameterSpec;

import javax.crypto.Cipher;
import javax.crypto.KeyAgreement;
import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;

public class KeyManager {
    private static final String CURVE_NAME = "secp521r1";
    private static final String HASH_ALGORITHM = "SHA-384";
    private static final String ENCRYPTION_ALGORITHM = "ChaCha20-Poly1305";

    public KeySet generateClientInputKeys() throws Exception {
        KeyPair k1 = generateKeyPair(); // Key exchange
        KeyPair k2 = generateKeyPair(); // Digital signature/auth
        KeyPair k3 = generateKeyPair(); // Encryption
        return new KeySet(k1, k2, k3);
    }

    public KeySet generateServerOutputKeys() throws Exception {
        KeyPair k4 = generateKeyPair(); // Key exchange
        KeyPair k5 = generateKeyPair(); // Digital signature/auth
        KeyPair k6 = generateKeyPair(); // Decryption
        return new KeySet(k4, k5, k6);
    }

    private KeyPair generateKeyPair() throws Exception {
        KeyPairGenerator kpg = KeyPairGenerator.getInstance("EC");
        ECGenParameterSpec ecGenSpec = new ECGenParameterSpec(CURVE_NAME);
        kpg.initialize(ecGenSpec, new SecureRandom());
        return kpg.generateKeyPair();
    }

    public byte[] generateSharedSecret(PrivateKey privateKey, PublicKey publicKey) throws Exception {
        KeyAgreement ka = KeyAgreement.getInstance("ECDH");
        ka.init(privateKey);
        ka.doPhase(publicKey, true);
        return ka.generateSecret();
    }

    public byte[] encrypt(byte[] plaintext, byte[] sharedSecret) throws Exception {
        Cipher cipher = Cipher.getInstance(ENCRYPTION_ALGORITHM);
        SecretKeySpec secretKeySpec = new SecretKeySpec(sharedSecret, ENCRYPTION_ALGORITHM);
        cipher.init(Cipher.ENCRYPT_MODE, secretKeySpec);
        return cipher.doFinal(plaintext);
    }

    public byte[] decrypt(byte[] ciphertext, byte[] sharedSecret) throws Exception {
        Cipher cipher = Cipher.getInstance(ENCRYPTION_ALGORITHM);
        SecretKeySpec secretKeySpec = new SecretKeySpec(sharedSecret, ENCRYPTION_ALGORITHM);
        cipher.init(Cipher.DECRYPT_MODE, secretKeySpec);
        return cipher.doFinal(ciphertext);
    }

    public byte[] sign(byte[] data, PrivateKey privateKey) throws Exception {
        Mac mac = Mac.getInstance(HASH_ALGORITHM);
        mac.init(privateKey);
        return mac.doFinal(data);
    }

    public boolean verify(byte[] data, byte[] signature, PublicKey publicKey) throws Exception {
        Mac mac = Mac.getInstance(HASH_ALGORITHM);
        mac.init(publicKey);
        byte[] expectedSignature = mac.doFinal(data);
        return java.util.Arrays.equals(signature, expectedSignature);
    }

    public static class KeySet {
        private KeyPair k1;
        private KeyPair k2;
        private KeyPair k3;

        public KeySet(KeyPair k1, KeyPair k2, KeyPair k3) {
            this.k1 = k1;
            this.k2 = k2;
            this.k3 = k3;
        }

        public KeyPair getK1() {
            return k1;
        }

        public KeyPair getK2() {
            return k2;
        }

        public KeyPair getK3() {
            return k3;
        }
    }
}