use std::net::{IpAddr, Ipv4Addr, SocketAddr};
fn main() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 8000);
    println!("Socket address is {}, port is {}", socket.ip(), socket.port());
    println!("is this an ipv6 Socket {}", socket.is_ipv6());
}
