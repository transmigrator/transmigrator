import jpype.PythonObject;
import jpype.PythonInterpreter;
import org.bouncycastle.jce.provider.BouncyCastleProvider;
import org.bouncycastle.jsse.provider.BouncyCastleJsseProvider;
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
        PythonInterpreter interpreter = new PythonInterpreter();
        interpreter.exec("import sshuttle");
        PythonObject sshuttle = interpreter.get("sshuttle");

        // Use sshuttle to establish a connection
        sshuttle.__call__("connect", host, port, "-v", "--dns", dohProxyServer);

        // Get the socket object from sshuttle
        // Note: sshuttle does not provide a direct way to get the socket object
        // We will use the socket created by sshuttle to establish a SSL connection
        SocketAddress proxyAddress = new InetSocketAddress(sshuttleProxyServer, sshuttleProxyPort);
        Proxy proxy = new Proxy(Proxy.Type.SOCKS, proxyAddress);
        Socket socket = new Socket(proxy);

        // Connect to the target host and port through the sshuttle proxy server
        socket.connect(new InetSocketAddress(host, port));

        // Upgrade the socket to an SSL socket using Bouncy Castle
        SSLSocketFactory sslSocketFactory = (SSLSocketFactory) SSLSocketFactory.getInstance("TLS", new BouncyCastleJsseProvider());
        SSLSocket sslSocket = (SSLSocket) sslSocketFactory.createSocket(socket, host, port, true);

        // Start the SSL handshake
        sslSocket.startHandshake();

        return sslSocket;
    }
}
