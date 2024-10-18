import init, { handle_fetch } from './pkg/transmigrator.js';

// Register the fetch event listener to call the handle_fetch function from Rust
self.addEventListener('fetch', event => {
    handle_fetch(event);
});

// Initialize the WebAssembly module
init().then(() => {
    console.log('Wasm module initialized');
});
