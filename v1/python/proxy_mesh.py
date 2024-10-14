import random


class ProxyMesh:
    def __init__(self):
        self.proxyChains = []

    def createProxyChain(self, proxyList):
        # Create a new ProxyChain with the given proxy list
        proxyChain = proxyList
        self.proxyChains.append(proxyChain)

    def getProxyChain(self):
        # Return a random ProxyChain from the list
        return random.choice(self.proxyChains)

    def addProxyChain(self, proxyChain):
        self.proxyChains.append(proxyChain)

    def removeProxyChain(self, proxyChain):
        self.proxyChains.remove(proxyChain)


def loadProxiesFromFiles(filename):
    proxyList = []
    with open(filename, 'r') as f:
        for line in f:
            ip, port = line.strip().split(':')
            proxyList.append((ip, int(port)))
    return proxyList


def createProxyMeshInstance(filename):
    proxyMesh = ProxyMesh()
    proxyList = loadProxiesFromFiles(filename)
    # Create multiple ProxyChains with 3 proxies each
    for i in range(0, len(proxyList), 3):
        proxyChainProxies = proxyList[i:i+3]
        proxyMesh.createProxyChain(proxyChainProxies)
    return proxyMesh
