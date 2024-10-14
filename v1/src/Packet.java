public class Packet {
    private final byte[] data;
    private final int sequenceNumber;
    private final byte[] authenticationTag;

    public Packet(byte[] data, int sequenceNumber, byte[] authenticationTag) {
        this.data = data;
        this.sequenceNumber = sequenceNumber;
        this.authenticationTag = authenticationTag;
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

    @Override
    public String toString() {
        return "Packet{" +
                "data=" + java.util.Arrays.toString(data) +
                ", sequenceNumber=" + sequenceNumber +
                ", authenticationTag=" + java.util.Arrays.toString(authenticationTag) +
                '}';
    }
}
