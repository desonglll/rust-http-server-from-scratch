use core::str;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").expect("Failed to connect");
    stream.write("Hello".as_bytes()).expect("Failed to write");
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    println!(
        "Got response from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
