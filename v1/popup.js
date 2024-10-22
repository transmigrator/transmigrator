document.getElementById('loadProxies').addEventListener('click', () => {
  const file = document.getElementById('proxyFile').files[0];
  if (file) {
    const reader = new FileReader();
    reader.onload = () => {
      browser.runtime.sendMessage({ action: 'loadProxies', data: reader.result });
    };
    reader.readAsText(file);
  }
});

document.getElementById('sessionMode').addEventListener('change', (event) => {
  const mode = event.target.value;
  browser.runtime.sendMessage({ action: 'setSessionMode', data: mode });
});
