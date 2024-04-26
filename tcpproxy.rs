use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::thread;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Please provide proxy-from proxy-to addresses");
        exit(2);
    }

    let proxy_server = &args[1];
    let origin_server = &args[2];
    let proxy_listener;
    if let Ok(proxy) = TcpListener::bind(proxy_server) {
        proxy_listener = proxy;
        let addr = proxy_listener.local_addr().unwrap().ip();
        let port = proxy_listener.local_addr().unwrap().port();
        if let Err(_err) = TcpStream::connect(origin_server) {
            println!("Please restart the origin server");
            exit(1);
        }
        println!("Running on addr:{}, Port:{}\n", addr, port);
    } else {
        eprintln!("Unable to bind to specified proxy port");
        exit(1);
    }
    let mut thread_handles = Vec::new();
    for proxy_stream in proxy_listener.incoming() {
        let mut proxy_stream = proxy_stream.expect("Error in incoming TCP connection");
        let mut origin_stream = TcpStream::connect(origin_server).expect("Please restart the origin server");
        let handle = thread::spawn(move || handle_connection(&mut proxy_stream, &mut origin_stream));
        thread_handles.push(handle);
    }
    for handle in thread_handles{
        handle.join().expect("Unable to join child thread");
    }
}

fn handle_connection(proxy_stream: &mut TcpStream, origin_stream: &mut TcpStream) {
    let mut in_buffer: Vec<u8> = vec![0; 1024];
    let mut out_buffer: Vec<u8> = vec![0; 1024];
    if let Err(err) = proxy_stream.read(&mut in_buffer) {
        println!("Error in reading from incoming proxy stream: {}", err);
    } else {
        println!("1: incoming client request:{}", String::from_utf8_lossy(&in_buffer));
    }
    let _ = origin_stream.write(&mut in_buffer).unwrap();
    println!("2: Forwarding request to rigin server\n");
    let _ = origin_stream.read(&mut out_buffer).unwrap();
    println!("3: Received response from origin server: {}", String::from_utf8_lossy(&out_buffer));
    let _ = proxy_stream.write(&mut out_buffer).unwrap();
    println!("4: Forwading response back to client");
}
