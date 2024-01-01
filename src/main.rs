use std::io::{BufRead, BufReader, Write};
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::{env, fs, thread};
use std::path::Path;

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


fn response_to_client(mut stream: TcpStream) {
    let response_200 = "HTTP/1.1 200 OK";
    let response_400 = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

    let buf_reader = BufReader::new(&mut stream);
    //let start_line = buf_reader.lines().next().expect("Request not found").expect("There was an error on the request");
    println!("Is it coming here ?");
    let http_request: Vec<_> = buf_reader.lines()
        .filter(|value| {
            println!("{:?}", value);
            value.is_ok()
        } )
        .map(|result| {
            if result.is_ok(){
                println!("Able to unwrap");
                result.unwrap()
            }else if result.is_err(){
                println!("UNABLE TO UNWRAP");
                String::new()
            }else{
                println!("SOMETHING ELSE");
                String::new()
            }
        })
        .take_while(|line| {
            println!("Length of the line : {}", line.len());
            !line.is_empty()
        })
        .collect();

    println!("Http Request : {:#?}", http_request);
    //let start_line = buf_reader.lines().next().expect("Request not found").expect("There was an error on the request");
    let start_line = http_request.get(0).expect("Error while getting the start line");
    println!("Start Line  : {:?}", start_line);
    // let path = start_line.split(" ").find(|&p| p == "/"); Answer of respond to with 404
    let p: Vec<&str> = start_line.split(" ").collect();
    //let path = String::new();
    if p[0] == "GET" {
    let path = start_line.split(" ").find(|&p| p.contains("/"));
    println!("Path : {:?}", path);
    match path {
        Some(p) => {
            if p == "/" {
                let _root_path = start_line.split(" ").find(|&p| p == "/").unwrap();
                let my_response = format!("{response_200}\r\n\r\n");
                println!("First if statement");
                //let _ = stream.write_all( my_response.as_bytes()).expect("Error while responding to client");
                //thread::sleep(Duration::from_secs(30));
                write_response_to_client(&mut stream, my_response);
            } else if p.starts_with("/echo/") {
                respond_with_content(&stream, p, response_200);
            } else if p.starts_with("/user-agent") {
                user_agent(&stream, &http_request, response_200);
            } else if p.starts_with("/files/") {
                get_a_file(&stream, &start_line, response_200);
            } else {
                write_response_to_client(&mut stream, response_400.to_string());
            }
        }
        None => {
            write_response_to_client(&mut stream, response_400.to_string());
        }
    }
    }
    else if p[0] == "POST"{
        //let path = start_line.split(" ").nth(1).unwrap();
        //let filename = &path[7..];
        post_a_file(&mut stream, &http_request);
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

fn get_a_file(mut stream: &TcpStream,start_line: &String, response_status: &str){

    let cmd_args: Vec<String> = env::args().collect();
    let directory_path = &cmd_args[2];
    let dir = fs::read_dir(directory_path).expect("Could not read the directory");
    let fname = &start_line[11..];
    let filename = fname.split(" ").nth(0).expect("Could not split the file name");
    println!("File name : {filename}");

    let my_file = dir.map(|result| result.unwrap())
        .find(|entry|  entry.file_name().to_str().unwrap().contains(filename));

    if my_file.is_some(){
        let file_name = my_file.expect("Could not get the file name").file_name().to_str().unwrap().to_string();
        let dir_path = Path::new(directory_path).join(file_name);
        let content = fs::read_to_string(dir_path).expect("Could not read the file");
        let length = content.len();
        let my_response = format!( "{response_status}\r\nContent-Type:application/octet-stream\r\nContent-Length:{length}\r\n\r\n{content}");
        write_response_to_client(&mut stream,my_response);
    }
    else{
        let response_400 = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let my_response = format!( "{response_400}\r\n\r\n");
        write_response_to_client(&mut stream,my_response);
    }

}

fn post_a_file(mut stream: &TcpStream, http_request: &Vec<String>){
    let cmd_args: Vec<String> = env::args().collect();
    let directory_path = &cmd_args[2];
    let start_line = &http_request[0];

    let fname = &start_line[12..];
    let filename = fname.split(" ").nth(0).expect("Could not split the file name");
    println!("File name for POST REQUEST : {filename}");
    let dir_path = Path::new(directory_path).join(filename);

    let content = &http_request[6];

    fs::write(dir_path, content.as_bytes()).expect("Could not write to file");
    let response = "HTTP/1.1 201\r\n\r\n";
    let my_response = format!( "{response}\r\n\r\n");
    write_response_to_client(&mut stream,my_response);
}
