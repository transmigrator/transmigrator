-The user temporarily loads a .txt file with a list of IP-only SOCKS5 catching all the proxies inside

-After this initial fetch, a .txt file is no longer needed unless proxy mortality reaches 50% (warning)

-In order to segment the traffic, the size of the future HTTP request is calculated while not yet sent

-For HTTP requests, it hard-codes a minimum of 3 packets with a maximum body size of 576 bytes

-Before this it will have to do DoH resolution, TCP handshake, CONNECT request, TLS handshake

-The DNS over HTTPS (DoH) resolution using Cloudflare makes all subsequent traffic look normal

-TCP, CONNECT, and TLS are done based on the calculated number of body segments (or packets)

-Hard-coded size/number non-binding for TCP handshake, CONNECT request, and TLS handshake

-Now it has an n number of proxy chains set up with the HTTPS connections between client-server

-The HTTP request is segmented, and each packet goes encrypted over TLS through a proxy chain

-When the server responds to the client, its response is TLS-encrypted too against malicious proxies

-The body is the only segmented part of requests with intact headers (and same session state in ER)

-Each proxy chain carries one HTTP request packet and is made of 3 dynamically chained proxies

-In dynamic chains, the overall order of the queued list is respected while avoiding any dead proxies

-It starts with the next 3 proxies from a circular queue of proxies in the cache, fetched from the .txt

-To form a proxy chain, each 3 proxies are shuffled in a random permutation (no same exit nodes)

-Any server response is consolidated at the client-side after receiving all the proxy chain segments 

-The system will allow 2 session modes: higher threat profile (ER) and highest threat profile (EST)

-In essence, ER includes session state cookies in the shared header and EST does not include them

-The default EST implies if the user wants to make a second HTTP request, there is a new session

-This means new proxy chains and repeating the TCP, CONNECT, and TLS process before HTTP

-Users can select ER mode that keeps sessions open, allowing follow-up HTTP requests/responses

-In ER, after 1st request normal limits do not apply with next segments reusing the existing chains 

-With EST, only one HTTP request segment/packet per chain and all further requests in new chains

-When Firefox browser is closed, it acts as an automatic kill switch for the add-on (no persistence)

-Cryptographic keys live at the RAM and are not stored, merely rewritten or forgotten without leaks
