use std::io::{BufRead, BufReader, Write};
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::{fs, thread};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                thread::spawn(|| response_to_client(stream));
                //response_to_client(stream);
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
    let http_request: Vec<_> = buf_reader.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    //let start_line = buf_reader.lines().next().expect("Request not found").expect("There was an error on the request");
    let start_line = http_request.get(0).expect("Error while getting the start line");
    println!("Start Line  : {:?}", start_line);
    // let path = start_line.split(" ").find(|&p| p == "/"); Answer of respond to with 404
    let path = start_line.split(" ").find(|&p| p.contains("/"));
    println!("Path : {:?}", path);
    match path {
        Some(p) => {
            if p == "/"{
                let _root_path = start_line.split(" ").find(|&p| p == "/").unwrap();
                let my_response = format!( "{response_200}\r\n\r\n");
                //let _ = stream.write_all( my_response.as_bytes()).expect("Error while responding to client");
                //thread::sleep(Duration::from_secs(30));
                write_response_to_client(&mut stream, my_response);
            }
            else if p.starts_with("/echo/"){
                respond_with_content(&stream, p, response_200);
            }else if p.starts_with("/user-agent"){
                user_agent(&stream, &http_request, response_200);
            }
            else{
                write_response_to_client(&mut stream, response_400.to_string());
            }
        }
        None => {
            write_response_to_client(&mut stream, response_400.to_string());
        }
    }
}

fn write_response_to_client(mut stream: &TcpStream, response: String){
    let _ = stream.write_all(response.as_bytes()).expect("Error while responding to client");
}

fn user_agent(mut stream: &TcpStream, http_request: &Vec<String>, response: &str){
    let user_agent = http_request.iter()
        .find(|&value| value.starts_with("User-Agent")).expect("Could not get the user agent");

    let user_agent_value = user_agent.split(":").nth(1).expect("Could not get the value of the user agent");
    let user_agent_value = user_agent_value.trim_start();
    let length = user_agent_value.len();
    let my_response = format!( "{response}\r\nContent-Type:text/plain\r\nContent-Length:{length}\r\n\r\n{user_agent_value}");
    write_response_to_client(&mut stream, my_response);
}

fn respond_with_content(mut stream: &TcpStream, start_line: &str, response: &str){
    //let random_string_from_client = p.split("/").nth(2).expect("Could not split the request header");
    let random_string = &start_line[6..];
    println!("Random String after echo: {random_string}");
    let length = random_string.len();
    let my_response = format!( "{response}\r\nContent-Type:text/plain\r\nContent-Length:{length}\r\n\r\n{random_string}");
    //let _ = stream.write_all( my_response.as_bytes()).expect("Error while responding to client");
    write_response_to_client(&mut stream,my_response);
}

fn get_a_file(directory_path: String, start_line: String, response_status: &str){
    let dir = fs::read_dir(directory_path).expect("Could not read the directory");
    let filename = &start_line[6..];

    for entry in dir{
        let entry = entry.unwrap();
        if entry.file_name().to_str().unwrap().contains(filename){
            let my_response = format!( "{response_status}\r\nContent-Type:application/octet-stream\r\nContent-Length:{length}\r\n\r\n{random_string}");

        }
    }

}
