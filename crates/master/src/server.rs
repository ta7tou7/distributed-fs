use common::protocol::chunkserver_master::ChunkServerMessage;
use common::protocol::client_master::{ClientMasterMessage, CreateFileResponse};
use common::threadpool::ThreadPool;
use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::JoinHandle;

pub fn start_data_server(port: u16) -> JoinHandle<()> {
    thread::spawn(move || {
        println!("Starting server on port {}", port);
        let listener = TcpListener::bind(("0.0.0.0", port)).unwrap();
        let pool = ThreadPool::new(2);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(|| {
                handle_data_connection(stream);
            });
        }
    })
}

pub fn start_server(port: u16) -> JoinHandle<()> {
    thread::spawn(move || {
        println!("Starting server on port {}", port);
        let listener = TcpListener::bind(("0.0.0.0", port)).unwrap();
        let pool = ThreadPool::new(1);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(|| {
                handle_connection(stream);
            });
        }
    })
}

fn handle_connection(stream: TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());
    let mut buf_reader = BufReader::new(&stream);
    let mut buf_writer = BufWriter::new(&stream);
    loop {
        let response: ClientMasterMessage = common::receive_frame(&mut buf_reader).unwrap();
        match response {
            ClientMasterMessage::CreateFileRequest(request) => {
                println!("Received create file request: {:?}", request);
                let response = CreateFileResponse {
                    uid: 12,
                    chunk_servers: vec![],
                };
                common::send_frame(
                    &mut buf_writer,
                    &ClientMasterMessage::CreateFileResponse(response),
                )
                .unwrap();
            }
            ClientMasterMessage::CreateFileResponse(response) => {
                println!("Received create file response: {:?}", response);
            }
        }
    }
}

fn handle_data_connection(stream: TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());
    let mut buf_reader = BufReader::new(&stream);
    loop {
        let response: ChunkServerMessage = common::receive_frame(&mut buf_reader).unwrap();
        match response {
            ChunkServerMessage::HeartBeat(msg) => {
                println!("Received heartbeat request: {:?}", msg);
            }
            ChunkServerMessage::CheckPoint(msg) => {
                println!("Received checkpoint: {:?}", msg);
            }
        }
    }
}
