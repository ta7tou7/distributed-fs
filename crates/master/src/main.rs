mod server;

fn main() {
    // port listening to messages from client
    let server = server::start_server(12000);
    // port listening to messages from chunkserver
    let data_server = server::start_data_server(12001);
    data_server.join().unwrap();
    server.join().unwrap();
}
