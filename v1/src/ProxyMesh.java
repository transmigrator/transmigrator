import java.util.ArrayList;
import java.util.List;
import java.util.Random;

public class ProxyMesh {
    private List<Proxy> proxies; // all available proxies
    private int nextProxyIndex; // index of the next proxy to use
    private Random random; // random number generator

    public ProxyMesh(List<Proxy> proxies) {
        this.proxies = proxies;
        this.nextProxyIndex = 0;
        this.random = new Random();
    }

    public ProxyChain createProxyChain() {
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

        return chain;
    }

    private void shuffleArray(Proxy[] array) {
        for (int i = array.length - 1; i > 0; i--) {
            int index = random.nextInt(i + 1);
            Proxy temp = array[index];
            array[index] = array[i];
            array[i] = temp;
        }
    }

    public List<Proxy> getProxies() {
        return proxies;
    }

    public void setProxies(List<Proxy> proxies) {
        this.proxies = proxies;
    }

    public int getNextProxyIndex() {
        return nextProxyIndex;
    }

    public void setNextProxyIndex(int nextProxyIndex) {
        this.nextProxyIndex = nextProxyIndex;
    }
}
