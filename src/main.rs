// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use nom::AsBytes;
use tokio::io::AsyncWriteExt;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                response_with_200(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn response_with_200(mut stream: &TcpStream){
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let _ = stream.write(response.as_bytes()).expect("Error while responding to client");
}
