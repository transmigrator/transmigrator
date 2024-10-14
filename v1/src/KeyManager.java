import org.bouncycastle.crypto.prng.FortunaGenerator;
import org.bouncycastle.jce.provider.BouncyCastleProvider;

import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.PrivateKey;
import java.security.PublicKey;
import java.security.SecureRandom;
import java.security.Security;
import java.security.spec.ECGenParameterSpec;

import javax.crypto.Cipher;
import javax.crypto.KeyAgreement;
import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;

public class KeyManager {
    static {
        Security.addProvider(new BouncyCastleProvider());
    }

    private static final String CURVE_NAME = "secp521r1";
    private static final String HASH_ALGORITHM = "SHA-384";
    private static final String ENCRYPTION_ALGORITHM = "ChaCha20-Poly1305";

    public KeySet generateClientInputKeys() throws Exception {
        try {
            KeyPair k1 = generateKeyPair(); // Key exchange
            KeyPair k2 = generateKeyPair(); // Digital signature/auth
            KeyPair k3 = generateKeyPair(); // Encryption
            return new KeySet(k1, k2, k3);
        } catch (Exception e) {
            // Handle the exception and throw a meaningful error message
            throw new Exception("Failed to generate client input keys: " + e.getMessage());
        }
    }

    public KeySet generateServerOutputKeys() throws Exception {
        try {
            KeyPair k4 = generateKeyPair(); // Key exchange
            KeyPair k5 = generateKeyPair(); // Digital signature/auth
            KeyPair k6 = generateKeyPair(); // Decryption
            return new KeySet(k4, k5, k6);
        } catch (Exception e) {
            // Handle the exception and throw a meaningful error message
            throw new Exception("Failed to generate server output keys: " + e.getMessage());
        }
    }

    private KeyPair generateKeyPair() throws Exception {
        try {
            // Create a FortunaGenerator instance
            FortunaGenerator fortunaGenerator = new FortunaGenerator();

            // Add entropy sources (e.g., system time, user input)
            fortunaGenerator.addEntropy(System.currentTimeMillis());
            fortunaGenerator.addEntropy(System.identityHashCode(this));

            // Create a SecureRandom instance using the FortunaGenerator
            SecureRandom secureRandom = new SecureRandom() {
                @Override
                public void nextBytes(byte[] bytes) {
                    fortunaGenerator.nextBytes(bytes);
                }
            };

            // Use the SecureRandom instance to generate the key pair
            KeyPairGenerator kpg = KeyPairGenerator.getInstance("EC", new BouncyCastleProvider());
            ECGenParameterSpec ecGenSpec = new ECGenParameterSpec(CURVE_NAME);
            kpg.initialize(ecGenSpec, secureRandom);
            return kpg.generateKeyPair();
        } catch (Exception e) {
            // Handle the exception and throw a meaningful error message
            throw new Exception("Failed to generate key pair: " + e.getMessage());
        }
    }

    public byte[] generateSharedSecret(PrivateKey privateKey, PublicKey publicKey) throws Exception {
        try {
            KeyAgreement ka = KeyAgreement.getInstance("ECDH", new BouncyCastleProvider());
            ka.init(privateKey);
            ka.doPhase(publicKey, true);
            return ka.generateSecret();
        } catch (Exception e) {
            // Handle the exception and throw a meaningful error message
            throw new Exception("Failed to generate shared secret: " + e.getMessage());
        }
    }

    public byte[] encrypt(byte[] plaintext, byte[] sharedSecret) throws Exception {
        try {
            Cipher cipher = Cipher.getInstance(ENCRYPTION_ALGORITHM, new BouncyCastleProvider());
            SecretKeySpec secretKeySpec = new SecretKeySpec(sharedSecret, ENCRYPTION_ALGORITHM);
            cipher.init(Cipher.ENCRYPT_MODE, secretKeySpec);
            return cipher.doFinal(plaintext);
        } catch (Exception e) {
            // Handle the exception and throw a meaningful error message
            throw new Exception("Failed to encrypt data: " + e.getMessage());
        }
    }

    public byte[] decrypt(byte[] ciphertext, byte[] sharedSecret) throws Exception {
        try {
            Cipher cipher = Cipher.getInstance(ENCRYPTION_ALGORITHM, new BouncyCastleProvider());
            SecretKeySpec secretKeySpec = new SecretKeySpec(sharedSecret, ENCRYPTION_ALGORITHM);
            cipher.init(Cipher.DECRYPT_MODE, secretKeySpec);
            return cipher.doFinal(ciphertext);
        } catch (Exception e) {
            // Handle the exception and throw a meaningful error message
            throw new Exception("Failed to decrypt data: " + e.getMessage());
        }
    }

    public byte[] sign(byte[] data, PrivateKey privateKey) throws Exception {
        try {
            Mac mac = Mac.getInstance(HASH_ALGORITHM, new BouncyCastleProvider());
            mac.init(privateKey);
            return mac.doFinal(data);
        } catch (Exception e) {
            // Handle the exception and throw a meaningful error message
            throw new Exception("Failed to sign data: " + e.getMessage());
        }
    }

    public boolean verify(byte[] data, byte[] signature, PublicKey publicKey) throws Exception {
        try {
            Mac mac = Mac.getInstance(HASH_ALGORITHM, new BouncyCastleProvider());
            mac.init(publicKey);
            byte[] expectedSignature = mac.doFinal(data);
            return java.util.Arrays.equals(signature, expectedSignature);
        } catch (Exception e) {
            // Handle the exception and throw a meaningful error message
            throw new Exception("Failed to verify signature: " + e.getMessage());
        }
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
