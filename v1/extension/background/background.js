import init, { handle_fetch } from './pkg/transmigrator.js';

self.addEventListener('fetch', event => {
    handle_fetch(event);
});

init().then(() => {
    console.log('Wasm module initialized');
});