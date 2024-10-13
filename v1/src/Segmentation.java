import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

public class Segmentation {
    private static final int MAX_PACKET_SIZE = 1280;
    private SshuttleWrapper sshuttleWrapper;

    public Segmentation(SshuttleWrapper sshuttleWrapper) {
        this.sshuttleWrapper = sshuttleWrapper;
    }

    public List<Packet> segmentData(byte[] data) {
        List<Packet> packets = new ArrayList<>();
        int offset = 0;
        int sequenceNumber = 0;

        while (offset < data.length) {
            int chunkSize = Math.min(MAX_PACKET_SIZE, data.length - offset);
            byte[] chunk = new byte[chunkSize];
            System.arraycopy(data, offset, chunk, 0, chunkSize);
            Packet packet = new Packet(chunk, sequenceNumber, null, null, null, null, null, null);
            packets.add(packet);
            offset += chunkSize;
            sequenceNumber++;
        }

        return packets;
    }

    public byte[] consolidatePackets(List<Packet> packets) {
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        int expectedSequenceNumber = 0;

        for (Packet packet : packets) {
            if (!sshuttleWrapper.verify(packet.getData(), packet.getDigitalSignatureAuthKeyServerOutput(), packet.getKeyExchangeKeyServerOutput())) {
                throw new RuntimeException("Packet signature verification failed");
            }
            if (packet.getSequenceNumber() != expectedSequenceNumber) {
                throw new RuntimeException("Missing or corrupted packet");
            }
            bos.write(packet.getData());
            expectedSequenceNumber++;
        }

        return bos.toByteArray();
    }
}
