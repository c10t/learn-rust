extern crate web_server;
use web_server::ThreadPool;

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

// `cargo run` example:
// ------
// Worker 0 got a job; executing.
// Shutting down.
// Worker 1 got a job; executing.
// Sending terminate message to all workers.
// Shutting down all workers.
// Shutting down worker 0
// Worker 1 was told to terminate.
// Worker 3 was told to terminate.
// Worker 2 was told to terminate.
// Worker 0 was told to terminate.
// Shutting down worker 1
// Shutting down worker 2
// Shutting down worker 3
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // thread::spawn(|| {
        pool.execute(|| {
          handle_connection(stream);
        });
    }

    println!("Shutting down.");
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
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    // HTTP RESPONSE FORMAT:
    //
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
