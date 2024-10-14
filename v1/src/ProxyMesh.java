import org.bouncycastle.jce.provider.BouncyCastleProvider;
import org.bouncycastle.jsse.provider.BouncyCastleJsseProvider;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;

/**
 * Represents a mesh of proxies that can be used to create proxy chains.
 */
public class ProxyMesh {
    private List<Proxy> proxies; // all available proxies
    private int nextProxyIndex; // index of the next proxy to use
    private Random random; // random number generator
    private BouncyCastleJsseProvider jsseProvider; // Bouncy Castle JSSE provider

    /**
     * Creates a new ProxyMesh instance with the given list of proxies and Bouncy Castle JSSE provider.
     *
     * @param proxies the list of proxies to use
     * @param jsseProvider the Bouncy Castle JSSE provider
     */
    public ProxyMesh(List<Proxy> proxies, BouncyCastleJsseProvider jsseProvider) {
        this.proxies = proxies;
        this.nextProxyIndex = 0;
        this.random = new Random();
        this.jsseProvider = jsseProvider;
    }

    /**
     * Creates a new proxy chain using the next three proxies in the mesh.
     *
     * @return a new proxy chain
     */
    public ProxyChain createProxyChain() {
        if (proxies.isEmpty()) {
            throw new RuntimeException("No proxies available");
        }

        ProxyChain chain = new ProxyChain();

        // select the next three proxies from the queue
        Proxy[] selectedProxies = new Proxy[3];
        for (int i = 0; i < 3; i++) {
            selectedProxies[i] = proxies.get(nextProxyIndex);
            nextProxyIndex = (nextProxyIndex + 1) % proxies.size();
        }

        // randomly permute the selected proxies
        shuffleArray(selectedProxies);

        // create a new ProxyChain with the permuted proxies
        chain.setEntryProxy(selectedProxies[0]);
        chain.setMiddleProxy(selectedProxies[1]);
        chain.setExitProxy(selectedProxies[2]);

        // create an SSL socket factory using the Bouncy Castle provider
        SSLSocketFactory sslSocketFactory = (SSLSocketFactory) SSLSocketFactory.getInstance("TLS", jsseProvider);
        chain.setSslSocketFactory(sslSocketFactory);

        return chain;
    }

    /**
     * Randomly shuffles the elements of the given array.
     *
     * @param array the array to shuffle
     */
    private void shuffleArray(Proxy[] array) {
        for (int i = array.length - 1; i > 0; i--) {
            int index = random.nextInt(i + 1);
            Proxy temp = array[index];
            array[index] = array[i];
            array[i] = temp;
        }
    }

    /**
     * Returns the list of proxies used by this mesh.
     *
     * @return the list of proxies
     */
    public List<Proxy> getProxies() {
        return proxies;
    }

    /**
     * Sets the list of proxies used by this mesh.
     *
     * @param proxies the new list of proxies
     */
    public void setProxies(List<Proxy> proxies) {
        this.proxies = proxies;
    }

    /**
     * Returns the index of the next proxy to use.
     *
     * @return the index of the next proxy
     */
    public int getNextProxyIndex() {
        return nextProxyIndex;
    }

    /**
     * Sets the index of the next proxy to use.
     *
     * @param nextProxyIndex the new index
     */
    public void setNextProxyIndex(int nextProxyIndex) {
        this.nextProxyIndex = nextProxyIndex;
    }
}
