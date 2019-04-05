use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// The `TcpStream` instance keeps track of what data it returns to us internally.
// It might read more data than we asked for 
// and save that data for the next time we ask for data. 
// It therefore needs to be mut because its internal state might change;
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // it will replace the invalid sequence with �, the U+FFFD REPLACEMENT CHARACTER. 
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
