async function fetchProxies() {
    const url = document.getElementById('url-input').value;
    await wasm_bindgen.fetch_proxies(url);
}