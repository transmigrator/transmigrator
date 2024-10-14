import java.io.IOException;
import java.security.KeyPair;
import java.security.PrivateKey;
import java.security.PublicKey;

import javax.net.ssl.SSLSocket;
import javax.net.ssl.SSLSocketFactory;

public class SshuttleWrapper {
    private KeyManager keyManager;
    private SshuttleGateway sshuttleGateway;

    public SshuttleWrapper(KeyManager keyManager, SshuttleGateway sshuttleGateway) {
        this.keyManager = keyManager;
        this.sshuttleGateway = sshuttleGateway;
    }

    public void establishConnection(String host, int port) throws Exception {
        // Establish sshuttle connection using KeyManager
        KeyPair clientInputKeyPair = keyManager.generateClientInputKeys();
        KeyPair serverOutputKeyPair = keyManager.generateServerOutputKeys();

        // Use client-input keys (K1, K2, K3) for sending data to server
        PrivateKey clientInputPrivateKey = clientInputKeyPair.getPrivate();
        PublicKey clientInputPublicKey = clientInputKeyPair.getPublic();
        KeyPair k1 = keyManager.getK1();
        KeyPair k2 = keyManager.getK2();
        KeyPair k3 = keyManager.getK3();

        // Use server-output keys (K4, K5, K6) for receiving data from server
        PrivateKey serverOutputPrivateKey = serverOutputKeyPair.getPrivate();
        PublicKey serverOutputPublicKey = serverOutputKeyPair.getPublic();
        KeyPair k4 = keyManager.getK4();
        KeyPair k5 = keyManager.getK5();
        KeyPair k6 = keyManager.getK6();

        // Establish sshuttle connection using the generated keys
        sshuttleGateway.setupDnsResolver();
        SSLSocket socket = sshuttleGateway.connect(host, port);

        // Initialize sshuttle connection
        socket.startHandshake();
    }

    public void sendPacket(byte[] packet) throws Exception {
        // Use client-input keys (K1, K2, K3) for sending data to server
        KeyPair clientInputKeyPair = keyManager.generateClientInputKeys();
        PrivateKey clientInputPrivateKey = clientInputKeyPair.getPrivate();
        PublicKey clientInputPublicKey = clientInputKeyPair.getPublic();
        KeyPair k1 = keyManager.getK1();
        KeyPair k2 = keyManager.getK2();
        KeyPair k3 = keyManager.getK3();

        // Encrypt packet using K3
        byte[] encryptedPacket = keyManager.encrypt(packet, k3.getPrivate().getEncoded());

        // Send encrypted packet through sshuttle connection
        SSLSocket socket = sshuttleGateway.getSocket();
        socket.getOutputStream().write(encryptedPacket);
    }

    public byte[] receivePacket() throws Exception {
        // Use server-output keys (K4, K5, K6) for receiving data from server
        KeyPair serverOutputKeyPair = keyManager.generateServerOutputKeys();
        PrivateKey serverOutputPrivateKey = serverOutputKeyPair.getPrivate();
        PublicKey serverOutputPublicKey = serverOutputKeyPair.getPublic();
        KeyPair k4 = keyManager.getK4();
        KeyPair k5 = keyManager.getK5();
        KeyPair k6 = keyManager.getK6();

        // Receive encrypted packet through sshuttle connection
        SSLSocket socket = sshuttleGateway.getSocket();
        byte[] encryptedPacket = new byte[1280];
        socket.getInputStream().read(encryptedPacket);

        // Decrypt packet using K6
        byte[] decryptedPacket = keyManager.decrypt(encryptedPacket, k6.getPrivate().getEncoded());

        return decryptedPacket;
    }

    public void closeConnection() throws IOException {
        sshuttleGateway.getSocket().close();
    }
}
