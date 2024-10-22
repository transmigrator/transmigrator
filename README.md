-The user temporarily loads a .txt file with a list of IP-only SOCKS5 catching all the proxies inside

-After this initial fetch, a .txt file is no longer needed unless success rate drops under 50% (warning)

-In order to segment the traffic, the size of the future HTTP request is pre-calculated but not sent yet

-For the HTTP requests, it hard-codes a max packet size of 576 bytes and min packet number of 3

-Before this it will have to do DoH resolution, TCP handshake, CONNECT request, TLS handshake

-The DNS over HTTPS (DoH) resolution using Cloudflare will make our future traffic look normal

-TCP, CONNECT, and TLS are done based on the pre-calculated number of segments (e.g. packets)

-Hard-coded size or number do not affect TCP handshake, CONNECT request, and TLS handshake

-Now it has an n number of proxy chains set up with the HTTPS connections between client-server

-The HTTP request is segmented, and each packet goes encrypted over TLS through a proxy chain

-When the server responds to the client, its response is TLS-encrypted too against malicious proxies

-Each proxy chain carries one HTTP request packet and is made of 3 dynamically chained proxies

-It starts with the next 3 proxies from a circular queue of proxies in the cache, fetched from the .txt

-To form a proxy chain, these 3 proxies are shuffled in a random permutation (no same exit nodes)

-Any server response is consolidated at the client-side after receiving all the proxy chain segments

-The system will allow 2 session modes: higher threat profile (ER) and highest threat profile (EST)

-The default EST means if the user wants to make a second HTTP request, there is a new session

-This means new proxy chains and repeating the TCP, CONNECT, and TLS process before HTTP

-Users can opt for ER that will keep the sessions open, allowing for new HTTP requests/responses

-In ER, after 1st HTTP request the 576-byte limit does not apply to fit packets in existing segments

-With EST, only one HTTP request packet per proxy chain and no further request in that segment

-When Firefox browser is closed, it acts as an automatic kill switch for the add-on (no persistence)

-Cryptographic keys live at the RAM and are not stored, merely rewritten or forgotten without leaks
