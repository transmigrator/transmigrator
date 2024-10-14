import org.bouncycastle.jce.provider.BouncyCastleProvider;
import org.bouncycastle.jsse.BCSSLContext;
import org.bouncycastle.jsse.BCSSLParameters;
import org.bouncycastle.jsse.BCSSLSocketFactory;

import javax.net.ssl.SSLContext;
import javax.net.ssl.SSLSocketFactory;
import java.net.InetSocketAddress;
import java.security.Security;
import java.util.ArrayList;
import java.util.List;

/**
 * Represents a chain of proxies.
 */
public class ProxyChain {
    private List<InetSocketAddress> proxies;
    private static final int MAX_PROXIES = 3;
    private SSLSocketFactory sslSocketFactory;

    /**
     * Constructs a new ProxyChain object.
     */
    public ProxyChain() {
        this.proxies = new ArrayList<>(MAX_PROXIES);
        // Initialize Bouncy Castle provider
        Security.addProvider(new BouncyCastleProvider());
    }

    /**
     * Adds a proxy to the chain.
     *
     * @param proxy the proxy to add
     */
    public void addProxy(InetSocketAddress proxy) {
        if (proxy == null) {
            throw new NullPointerException("Proxy cannot be null");
        }
        if (proxies.size() < MAX_PROXIES) {
            proxies.add(proxy);
        } else {
            throw new IllegalStateException("Proxy chain is full");
        }
    }

    /**
     * Returns the list of proxies in the chain.
     *
     * @return the list of proxies
     */
    public List<InetSocketAddress> getProxies() {
        return proxies;
    }

    /**
     * Sets the SSL socket factory for the proxy chain.
     *
     * @param sslSocketFactory the SSL socket factory
     */
    public void setSslSocketFactory(SSLSocketFactory sslSocketFactory) {
        this.sslSocketFactory = sslSocketFactory;
    }

    /**
     * Creates a default SSL socket factory using Bouncy Castle.
     *
     * @return the default SSL socket factory
     */
    public SSLSocketFactory createDefaultSslSocketFactory() {
        try {
            // Create a Bouncy Castle SSL context
            SSLContext sslContext = BCSSLContext.getInstance("TLS", new BouncyCastleProvider());
            // Create a Bouncy Castle SSL socket factory
            BCSSLSocketFactory sslSocketFactory = (BCSSLSocketFactory) sslContext.getSocketFactory();
            // Set the SSL parameters
            BCSSLParameters sslParameters = new BCSSLParameters();
            // ... customize SSL parameters as needed ...
            sslSocketFactory.setParameters(sslParameters);
            return sslSocketFactory;
        } catch (Exception e) {
            throw new RuntimeException("Failed to create default SSL socket factory", e);
        }
    }
}
