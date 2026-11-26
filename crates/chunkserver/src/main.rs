use crate::config::Config;
use common::protocol::chunkserver_master::{ChunkServerMessage, Heartbeat};
use common::send_frame;
use std::io::ErrorKind::{
    BrokenPipe, ConnectionAborted, ConnectionReset, TimedOut, WouldBlock, WriteZero,
};
use std::net::TcpStream;
use std::sync::mpsc;
use std::time::Duration;
use std::{env, thread};

pub mod config;
mod server;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const READ_TIMEOUT: Duration = Duration::from_secs(2);
fn main() {
    let config = Config::build(env::args()).unwrap();
    let host = config.master_host;
    let port = config.master_port;
    let (tx, rx) = mpsc::channel();
    let socket_handler = thread::spawn(move || {
        let mut stream = connect_with_retry(&host, port);
        while let Ok(msg) = rx.recv() {
            loop {
                let res = try_send(&mut stream, &msg);
                match res {
                    Ok(true) => {
                        println!("Succeeded");
                        break;
                    }
                    Ok(false) => {
                        stream = connect_with_retry(&host, port);
                    }
                    Err(e) => {
                        // issue not related to tcp socket, so no need to reconnect
                        println!("Failed to send chunk: {}", e);
                        break;
                    }
                }
            }
        }
    });
    let heartbeat_handler = thread::spawn(move || {
        loop {
            let heartbeat = ChunkServerMessage::HeartBeat(Heartbeat {});
            if tx.send(heartbeat).is_err() {
                println!("Failed to send heartbeat");
                break;
            }
            thread::sleep(HEARTBEAT_INTERVAL);
        }
    });
    server::start_server(config.port);
    heartbeat_handler.join().expect("Failed to join server");
    socket_handler.join().expect("Failed to join server");
}
fn connect_with_retry(host: &str, port: u16) -> TcpStream {
    loop {
        match TcpStream::connect(format!("{}:{}", host, port)) {
            Ok(stream) => {
                let _ = stream.set_write_timeout(Some(READ_TIMEOUT));
                return stream;
            }
            Err(e) => {
                eprintln!("Failed to connect ({e}), retrying in 1s...");
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

fn try_send(stream: &mut TcpStream, msg: &ChunkServerMessage) -> std::io::Result<bool> {
    match send_frame(stream, msg) {
        Ok(_) => Ok(true),
        Err(e) => match e.kind() {
            BrokenPipe | ConnectionReset | ConnectionAborted | WriteZero | TimedOut
            | WouldBlock => Ok(false),
            _ => Err(e),
        },
    }
}
