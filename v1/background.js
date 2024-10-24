let proxyList = [];
let proxyQueue = [];
let deadProxies = new Set();
let sessionMode = 'EST'; // Default session mode
let liveCount = 0;
let failureCount = 0;
let deadCount = 0;

// Change session mode (optional)
function changeSessionMode(mode) {
  if (mode === 'EST' || mode === 'ER') {
    sessionMode = mode;
  }
}

// Load proxies from a .txt file
function loadProxies(file) {
  fetch(file)
    .then(response => response.text())
    .then(text => {
      proxyList = text.split('\n').filter(ip => ip.trim() !== '');
      proxyQueue = [...proxyList];
    });
}

// DNS over HTTPS (DoH) resolution
function resolveDNS(hostname) {
  const url = `https://cloudflare-dns.com/dns-query?name=${hostname}`;
  return fetch(url, {
    headers: {
      'Accept': 'application/dns-json'
    }
  }).then(response => response.json());
}

// Handle HTTP requests
async function handleRequest(details) {
  const requestUrl = new URL(details.url);

  // Perform DoH resolution
  const dnsResponse = await resolveDNS(requestUrl.hostname);
  const resolvedIP = dnsResponse.Answer[0].data;

  // Calculate the size of the HTTP request
  const requestSize = new TextEncoder().encode(details.requestBody.raw[0].bytes).length;

  // Segment the HTTP request into a minimum of 3 segments with a maximum body size of 576 bytes
  const maxSegmentSize = 576;
  const minSegmentNumber = 3;
  const segments = [];
  const segmentSize = Math.max(Math.ceil(requestSize / minSegmentNumber), maxSegmentSize);

  for (let i = 0; i < requestSize; i += segmentSize) {
    segments.push(details.requestBody.raw[0].bytes.slice(i, i + segmentSize));
  }

  // Create proxy chains for each segment (body segment)
  const proxyChains = segments.map(() => createProxyChain());

  // Handle session mode
  if (sessionMode === 'ER') {
    // Body segments of the same request share the same header
    // Include session state cookies in the header
    const headers = new Headers(details.requestHeaders);
    headers.append('Cookie', document.cookie);

    for (let i = 0; i < segments.length; i++) {
      const segment = segments[i];
      const chain = proxyChains[i];
      try {
        let ws;
        for (const proxy of chain) {
          ws = await establishTCPConnection(proxy);
          await sendCONNECTRequest(ws, resolvedIP, requestUrl.port);
        }
        const wss = await performTLSHandshake(ws, resolvedIP, requestUrl.port);
        await sendSegment(wss, segment, headers);
        liveCount++;
      } catch (error) {
        console.error(`Error sending segment through proxy chain: ${error}`);
        failureCount++;
        markProxyAsDead(chain);
      }
    }
  } else {
    // Body segments of the same request still share the same header
    // Do not include session state cookies in the header
    const headers = new Headers(details.requestHeaders);

    for (let i = 0; i < segments.length; i++) {
      const segment = segments[i];
      const chain = proxyChains[i];
      try {
        let ws;
        for (const proxy of chain) {
          ws = await establishTCPConnection(proxy);
          await sendCONNECTRequest(ws, resolvedIP, requestUrl.port);
        }
        const wss = await performTLSHandshake(ws, resolvedIP, requestUrl.port);
        await sendSegment(wss, segment, headers);
        liveCount++;
      } catch (error) {
        console.error(`Error sending segment through proxy chain: ${error}`);
        failureCount++;
        markProxyAsDead(chain);
      }
    }
  }

  // Monitor proxy mortality and issue a warning if it reaches 50%
  monitorMortalityRate();
}

// Shuffle the selected proxies to randomize their roles
function shuffleProxies(proxies) {
  for (let i = proxies.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [proxies[i], proxies[j]] = [proxies[j], proxies[i]];
  }
  return proxies;
}

// Monitor proxy mortality and issue a warning if it reaches 50%
function monitorMortalityRate() {
  const totalProxies = deadCount + liveCount;
  if (totalProxies > 0) {
    const mortalityRate = (deadCount / totalProxies) * 100;
    if (mortalityRate >= 50) {
      console.warn('Warning: Proxy mortality has reached 50%. Please load a new proxy list.');
      // Reload proxies if mortality rate reaches 50%
      loadProxies('path/to/proxies.txt');
    }
  }
}

// Mark proxies as dead
function markProxyAsDead(chain) {
  chain.forEach(proxy => deadProxies.add(proxy));
  deadCount++; // Increment deadCount
}

// Create a proxy chain
function createProxyChain() {
  // Ensure the queue has at least 9 live proxies
  if (proxyQueue.filter(proxy => !deadProxies.has(proxy)).length < 9) {
    proxyQueue = [...proxyList];
  }

  // Take the next 3 live proxies from the queue
  let chain = [];
  while (chain.length < 3 && proxyQueue.length > 0) {
    const proxy = proxyQueue.shift();
    if (!deadProxies.has(proxy)) {
      chain.push(proxy);
    }
  }

  // If we do not have enough proxies, refill the queue and try again
  if (chain.length < 3) {
    proxyQueue = [...proxyList];
    while (chain.length < 3 && proxyQueue.length > 0) {
      const proxy = proxyQueue.shift();
      if (!deadProxies.has(proxy)) {
        chain.push(proxy);
      }
    }
  }

  // Shuffle the selected proxies to randomize their roles
  chain = shuffleProxies(chain);

  return chain;
}

// Establish WebSocket over TCP connection to a proxy
async function establishTCPConnection(proxy) {
  // Implement WSS connection logic here
  // Example: Using WebSocket for simplicity
  return new Promise((resolve, reject) => {
    const ws = new WebSocket(`ws://${proxy}`);
    ws.onopen = () => resolve(ws);
    ws.onerror = (error) => reject(new Error(`Failed to establish TCP connection: ${error.message}`));
  });
}

// Send CONNECT request through a proxy
async function sendCONNECTRequest(ws, resolvedIP, port) {
  // Implement CONNECT request logic here
  // Example: Sending a simple CONNECT request over WebSocket
  return new Promise((resolve, reject) => {
    ws.send(`CONNECT ${resolvedIP}:${port} HTTP/1.1\r\n\r\n`);
    ws.onmessage = (event) => {
      if (event.data.includes('200 Connection established')) {
        resolve();
      } else {
        reject(new Error('CONNECT request failed'));
      }
    };
  });
}

// Perform TLS handshake with the final destination server
async function performTLSHandshake(ws, resolvedIP, port) {
  // Implement TLS handshake logic here
  // Example: Using WebSocket Secure (wss) for simplicity
  return new Promise((resolve, reject) => {
    const wss = new WebSocket(`wss://${resolvedIP}:${port}`);
    wss.onopen = () => resolve(wss);
    wss.onerror = (error) => reject(new Error(`TLS handshake failed: ${error.message}`));
  });
}

// Send segment through the established proxy chain
async function sendSegment(ws, segment, headers) {
  // Implement segment sending logic here
  return new Promise((resolve, reject) => {
    ws.send(segment);
    ws.onmessage = (event) => resolve(event.data);
    ws.onerror = (error) => reject(new Error(`Failed to send segment: ${error.message}`));
  });
}

// Add event listener for web requests
browser.webRequest.onBeforeRequest.addListener(
  handleRequest,
  { urls: ["<all_urls>"] },
  ["blocking", "requestBody"]
);

// Add listener for browser close to act as kill switch
browser.windows.onRemoved.addListener(() => {
  // Clear cryptographic keys and other sensitive data
});
