-The user temporarily loads a .txt file with a list of IP-only SOCKS5 catching all the proxies inside it

-After this initial fetch, a .txt file is no longer needed unless proxy mortality reaches 50% (warning)

-In order to segment the traffic, the size of the future HTTP request is calculated while not yet sent

-For HTTP requests, it hard-codes a minimum of 3 segments with maximum body size of 576 bytes

-1st must do DoH resolution, 2nd WebSocket (WSS), 3rd CONNECT request, and 4th TLS handshake

-The DNS over HTTPS (DoH) resolution using Cloudflare will make all subsequent traffic look normal

-The WSS-CONNECT-TLS process is therefore adjusted to a now known amount of request segments

-Hard-coded limits are non-binding for all WebSocket (WSS), CONNECT request, and TLS handshake

-Now it has an n number of proxy chains as a mesh of HTTPS connections between client and server

-The HTTP request is segmented, and each segment goes encrypted over TLS through a proxy chain

-When the server responds to the client, its response is TLS-encrypted too against malicious proxies

-The body is the only segmented part of requests with same headers (and same session state in ER)

-Each proxy chain carries one HTTP request segment and is made of 3 dynamically chained proxies

-In dynamic chains, the overall order of the queued list is respected while avoiding any dead proxies

-It starts with the next 3 proxies from a circular queue of proxies in the cache, fetched from the .txt

-To form a proxy chain, each 3 proxies are shuffled in a random permutation (changing chain roles)

-Any server response is consolidated at the client-side after fully receiving the proxy chain segments 

-The system must allow 2 session modes: highER threat profile (ER) and highEST threat profile (EST)

-In essence, ER includes session state (cookies) in the shared header and EST does not include them

-The default EST implies that if the user wants to make a follow-up HTTP request, it is a new session

-This means fresh proxy chains and repeating their WSS-CONNECT-TLS process for said new request

-Users can select ER mode to keep sessions open, which follows up with new requests for responses

-In ER, after the 1st request normal limits do not apply with next segments reusing all existing chains 

-In EST, it is only one request segment per chain and any further requests segmented into new chains

-When the Firefox browser is closed, it acts as an automatic kill switch for the add-on (no persistence)

-All cryptographic keys live at the RAM, never stored, merely rewritten and/or forgotten without leaks
