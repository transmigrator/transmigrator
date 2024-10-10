import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class ProxyMeshConfig {
    private String[] proxyList;
    private static final int NUM_PROXIES_IN_SERIES = 3;

    /**
     * Constructs a new ProxyMeshConfig object with the given proxy list.
     *
     * @param proxyList the list of proxies to use
     */
    public ProxyMeshConfig(String[] proxyList) {
        if (proxyList == null || proxyList.length == 0) {
            throw new IllegalArgumentException("Proxy list cannot be null or empty");
        }
        this.proxyList = proxyList;
    }

    /**
     * Loads the proxy list from a file.
     *
     * @param proxyListFile the file containing the proxy list
     * @return a new ProxyMeshConfig object with the loaded proxy list
     * @throws IOException if an error occurs while reading the file
     */
    public static ProxyMeshConfig loadProxyListFromFile(String proxyListFile) throws IOException {
        Set<String> proxyList = new HashSet<>();
        try (BufferedReader reader = new BufferedReader(new FileReader(proxyListFile))) {
            String line;
            while ((line = reader.readLine()) != null) {
                String proxy = line.trim();
                if (!proxy.isEmpty() && isValidProxy(proxy)) {
                    proxyList.add(proxy);
                }
            }
        }
        return new ProxyMeshConfig(proxyList.toArray(new String[0]));
    }

    private static boolean isValidProxy(String proxy) {
        String[] parts = proxy.split(":");
        if (parts.length != 2) {
            return false;
        }
        String host = parts[0];
        String port = parts[1];
        if (host.isEmpty() || port.isEmpty()) {
            return false;
        }
        try {
            int portNumber = Integer.parseInt(port);
            if (portNumber < 0 || portNumber > 65535) {
                return false;
            }
        } catch (NumberFormatException e) {
            return false;
        }
        return true;
    }

    /**
     * Returns the list of proxies.
     *
     * @return the list of proxies
     */
    public String[] getProxyList() {
        return proxyList;
    }

    /**
     * Returns the number of proxies in series.
     *
     * @return the number of proxies in series
     */
    public int getNumProxiesInSeries() {
        return NUM_PROXIES_IN_SERIES;
    }

    /**
     * Returns the number of proxy chains in parallel based on the number of packets.
     *
     * @param numPackets the number of packets
     * @return the number of proxy chains in parallel
     */
    public int getNumProxyChainsInParallel(int numPackets) {
        if (numPackets < 0) {
            throw new IllegalArgumentException("Number of packets cannot be negative");
        }

        // Calculate the number of proxy chains in parallel based on the number of packets
        // and the total number of proxies available
        int maxProxyChains = proxyList.length / NUM_PROXIES_IN_SERIES;
        if (maxProxyChains == 0) {
            // Handle the case where maxProxyChains is zero
            return 0;
        }
        return Math.min(numPackets, maxProxyChains);
    }
}