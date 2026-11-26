use common::protocol::client_master::ClientMasterMessage;
use common::protocol::client_master::CreateFileRequest;
use common::protocol::client_master::CreateFileResponse;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::net::{IpAddr, TcpStream};

pub struct Namespace {
    master_stream: TcpStream,
}

impl Namespace {
    pub fn get(ip_addr: IpAddr, port: u16) -> Namespace {
        let master_stream =
            TcpStream::connect((ip_addr, port)).expect("Failed to connect to Master on port");
        Namespace { master_stream }
    }

    pub fn create_file(&mut self, path: &str) -> Result<CreateFileResponse, Error> {
        let request = CreateFileRequest {
            path: path.to_string(),
            replication: 3,
        };
        common::send_frame(
            &mut self.master_stream,
            &ClientMasterMessage::CreateFileRequest(request),
        )?;
        let response: ClientMasterMessage = common::receive_frame(&mut self.master_stream)?;
        println!("{:?}", response);
        match response {
            ClientMasterMessage::CreateFileResponse(response) => Ok(response),
            _ => Err(Error::new(InvalidData, "invalid response")),
        }
    }

    pub fn open_file(&self, path: &str) -> FileDataOutputStream {
        println!("Opening file {}", path);
        todo!()
    }
}

struct ChunkHandler {
    primary_handler: (IpAddr, u16),
    slave_handlers: Vec<(IpAddr, u16)>,
    primary_stream: Option<FileDataOutputStream>,
}

fn connect(primary: &(IpAddr, u16)) -> FileDataOutputStream {
    let stream = TcpStream::connect(primary).expect("failed to connect to primary handler");
    FileDataOutputStream { handler: stream }
}

pub struct FileDataOutputStream {
    handler: TcpStream,
}

impl FileDataOutputStream {
    pub fn write(&mut self, data: &[u8]) {
        todo!()
    }

    pub fn read(&mut self, data: &mut [u8]) {
        todo!()
    }
}

impl Drop for FileDataOutputStream {
    fn drop(&mut self) {
        todo!()
    }
}

/*
Example of create and then write
    {
        let namespace = Namespace::get(master)
        let out = namespace.create_file(filePath) // out should implement Write Trait
        out.write("content")
     }

Example of reading
{
    let namespace = Namespace::get(master)
    let in = namespace.open(filePath) // in should implement Read Trait
    let content = in.read()
}
 */
