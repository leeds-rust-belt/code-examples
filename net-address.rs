use std::net::{Ipv4Addr, Ipv6Addr};

fn main(){
    let ip_v4_addr1 = Ipv4Addr::new(106, 201, 34, 209);
    let ip_v4_addr2 = Ipv4Addr::LOCALHOST;
    println!("IS IP_v4_addr1 a loopback address? {}", ip_v4_addr1.is_loopback());
    println!("IS IP_v4_addr2 a loopback address? {}", ip_v4_addr2.is_loopback());

    let ip_v6_addr1 = Ipv6Addr::new(2001, 0000, 3238, 0xDFE1, 0063, 0000, 0000, 0xFEFB);
    let ip_v6_addr2 = Ipv6Addr::LOCALHOST;
    println!("IPV6 segments {:?}", ip_v6_addr1.segments());
    println!("IPV6 segments for localhost {:?}", ip_v6_addr2.segments());
}
