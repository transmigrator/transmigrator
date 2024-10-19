use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tokio::sync::broadcast;
use log::{info, error};

pub async fn create_tunnel(proxy: &str, target: &str) -> io::Result<()> {
    // Bind the listener to a random local port
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let local_addr = listener.local_addr()?;
    info!("Listening on {}", local_addr);

    // Create a channel for graceful shutdown
    let (shutdown_tx, _) = broadcast::channel(1);

    // Spawn a new task to handle incoming connections
    tokio::spawn({
        let shutdown_tx = shutdown_tx.clone();
        async move {
            loop {
                tokio::select! {
                    result = listener.accept() => {
                        match result {
                            Ok((mut inbound, _)) => {
                                let proxy = proxy.to_string();
                                let target = target.to_string();
                                let shutdown_tx = shutdown_tx.clone();

                                // Spawn a new task to handle the connection
                                tokio::spawn(async move {
                                    match TcpStream::connect(&proxy).await {
                                        Ok(mut outbound) => {
                                            // Send the target address to the proxy
                                            if outbound.write_all(target.as_bytes()).await.is_ok() {
                                                let (mut ri, mut wi) = inbound.split();
                                                let (mut ro, mut wo) = outbound.split();

                                                // Copy data between client and server
                                                let client_to_server = tokio::io::copy(&mut ri, &mut wo);
                                                let server_to_client = tokio::io::copy(&mut ro, &mut wi);

                                                // Use select to handle both directions concurrently
                                                tokio::select! {
                                                    result = client_to_server => {
                                                        if let Err(e) = result {
                                                            error!("Client to server copy error: {}", e);
                                                        }
                                                    },
                                                    result = server_to_client => {
                                                        if let Err(e) = result {
                                                            error!("Server to client copy error: {}", e);
                                                        }
                                                    },
                                                    _ = shutdown_tx.subscribe().recv() => {
                                                        info!("Shutting down connection");
                                                    }
                                                }
                                            } else {
                                                error!("Failed to send target address to proxy");
                                            }
                                        }
                                        Err(e) => error!("Failed to connect to proxy: {}", e),
                                    }
                                });
                            }
                            Err(e) => error!("Failed to accept connection: {}", e),
                        }
                    },
                    _ = shutdown_tx.subscribe().recv() => {
                        info!("Shutting down listener");
                        break;
                    }
                }
            }
        }
    });

    Ok(())
}
