use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

mod common;
use common::protobuf::{create_thermostat_state, serialize_message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    let mut msg = create_thermostat_state();
    msg.name = "Bedroom".to_string();
    msg.air_temp = 17.5;
    msg.rad_temp = 22.6;

    match serialize_message(&msg) {
        Ok(buf) => stream.write_all(&buf[0..]).await?,
        Err(e) => println!("error = {}", e),
    }

    Ok(())
}
