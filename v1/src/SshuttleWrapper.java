import jpype.PythonObject;
import jpype.PythonInterpreter;

public class SshuttleWrapper {
    public static String getClientInputKeyK1() throws Exception {
        PythonInterpreter interpreter = new PythonInterpreter();
        interpreter.exec("import key_manager");
        PythonObject keyManager = interpreter.eval("key_manager.KeyManager()");
        PythonObject clientInputKeys = keyManager.invoke("generate_client_input_keys");
        String K1 = clientInputKeys.__getitem__(0).toString();
        interpreter.close();
        return K1;
    }

    public static String getClientInputKeyK2() throws Exception {
        PythonInterpreter interpreter = new PythonInterpreter();
        interpreter.exec("import key_manager");
        PythonObject keyManager = interpreter.eval("key_manager.KeyManager()");
        PythonObject clientInputKeys = keyManager.invoke("generate_client_input_keys");
        String K2 = clientInputKeys.__getitem__(1).toString();
        interpreter.close();
        return K2;
    }

    public static String getClientInputKeyK3() throws Exception {
        PythonInterpreter interpreter = new PythonInterpreter();
        interpreter.exec("import key_manager");
        PythonObject keyManager = interpreter.eval("key_manager.KeyManager()");
        PythonObject clientInputKeys = keyManager.invoke("generate_client_input_keys");
        String K3 = clientInputKeys.__getitem__(2).toString();
        interpreter.close();
        return K3;
    }

    public static String getServerOutputKeyK4() throws Exception {
        PythonInterpreter interpreter = new PythonInterpreter();
        interpreter.exec("import key_manager");
        PythonObject keyManager = interpreter.eval("key_manager.KeyManager()");
        PythonObject serverOutputKeys = keyManager.invoke("generate_server_output_keys");
        String K4 = serverOutputKeys.__getitem__(0).toString();
        interpreter.close();
        return K4;
    }

    public static String getServerOutputKeyK5() throws Exception {
        PythonInterpreter interpreter = new PythonInterpreter();
        interpreter.exec("import key_manager");
        PythonObject keyManager = interpreter.eval("key_manager.KeyManager()");
        PythonObject serverOutputKeys = keyManager.invoke("generate_server_output_keys");
        String K5 = serverOutputKeys.__getitem__(1).toString();
        interpreter.close();
        return K5;
    }

    public static String getServerOutputKeyK6() throws Exception {
        PythonInterpreter interpreter = new PythonInterpreter();
        interpreter.exec("import key_manager");
        PythonObject keyManager = interpreter.eval("key_manager.KeyManager()");
        PythonObject serverOutputKeys = keyManager.invoke("generate_server_output_keys");
        String K6 = serverOutputKeys.__getitem__(2).toString();
        interpreter.close();
        return K6;
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
        String K1 = clientInputKeys.__getitem__(0).toString();
        String K2 = clientInputKeys.__getitem__(1).toString();
        String K3 = clientInputKeys.__getitem__(2).toString();

        // Generate server-output keys (K4, K5, K6)
        PythonObject serverOutputKeys = keyManager.invoke("generate_server_output_keys");

        // Extract K4, K5, and K6 from the output
        String K4 = serverOutputKeys.__getitem__(0).toString();
        String K5 = serverOutputKeys.__getitem__(1).toString();
        String K6 = serverOutputKeys.__getitem__(2).toString();

        // TO DO: Implement key exchange and encryption using the generated keys

        // Clean up
        interpreter.close();
    }
}
