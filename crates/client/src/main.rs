use crate::client::Namespace;
use std::net::{IpAddr, Ipv4Addr};

pub mod client;

fn main() {
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let mut ns = Namespace::get(localhost_v4, 12000);
    ns.create_file("/hello.txt").unwrap();
    println!("Hello, world!");
}
