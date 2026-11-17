use std::io::Write;
use std::net::TcpStream;
use std::{env, thread};
use std::time::Duration;
use common::protocol::ChunkServerMessage;
use common::serde::Serialize;
use crate::config::Config;

mod server;
pub mod config;

fn main() {
    let config = Config::build(env::args()).unwrap();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut connect = TcpStream::connect(format!("{host}:{port})", host=config.master_host, port=config.master_port))
            .expect("Failed to connect to Master on port 12000");
        let join = ChunkServerMessage::Join;
        let res = join.serialize(&mut connect);
        match res {
            Ok(_) => println!("Succeeded"),
            Err(e) => {
                println!("failed to serialize chunk server response: {}", e);
            }
        }
        connect.flush().unwrap();
    });
    server::start_server(config.port);

}
