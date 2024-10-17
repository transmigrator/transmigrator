use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

pub async fn create_tunnel(proxy: &str, target: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(proxy).await?;
    stream.write_all(target.as_bytes()).await?;
    let mut buffer = vec![0; 1024];
    stream.read(&mut buffer).await?;
    Ok(())
}