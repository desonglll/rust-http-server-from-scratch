use std::{
    io::{Read, Write},
    net::TcpListener,
};
fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:3000").expect("Failed to bind");
    println!("Running on port 3000");
    for stream in connection_listener.incoming() {
        let mut stream = stream.expect("Failed to unwrap stream");
        println!("Connection established");

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).expect("Failed to read");
        stream.write(&mut buffer).expect("Failed to write");
    }
}
