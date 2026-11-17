use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use common::protocol::ChunkServerMessage;
use common::serde::Deserialize;

pub fn start_server(port: u16) {
    let listener = TcpListener::bind(("0.0.0.0", port)).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());
    let mut buf_reader = BufReader::new(&stream);
    match ChunkServerMessage::deserialize(&mut buf_reader) {
        Ok(message) => {
            match message {
                ChunkServerMessage::Join => {
                    println!("ChunkServer joined the cluster");
                }
                _ => {
                    println!("unknown action send by the server")
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to deserialize message: {}", e);
        }
    }
}
