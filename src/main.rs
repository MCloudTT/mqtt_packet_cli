use bytes::BytesMut;
use clap::Parser;
use mqtt_v5::types::Packet;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    //Read packet from command line
    let args = Args::parse();

    //Connect to broker
    let mut stream = TcpStream::connect("127.0.0.1:1883").await.unwrap();

    //Decode packet: hex -> [u8]
    let lower_hex_string = args.raw_packet.to_lowercase();
    let hex_string = lower_hex_string.as_bytes();
    let decoded_buf = hex::decode(hex_string).unwrap();
    println!("Sending: {:?}", &decoded_buf);
    if let Some(send_packet) = decode_packet(&mut decoded_buf.clone()) {
        println!("Sending: {:?}", &send_packet);
    } else {
        println!("Cannot decode input packet");
    }

    //Send packet
    stream.write_all(&decoded_buf).await.unwrap();
    stream.flush().await.unwrap();
    stream.readable().await.unwrap();

    //Read response
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    println!();

    //Display response
    let encoded = hex::encode(&buf[..n]);
    println!("Received: {encoded:?}");
    println!("Received: {:?}", &buf[..n]);
    if let Some(packet) = decode_packet(&mut buf[..n]) {
        println!("Received: {:?}", &packet);
    } else {
        println!("Cannot decode response packet");
    }
}

/// Decode a packet from a buffer
fn decode_packet(buf: &mut [u8]) -> Option<Packet> {
    let mut bytes_buf = BytesMut::new();
    bytes_buf.extend_from_slice(buf);

    match mqtt_v5::decoder::decode_mqtt(&mut bytes_buf, mqtt_v5::types::ProtocolVersion::V500) {
        Ok(packet) => Some(packet.unwrap()),
        Err(e) => {
            println!("Error: {e:?}");
            None
        }
    }
}

#[derive(Parser, Debug)]
pub struct Args {
    raw_packet: String,
}
