use std::fs::File;
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
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // HTTP REQUEST FORMAT:
    //
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
    let mut file = File::open("hello.html").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    // HTTP RESPONSE FORMAT:
    //
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let mut file = File::open("404.html").unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}