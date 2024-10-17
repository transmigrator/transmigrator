import init, { fetch_proxies } from './pkg/transmigrator.js';

async function fetchProxies() {
    const url = document.getElementById('url').value;
    await fetch_proxies(url);
}

init().then(() => {
    console.log('Wasm loaded');
});
