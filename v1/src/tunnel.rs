use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use std::net::SocketAddr;

pub async fn create_tunnel(proxy: &str, target: &str) -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let local_addr = listener.local_addr()?;
    println!("Listening on {}", local_addr);

    tokio::spawn(async move {
        loop {
            let (mut inbound, _) = listener.accept().await.unwrap();
            let proxy = proxy.to_string();
            let target = target.to_string();

            tokio::spawn(async move {
                match TcpStream::connect(&proxy).await {
                    Ok(mut outbound) => {
                        if outbound.write_all(target.as_bytes()).await.is_ok() {
                            let (mut ri, mut wi) = inbound.split();
                            let (mut ro, mut wo) = outbound.split();

                            let client_to_server = tokio::io::copy(&mut ri, &mut wo);
                            let server_to_client = tokio::io::copy(&mut ro, &mut wi);

                            tokio::select! {
                                result = client_to_server => {
                                    if let Err(e) = result {
                                        eprintln!("Client to server copy error: {}", e);
                                    }
                                },
                                result = server_to_client => {
                                    if let Err(e) = result {
                                        eprintln!("Server to client copy error: {}", e);
                                    }
                                },
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to connect to proxy: {}", e),
                }
            });
        }
    });

    Ok(())
}
