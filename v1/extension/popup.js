import init, { fetch_proxies } from './pkg/transmigrator.js';

async function run() {
    await init();
    document.getElementById('proxy-form').addEventListener('submit', async (event) => {
        event.preventDefault();
        const url = document.getElementById('url').value;
        if (url) {
            try {
                await fetch_proxies(url);
                console.log('Proxies fetched and stored in memory');
            } catch (error) {
                console.error('Error fetching proxies:', error);
            }
        }
    });
}

run();

