use tokio::net::UdpSocket;
use std::error::Error;
use log::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    
    let addr = "127.0.0.1:8080";
    let socket = UdpSocket::bind(addr).await?;
    info!("UDP server listening on {}", addr);

    let mut buf = vec![0u8; 4096];
    
    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                info!("Received {} bytes from {}", len, addr);
                
                // Echo the data back to the client
                if let Err(e) = socket.send_to(&buf[..len], addr).await {
                    error!("Failed to send response: {}", e);
                }
            }
            Err(e) => {
                error!("Failed to receive data: {}", e);
            }
        }
    }
}
