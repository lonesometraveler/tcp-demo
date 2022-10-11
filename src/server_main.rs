use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

mod common;
use common::protobuf::deserialize_message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening: {:?}", listener);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(process_socket(socket));
    }
}

async fn process_socket<T>(mut socket: T)
where
    T: AsyncWriteExt + AsyncReadExt + Unpin,
{
    let mut buf = [0; 1024];

    loop {
        match socket.read(&mut buf).await {
            Ok(n) if n == 0 => {
                // socket closed
                return;
            }
            Ok(n) => match deserialize_message(&buf[0..n]) {
                Ok(decoded) => {
                    println!("received: {decoded}");
                }
                Err(e) => println!("failed to deserialize; err = {:?}", e),
            },
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };
    }
}
