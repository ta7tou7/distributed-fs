use std::net::TcpListener;
pub fn start_server(port: u16) {
    let listener = TcpListener::bind(("0.0.0.0", port)).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!(
            "Chunkserver new connection: {}",
            stream.peer_addr().unwrap()
        );
    }
}
