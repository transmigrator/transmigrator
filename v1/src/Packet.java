public class Packet {
    private byte[] data;
    private int sequenceNumber;
    private PythonObject keyExchangeKeyClientInput;
    private PythonObject digitalSignatureAuthKeyClientInput;
    private PythonObject encryptionKeyClientInput;
    private PythonObject keyExchangeKeyServerOutput;
    private PythonObject digitalSignatureAuthKeyServerOutput;
    private PythonObject decryptionKeyServerOutput;

    public Packet(byte[] data, int sequenceNumber, PythonObject keyExchangeKeyClientInput, PythonObject digitalSignatureAuthKeyClientInput, PythonObject encryptionKeyClientInput, PythonObject keyExchangeKeyServerOutput, PythonObject digitalSignatureAuthKeyServerOutput, PythonObject decryptionKeyServerOutput) {
        this.data = data;
        this.sequenceNumber = sequenceNumber;
        this.keyExchangeKeyClientInput = keyExchangeKeyClientInput;
        this.digitalSignatureAuthKeyClientInput = digitalSignatureAuthKeyClientInput;
        this.encryptionKeyClientInput = encryptionKeyClientInput;
        this.keyExchangeKeyServerOutput = keyExchangeKeyServerOutput;
        this.digitalSignatureAuthKeyServerOutput = digitalSignatureAuthKeyServerOutput;
        this.decryptionKeyServerOutput = decryptionKeyServerOutput;
    }

    public byte[] getData() {
        return data;
    }

    public int getSequenceNumber() {
        return sequenceNumber;
    }

    public PythonObject getKeyExchangeKeyClientInput() {
        return keyExchangeKeyClientInput;
    }

    public PythonObject getDigitalSignatureAuthKeyClientInput() {
        return digitalSignatureAuthKeyClientInput;
    }

    public PythonObject getEncryptionKeyClientInput() {
        return encryptionKeyClientInput;
    }

    public PythonObject getKeyExchangeKeyServerOutput() {
        return keyExchangeKeyServerOutput;
    }

    public PythonObject getDigitalSignatureAuthKeyServerOutput() {
        return digitalSignatureAuthKeyServerOutput;
    }

    public PythonObject getDecryptionKeyServerOutput() {
        return decryptionKeyServerOutput;
    }
}
