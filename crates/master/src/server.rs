use std::io::{BufReader, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::thread;
use common::protocol::ChunkServerMessage;
use common::serde::Deserialize;

pub fn start_server(port: u16) {
    let listener = TcpListener::bind(("0.0.0.0", port)).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(stream: TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());
    let peer_addr = stream.peer_addr().unwrap();
    let mut buf_reader = BufReader::new(&stream);
    loop {
        match ChunkServerMessage::deserialize(&mut buf_reader) {
            Ok(message) => {
                match message {
                    ChunkServerMessage::Join => {
                        println!("ChunkServer joined the cluster");
                    }
                    ChunkServerMessage::HeartBeat => {
                        println!("ChunkServer heartbeat for the cluster");
                    }
                }
            }
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                println!("ChunkServer {:?} disconnected", peer_addr);
                break;
            }
            Err(e) => {
                eprintln!("Failed to deserialize message: {}", e);
                break;
            }
        }
    }
}
