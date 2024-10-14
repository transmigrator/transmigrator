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
        int sequenceNumber = 0;

        for (int i = 0; i < data.length; i += MAX_PACKET_SIZE) {
            int packetSize = Math.min(MAX_PACKET_SIZE, data.length - i);
            byte[] packetData = new byte[packetSize];
            System.arraycopy(data, i, packetData, 0, packetSize);

            byte[] digitalSignatureAuthKeyServerOutput = sshuttleWrapper.getServerOutputKeyK5();
            byte[] keyExchangeKeyServerOutput = sshuttleWrapper.getServerOutputKeyK4();

            Packet packet = new Packet(packetData, sequenceNumber, null, null, null, keyExchangeKeyServerOutput, digitalSignatureAuthKeyServerOutput, sshuttleWrapper.getServerOutputKeyK6());
            packets.add(packet);

            sequenceNumber++;
        }

        return packets;
    }

    public byte[] consolidatePackets(List<Packet> packets) {
        if (packets == null || packets.isEmpty()) {
            throw new RuntimeException("No packets to consolidate");
        }

        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        int expectedSequenceNumber = 0;

        for (Packet packet : packets) {
            if (packet == null) {
                throw new RuntimeException("Null packet encountered");
            }

            if (packet.getSequenceNumber() < 0) {
                throw new RuntimeException("Invalid sequence number");
            }

            if (packet.getData() == null || packet.getData().length == 0) {
                throw new RuntimeException("Packet data is empty");
            }

            if (packet.getDigitalSignatureAuthKeyServerOutput() == null || packet.getDigitalSignatureAuthKeyServerOutput().length == 0) {
                throw new RuntimeException("Packet digital signature authentication key is empty");
            }

            if (packet.getKeyExchangeKeyServerOutput() == null || packet.getKeyExchangeKeyServerOutput().length == 0) {
                throw new RuntimeException("Packet key exchange key is empty");
            }

            if (!sshuttleWrapper.verify(packet.getData(), packet.getDigitalSignatureAuthKeyServerOutput(), packet.getKeyExchangeKeyServerOutput())) {
                throw new RuntimeException("Packet signature verification failed");
            }

            if (packet.getSequenceNumber() != expectedSequenceNumber) {
                throw new RuntimeException("Missing or corrupted packet");
            }

            try {
                bos.write(packet.getData());
            } catch (IOException e) {
                throw new RuntimeException("Error writing packet data to output stream", e);
            }

            expectedSequenceNumber++;
        }

        return bos.toByteArray();
    }
}
