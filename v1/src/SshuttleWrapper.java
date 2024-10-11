import jpype.PythonObject;
import jpype.PythonException;

public class SshuttleWrapper {
    private PythonObject packetHandler;
    private PythonObject proxyMesh;
    private PythonObject K1; // key exchange key (client-input)
    private PythonObject K2; // digital signature/auth key (client-input)
    private PythonObject K3; // encryption key (client-input)
    private PythonObject K4; // key exchange key (server-output)
    private PythonObject K5; // digital signature/auth key (server-output)
    private PythonObject K6; // decryption key (server-output)

    public SshuttleWrapper() {
        // Initialize the packet handler and proxy mesh objects
        PythonObject.importModule("packet_handler");
        packetHandler = PythonObject.getInstance("packet_handler");

        PythonObject.importModule("proxy_mesh");
        proxyMesh = PythonObject.getInstance("proxy_mesh");
    }

    public PythonObject generateKeypair() {
        try {
            // Generate a keypair using the packet handler
            PythonObject keypair = packetHandler.invoke("generate_keypair");
            
            // Store the keypair for later use
            K1 = keypair.__getattr__("private_key"); // key exchange key (client-input)
            K2 = keypair.__getattr__("private_key"); // digital signature/auth key (client-input)
            K3 = keypair.__getattr__("private_key"); // encryption key (client-input)
            K4 = keypair.__getattr__("public_key"); // key exchange key (server-output)
            K5 = keypair.__getattr__("public_key"); // digital signature/auth key (server-output)
            K6 = keypair.__getattr__("public_key"); // decryption key (server-output)
            
            return keypair;
        } catch (PythonException e) {
            // Handle any Python exceptions that may occur when generating a keypair
            System.err.println("Error generating keypair: " + e.getMessage());
            return null;
        }
    }

    public PythonObject ecdhKeyExchange(PythonObject private_key, PythonObject peer_public_key) {
        try {
            // Perform ECDH key exchange using the packet handler
            PythonObject shared_secret = packetHandler.invoke("ecdh_key_exchange", private_key, peer_public_key);
            
            // Reuse the same key for sshuttle and SSL
            K1 = shared_secret; // key exchange key (client-input)
            K4 = shared_secret; // key exchange key (server-output)
            
            return shared_secret;
        } catch (PythonException e) {
            // Handle any Python exceptions that may occur when performing ECDH key exchange
            System.err.println("Error performing ECDH key exchange: " + e.getMessage());
            return null;
        }
    }

    public PythonObject deriveSymmetricKey(PythonObject shared_secret) {
        try {
            // Derive a symmetric key using the packet handler
            PythonObject symmetric_key = packetHandler.invoke("derive_symmetric_key", shared_secret);
            
            // Reuse the same key for sshuttle and SSL
            K3 = symmetric_key; // encryption key (client-input)
            K6 = symmetric_key; // decryption key (server-output)
            
            return symmetric_key;
        } catch (PythonException e) {
            // Handle any Python exceptions that may occur when deriving a symmetric key
            System.err.println("Error deriving symmetric key: " + e.getMessage());
            return null;
        }
    }

    public PythonObject encrypt(byte[] data, PythonObject symmetric_key) {
        try {
            // Encrypt the data using the packet handler
            PythonObject encrypted_data = packetHandler.invoke("encrypt", data, symmetric_key);
            
            // Reuse the same key for sshuttle and SSL
            K3 = symmetric_key; // encryption key (client-input)
            
            return encrypted_data;
        } catch (PythonException e) {
            // Handle any Python exceptions that may occur when encrypting data
            System.err.println("Error encrypting data: " + e.getMessage());
            return null;
        }
    }

    public PythonObject decrypt(byte[] data, PythonObject symmetric_key) {
        try {
            // Decrypt the data using the packet handler
            PythonObject decrypted_data = packetHandler.invoke("decrypt", data, symmetric_key);
            
            // Reuse the same key for sshuttle and SSL
            K6 = symmetric_key; // decryption key (server-output)
            
            return decrypted_data;
        } catch (PythonException e) {
            // Handle any Python exceptions that may occur when decrypting data
            System.err.println("Error decrypting data: " + e.getMessage());
            return null;
        }
    }

    public PythonObject sign(byte[] data, PythonObject private_key) {
        try {
            // Sign the data using the packet handler
            PythonObject signature = packetHandler.invoke("sign", data, private_key);
            
            // Reuse the same key for sshuttle and SSL
            K2 = private_key; // digital signature/auth key (client-input)
            
            return signature;
        } catch (PythonException e) {
            // Handle any Python exceptions that may occur when signing data
            System.err.println("Error signing data: " + e.getMessage());
            return null;
        }
    }

    public PythonObject verify(byte[] data, PythonObject signature, PythonObject public_key) {
        try {
            // Verify the signature using the packet handler
            PythonObject verification_result = packetHandler.invoke("verify", data, signature, public_key);
            
            // Reuse the same key for sshuttle and SSL
            K5 = public_key; // digital signature/auth key (server-output)
            
            return verification_result;
        } catch (PythonException e) {
            // Handle any Python exceptions that may occur when verifying a signature
            System.err.println("Error verifying signature: " + e.getMessage());
            return null;
        }
    }
}
