use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use common::protocol::ChunkServerMessage;
use common::serde::Serialize;
mod server;

fn main() {
    println!("Hello, world!");
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(50));
        let mut connect = TcpStream::connect("localhost:12000")
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
    server::start_server(12001);

}
