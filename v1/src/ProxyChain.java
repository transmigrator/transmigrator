import java.net.InetSocketAddress;
import java.util.ArrayList;
import java.util.List;

/**
 * Represents a chain of proxies.
 */
public class ProxyChain {
    private List<InetSocketAddress> proxies;
    private static final int MAX_PROXIES = 3;

    /**
     * Constructs a new ProxyChain object.
     */
    public ProxyChain() {
        this.proxies = new ArrayList<>(MAX_PROXIES);
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
}