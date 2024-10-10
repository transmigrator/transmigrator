import random

class Proxy:
    def __init__(self, ip, port):
        self.ip = ip
        self.port = port

class ProxyChain:
    def __init__(self, proxies):
        self.proxies = proxies

    def get_proxies(self):
        return self.proxies

class ProxyMesh:
    def __init__(self):
        self.proxy_chains = []

    def create_proxy_chain(self, proxy_list):
        # Create a new ProxyChain with the given proxy list
        proxies = [Proxy(proxy[0], proxy[1]) for proxy in proxy_list]
        proxy_chain = ProxyChain(proxies)
        self.proxy_chains.append(proxy_chain)

    def get_proxy_chain(self):
        # Return a random ProxyChain from the list
        return random.choice(self.proxy_chains)

    def add_proxy_chain(self, proxy_chain):
        self.proxy_chains.append(proxy_chain)

    def remove_proxy_chain(self, proxy_chain):
        self.proxy_chains.remove(proxy_chain)

def load_proxies_from_file(filename):
    proxy_list = []
    with open(filename, 'r') as f:
        for line in f:
            ip, port = line.strip().split(':')
            proxy_list.append((ip, int(port)))
    return proxy_list

def create_proxy_mesh(filename):
    proxy_mesh = ProxyMesh()
    proxy_list = load_proxies_from_file(filename)
    # Create multiple ProxyChains with 3 proxies each
    for i in range(0, len(proxy_list), 3):
        proxy_chain_proxies = proxy_list[i:i+3]
        proxy_mesh.create_proxy_chain(proxy_chain_proxies)
    return proxy_mesh