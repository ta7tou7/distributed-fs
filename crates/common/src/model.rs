use std::net::IpAddr;

struct Namespace {
    name: String,
}

pub struct Chunk {
    id: u32,
    ip_addr: IpAddr,
    path: String,
    file_identifier: u32,
}

pub struct ChunkHost {
    id: u32,
    ip_addr: IpAddr,
}
