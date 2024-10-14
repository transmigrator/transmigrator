import jpype.PythonObject;
import jpype.PythonInterpreter;

public class SshuttleWrapper {
    public static byte[] getClientInputKeyK1() throws Exception {
        try {
            PythonInterpreter interpreter = new PythonInterpreter();
            interpreter.exec("import key_manager");
            PythonObject keyManager = interpreter.eval("key_manager.KeyManager()");
            PythonObject clientInputKeys = keyManager.invoke("generate_client_input_keys");
            byte[] K1 = clientInputKeys.__getitem__(0).tojava(byte[].class);
            interpreter.close();
            return K1;
        } catch (Exception e) {
            throw new Exception("Error generating client input key K1: " + e.getMessage());
        }
    }

    public static byte[] getClientInputKeyK2() throws Exception {
        try {
            PythonInterpreter interpreter = new PythonInterpreter();
            interpreter.exec("import key_manager");
            PythonObject keyManager = interpreter.eval("key_manager.KeyManager()");
            PythonObject clientInputKeys = keyManager.invoke("generate_client_input_keys");
            byte[] K2 = clientInputKeys.__getitem__(1).tojava(byte[].class);
            interpreter.close();
            return K2;
        } catch (Exception e) {
            throw new Exception("Error generating client input key K2: " + e.getMessage());
        }
    }

    public static byte[] getClientInputKeyK3() throws Exception {
        try {
            PythonInterpreter interpreter = new PythonInterpreter();
            interpreter.exec("import key_manager");
            PythonObject keyManager = interpreter.eval("key_manager.KeyManager()");
            PythonObject clientInputKeys = keyManager.invoke("generate_client_input_keys");
            byte[] K3 = clientInputKeys.__getitem__(2).tojava(byte[].class);
            interpreter.close();
            return K3;
        } catch (Exception e) {
            throw new Exception("Error generating client input key K3: " + e.getMessage());
        }
    }

    public static byte[] getServerOutputKeyK4() throws Exception {
        try {
            PythonInterpreter interpreter = new PythonInterpreter();
            interpreter.exec("import key_manager");
            PythonObject keyManager = interpreter.eval("key_manager.KeyManager()");
            PythonObject serverOutputKeys = keyManager.invoke("generate_server_output_keys");
            byte[] K4 = serverOutputKeys.__getitem__(0).tojava(byte[].class);
            interpreter.close();
            return K4;
        } catch (Exception e) {
            throw new Exception("Error generating server output key K4: " + e.getMessage());
        }
    }

    public static byte[] getServerOutputKeyK5() throws Exception {
        try {
            PythonInterpreter interpreter = new PythonInterpreter();
            interpreter.exec("import key_manager");
            PythonObject keyManager = interpreter.eval("key_manager.KeyManager()");
            PythonObject serverOutputKeys = keyManager.invoke("generate_server_output_keys");
            byte[] K5 = serverOutputKeys.__getitem__(1).tojava(byte[].class);
            interpreter.close();
            return K5;
        } catch (Exception e) {
            throw new Exception("Error generating server output key K5: " + e.getMessage());
        }
    }

    public static byte[] getServerOutputKeyK6() throws Exception {
        try {
            PythonInterpreter interpreter = new PythonInterpreter();
            interpreter.exec("import key_manager");
            PythonObject keyManager = interpreter.eval("key_manager.KeyManager()");
            PythonObject serverOutputKeys = keyManager.invoke("generate_server_output_keys");
            byte[] K6 = serverOutputKeys.__getitem__(2).tojava(byte[].class);
            interpreter.close();
            return K6;
        } catch (Exception e) {
            throw new Exception("Error generating server output key K6: " + e.getMessage());
        }
    }

    public static void main(String[] args) throws Exception {
        // Initialize the Python interpreter
        PythonInterpreter interpreter = new PythonInterpreter();

        // Import the key_manager module
        interpreter.exec("import key_manager");

        // Create a new KeyManager object
        PythonObject keyManager = interpreter.eval("key_manager.KeyManager()");

        // Generate client-input keys (K1, K2, K3)
        PythonObject clientInputKeys = keyManager.invoke("generate_client_input_keys");

        // Extract K1, K2, and K3 from the output
        byte[] K1 = clientInputKeys.__getitem__(0).tojava(byte[].class);
        byte[] K2 = clientInputKeys.__getitem__(1).tojava(byte[].class);
        byte[] K3 = clientInputKeys.__getitem__(2).tojava(byte[].class);

        // Generate server-output keys (K4, K5, K6)
        PythonObject serverOutputKeys = keyManager.invoke("generate_server_output_keys");

        // Extract K4, K5, and K6 from the output
        byte[] K4 = serverOutputKeys.__getitem__(0).tojava(byte[].class);
        byte[] K5 = serverOutputKeys.__getitem__(1).tojava(byte[].class);
        byte[] K6 = serverOutputKeys.__getitem__(2).tojava(byte[].class);

        // TO DO: Implement key exchange and encryption using the generated keys

        // Clean up
        interpreter.close();
    }
}
