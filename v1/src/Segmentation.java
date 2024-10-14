import java.security.KeyPair;
import java.util.ArrayList;
import java.util.List;

public class Segmentation {
    private KeyManager keyManager;
    private int packetSize;

    public Segmentation(KeyManager keyManager) {
        this.keyManager = keyManager;
        this.packetSize = 1280;
    }

    public List<byte[]> segmentData(byte[] data) throws Exception {
        List<byte[]> packets = new ArrayList<>();

        // Use client-input keys (K1, K2, K3) for sending data to server
        KeyPair clientInputKeyPair = keyManager.generateClientInputKeys();
        PrivateKey clientInputPrivateKey = clientInputKeyPair.getPrivate();
        PublicKey clientInputPublicKey = clientInputKeyPair.getPublic();
        KeyPair k1 = keyManager.getK1();
        KeyPair k2 = keyManager.getK2();
        KeyPair k3 = keyManager.getK3();

        // Segment data into packets of packetSize bytes
        for (int i = 0; i < data.length; i += packetSize) {
            int packetLength = Math.min(packetSize, data.length - i);
            byte[] packet = new byte[packetLength];
            System.arraycopy(data, i, packet, 0, packetLength);

            // Encrypt packet using K3
            byte[] encryptedPacket = keyManager.encrypt(packet, k3.getPrivate().getEncoded());

            packets.add(encryptedPacket);
        }

        return packets;
    }

    public byte[] consolidatePackets(List<byte[]> packets) throws Exception {
        byte[] consolidatedData = new byte[0];

        // Use server-output keys (K4, K5, K6) for receiving data from server
        KeyPair serverOutputKeyPair = keyManager.generateServerOutputKeys();
        PrivateKey serverOutputPrivateKey = serverOutputKeyPair.getPrivate();
        PublicKey serverOutputPublicKey = serverOutputKeyPair.getPublic();
        KeyPair k4 = keyManager.getK4();
        KeyPair k5 = keyManager.getK5();
        KeyPair k6 = keyManager.getK6();

        // Consolidate packets into a single byte array
        for (byte[] packet : packets) {
            // Decrypt packet using K6
            byte[] decryptedPacket = keyManager.decrypt(packet, k6.getPrivate().getEncoded());

            // Append decrypted packet to consolidated data
            byte[] newConsolidatedData = new byte[consolidatedData.length + decryptedPacket.length];
            System.arraycopy(consolidatedData, 0, newConsolidatedData, 0, consolidatedData.length);
            System.arraycopy(decryptedPacket, 0, newConsolidatedData, consolidatedData.length, decryptedPacket.length);
            consolidatedData = newConsolidatedData;
        }

        return consolidatedData;
    }
}
