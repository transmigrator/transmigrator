import org.xbill.DNS.*;

import java.io.IOException;
import java.net.InetSocketAddress;
import java.net.Proxy;
import java.net.Socket;
import java.net.SocketAddress;

import javax.net.ssl.SSLSocket;
import javax.net.ssl.SSLSocketFactory;

public class SshuttleGateway {
    private String dohProxyServer;
    private int dohProxyPort;
    private String sshuttleProxyServer;
    private int sshuttleProxyPort;

    public SshuttleGateway(String dohProxyServer, int dohProxyPort, String sshuttleProxyServer, int sshuttleProxyPort) {
        this.dohProxyServer = dohProxyServer;
        this.dohProxyPort = dohProxyPort;
        this.sshuttleProxyServer = sshuttleProxyServer;
        this.sshuttleProxyPort = sshuttleProxyPort;
    }

    public void setupDnsResolver() {
        // Create a DoH resolver that uses the same proxy servers as sshuttle
        ExtendedResolver resolver = new ExtendedResolver();
        resolver.setHTTPS(true); // Use HTTPS for DNS queries
        resolver.setPort(dohProxyPort); // Use port 443 for DoH
        resolver.addServer(dohProxyServer); // Add the DoH proxy server

        // Set the DoH resolver as the default DNS resolver
        Lookup.setDefaultResolver(resolver);
    }

    public Socket connect(String host, int port) throws IOException {
        // Create a socket that uses the sshuttle proxy server
        SocketAddress proxyAddress = new InetSocketAddress(sshuttleProxyServer, sshuttleProxyPort);
        Proxy proxy = new Proxy(Proxy.Type.SOCKS, proxyAddress);
        Socket socket = new Socket(proxy);

        // Connect to the target host and port through the sshuttle proxy server
        socket.connect(new InetSocketAddress(host, port));

        // Upgrade the socket to an SSL socket
        SSLSocketFactory sslSocketFactory = (SSLSocketFactory) SSLSocketFactory.getDefault();
        SSLSocket sslSocket = (SSLSocket) sslSocketFactory.createSocket(socket, host, port, true);

        // Start the SSL handshake
        sslSocket.startHandshake();

        return sslSocket;
    }
}
