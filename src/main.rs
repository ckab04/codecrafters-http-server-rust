use std::io::{BufRead, BufReader, Write};
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4222").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                response_to_client(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn response_to_client(mut stream: TcpStream){
    let response_200 = "HTTP/1.1 200 OK";
    let response_400 = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

    let buf_reader = BufReader::new(&mut stream);
    //let start_line = buf_reader.lines().next().expect("Request not found").expect("There was an error on the request");
    let start_line = buf_reader.lines().next().expect("Request not found").expect("There was an error on the request");

    // let path = start_line.split(" ").find(|&p| p == "/"); Answer of respond to with 404
    let path = start_line.split(" ").find(|&p| p.contains("/echo/"));
    println!("Path : {:?}", path);
    match path {
        Some(p) => {
            let random_string_from_client = p.split("/").nth(2).expect("Could not split the request header");
            println!("Random String : {random_string_from_client}");
            let length = random_string_from_client.len();
            let my_response = format!( "{response_200}\r\nContent-Type:text/plain\r\nContent-Length:{length}\r\n\r\n{random_string_from_client}");
            let _ = stream.write_all( my_response.as_bytes()).expect("Error while responding to client");
        }
        None => {
            let _ = stream.write_all(response_400.as_bytes()).expect("Error while responding to client");
        }
    }


}
