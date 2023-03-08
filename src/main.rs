use bytes::BytesMut;
use clap::Parser;
use colored::Colorize;
use mqtt_v5::types::Packet;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    //Read arguments from command line
    let args = Args::parse();

    //Connect to broker
    let mut stream = TcpStream::connect(format!("{0}:{1}", args.host, args.port))
        .await
        .unwrap();

    //Send packet
    if let Some(packet) = args.packet {
        send_packet(&mut stream, packet).await;
    } else if let Some(file) = args.file {
        let file = std::fs::read_to_string(file).unwrap();
        // Send packets from file
        for (nr, packet) in file.lines().enumerate() {
            print!("{}", format!("Packet {}: ", nr + 1).yellow());
            match send_packet(&mut stream, packet.to_string()).await{
                Ok(_) => (),
                Err(e) => {println!("{}", format!("Error: {:?}", e).red());
                stream = TcpStream::connect(format!("{0}:{1}", args.host, args.port))
                    .await
                    .unwrap();
                    send_packet(&mut stream, packet.to_string()).await.unwrap();
                }
            }
        }
    } else {
        println!("No packet or file specified");
    }
}

async fn send_packet(stream: &mut TcpStream, packet: String) -> anyhow::Result<()>{
    //Decode packet: hex -> [u8]
    let lower_hex_string = packet.to_lowercase();
    let hex_string = lower_hex_string.as_bytes();
    let decoded_buf = hex::decode(hex_string).unwrap();
    println!("{}", format!("Sending: {:?}", &decoded_buf).green());
    if let Some(send_packet) = decode_packet(&mut decoded_buf.clone()) {
        println!("{}", format!("Sending: {:?}", &send_packet).green());
    } else {
        println!("{}", "Cannot decode input packet".red());
    }

    //Send packet
    match stream.write_all(&decoded_buf).await {
        Ok(_) => println!("{}", "Sent".green()),
        Err(e) => return Err(anyhow::anyhow!("Error: {:?}", e)),
    }
    stream.flush().await.unwrap();

    //Wait for response
    /*
    stream.readable().await;

    //Read response
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await.unwrap();

    //Display response
    let encoded = hex::encode(&buf[..n]);
    println!("{}", format!("\nReceived: {:?}", &buf[..n]).blue());
    if let Some(packet) = decode_packet(&mut buf[..n]) {
        println!("{}", format!("Received: {:?}", &packet).blue());
    } else {
        println!("{}", "Cannot decode response packet".red());
    }*/
    println!("{}", "====================================".black());
    Ok(())
}

/// Decode a packet from a buffer
fn decode_packet(buf: &mut [u8]) -> Option<Packet> {
    let mut bytes_buf = BytesMut::new();
    bytes_buf.extend_from_slice(buf);

    match mqtt_v5::decoder::decode_mqtt(&mut bytes_buf, mqtt_v5::types::ProtocolVersion::V500) {
        Ok(packet) => packet,
        Err(e) => {
            println!("Error: {e:?}");
            None
        }
    }
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    packet: Option<String>,
    #[arg(short, long)]
    file: Option<String>,
    #[arg(long, default_value = "localhost")]
    host: String,
    #[arg(long, default_value = "1883")]
    port: u16,
}
