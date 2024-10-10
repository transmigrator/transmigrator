import java.security.*;
import java.security.spec.*;
import javax.crypto.*;
import javax.crypto.spec.*;
import java.util.*;

public class PacketIntegrityVerifier {
    private ECPublicKey publicKey;
    private ECPrivateKey privateKey;

    public PacketIntegrityVerifier(ECPublicKey publicKey, ECPrivateKey privateKey) {
        this.publicKey = publicKey;
        this.privateKey = privateKey;
    }

    public boolean verifyPacketIntegrity(byte[] packetData, byte[] signature) throws Exception {
        // Calculate SHA-384 hash of the packet data
        MessageDigest sha384 = MessageDigest.getInstance("SHA-384");
        byte[] hash = sha384.digest(packetData);

        // Verify the signature
        Signature ecdsa = Signature.getInstance("SHA384withECDSA");
        ecdsa.initVerify(publicKey);
        ecdsa.update(hash);
        return ecdsa.verify(signature);
    }

    public byte[] signPacket(byte[] packetData) throws Exception {
        // Calculate SHA-384 hash of the packet data
        MessageDigest sha384 = MessageDigest.getInstance("SHA-384");
        byte[] hash = sha384.digest(packetData);

        // Sign the hash using ECDSA
        Signature ecdsa = Signature.getInstance("SHA384withECDSA");
        ecdsa.initSign(privateKey);
        ecdsa.update(hash);
        return ecdsa.sign();
    }

    public static KeyPair generateKeyPair() throws Exception {
        // Generate a key pair for ECDSA
        KeyPairGenerator kpg = KeyPairGenerator.getInstance("EC");
        kpg.initialize(521); // Use a 521-bit key size for maximum security
        return kpg.generateKeyPair();
    }

    public static void main(String[] args) throws Exception {
        // Generate a key pair for ECDSA
        KeyPair kp = generateKeyPair();
        ECPublicKey publicKey = (ECPublicKey) kp.getPublic();
        ECPrivateKey privateKey = (ECPrivateKey) kp.getPrivate();

        // Create a packet data example
        byte[] packetData = "Hello, World!".getBytes();

        // Create a PacketIntegrityVerifier instance
        PacketIntegrityVerifier verifier = new PacketIntegrityVerifier(publicKey, privateKey);

        // Sign the packet data
        byte[] signature = verifier.signPacket(packetData);

        // Verify the packet integrity
        boolean isValid = verifier.verifyPacketIntegrity(packetData, signature);

        if (isValid) {
            System.out.println("Packet integrity verification successful!");
        } else {
            System.out.println("Packet integrity verification failed!");
        }
    }
}