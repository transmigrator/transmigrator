public class Packet {
    private byte[] data;
    private String source;
    private String destination;

    public Packet(byte[] data, String source, String destination) {
        this.data = data;
        this.source = source;
        this.destination = destination;
    }

    public byte[] getData() {
        return data;
    }

    public String getSource() {
        return source;
    }

    public String getDestination() {
        return destination;
    }

    public void encrypt() {
        // ...
    }

    public void decrypt() {
        // ...
    }
}