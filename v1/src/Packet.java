public class Packet {
    private final byte[] data;
    private final int sequenceNumber;
    private final byte[] authenticationTag;
    private final KeyManager keyManager;

    public Packet(byte[] data, int sequenceNumber, byte[] authenticationTag, KeyManager keyManager) {
        this.data = data;
        this.sequenceNumber = sequenceNumber;
        this.authenticationTag = authenticationTag;
        this.keyManager = keyManager;
    }

    public byte[] getData() {
        return data;
    }

    public int getSequenceNumber() {
        return sequenceNumber;
    }

    public byte[] getAuthenticationTag() {
        return authenticationTag;
    }

    public byte[] encrypt() throws Exception {
        // Use client-input keys (K1, K2, K3) for sending data to server
        KeyPair clientInputKeyPair = keyManager.generateClientInputKeys();
        PrivateKey clientInputPrivateKey = clientInputKeyPair.getPrivate();
        PublicKey clientInputPublicKey = clientInputKeyPair.getPublic();
        KeyPair k1 = keyManager.getK1();
        KeyPair k2 = keyManager.getK2();
        KeyPair k3 = keyManager.getK3();

        // Encrypt packet using K3
        return keyManager.encrypt(data, k3.getPrivate().getEncoded());
    }

    public byte[] decrypt() throws Exception {
        // Use server-output keys (K4, K5, K6) for receiving data from server
        KeyPair serverOutputKeyPair = keyManager.generateServerOutputKeys();
        PrivateKey serverOutputPrivateKey = serverOutputKeyPair.getPrivate();
        PublicKey serverOutputPublicKey = serverOutputKeyPair.getPublic();
        KeyPair k4 = keyManager.getK4();
        KeyPair k5 = keyManager.getK5();
        KeyPair k6 = keyManager.getK6();

        // Decrypt packet using K6
        return keyManager.decrypt(data, k6.getPrivate().getEncoded());
    }

    @Override
    public String toString() {
        return "Packet{" +
                "data=" + java.util.Arrays.toString(data) +
                ", sequenceNumber=" + sequenceNumber +
                ", authenticationTag=" + java.util.Arrays.toString(authenticationTag) +
                '}';
    }
}
