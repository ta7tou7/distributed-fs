use std::io::Write;
use std::net::TcpStream;
use std::{env, thread};
use std::io::ErrorKind::BrokenPipe;
use std::time::Duration;
use common::protocol::ChunkServerMessage;
use common::serde::Serialize;
use crate::config::Config;

mod server;
pub mod config;

fn main() {
    let config = Config::build(env::args()).unwrap();
    thread::spawn(move || {
        let mut connect = TcpStream::connect(format!("{host}:{port}", host=config.master_host, port=config.master_port))
            .expect(format!("Failed to connect to Master on port {port}", port=config.master_port).as_str());
        loop {
            let heart_beat = ChunkServerMessage::HeartBeat;
            let res = heart_beat.serialize(&mut connect);
            match res {
                Ok(_) => println!("Succeeded"),
                Err(e) if e.kind() == BrokenPipe => {
                    println!("Master connection closed");
                    break;
                },
                Err(e) => {
                    println!("failed to serialize chunk server response: {}", e);
                    break;
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    server::start_server(config.port);

}
