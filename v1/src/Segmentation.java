import java.util.ArrayList;
import java.util.List;

public class Segmentation {
    private int maxPacketSize;

    public Segmentation(int maxPacketSize) {
        this.maxPacketSize = maxPacketSize;
    }

    public List<Packet> segmentData(byte[] data) {
        List<Packet> packets = new ArrayList<>();
        int offset = 0;

        while (offset < data.length) {
            int chunkSize = Math.min(maxPacketSize, data.length - offset);
            byte[] chunk = new byte[chunkSize];
            System.arraycopy(data, offset, chunk, 0, chunkSize);
            Packet packet = new Packet(chunk);
            packets.add(packet);
            offset += chunkSize;
        }

        return packets;
    }

    public static class Packet {
        private byte[] data;

        public Packet(byte[] data) {
            this.data = data;
        }

        public byte[] getData() {
            return data;
        }
    }
}