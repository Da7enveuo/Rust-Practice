use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;
use std::fs::File;

fn handle_read(mut stream: &TcpStream) -> String {
    let mut buf = [0u8 ;4096];
    let mut fp = String::from("");
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
            fp = match_route((&*req_str.to_string()).into());
            
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
    return fp
}

// I want to use this function to match the beginning string against requests paths and methods.
// I want further functions to do authentication
fn match_route(req: String) -> String {
    // want to split the request to just get the first line, aka the GET /{filepath} HTTP/1.1, so split by \r\n string
    let path = req.split("\r\n").collect::<Vec<&str>>();
    let fl = path[0];
    let mut file = String::from("");
    match fl {
        "GET / HTTP/1.1" | "GET /index.html HTTP/1.1" =>  file = String::from("index.html"),
        "GET /index.css HTTP/1.1" => file =  String::from("index.css"),
        "GET /favicon.ico HTTP/1.1" => file =  String::from("pic.png"),
        _ => {},
    };
    
    return file
}

/*
pub struct Http_Request{
    Method: String, 
    Host:String,
    Port:String,
    Connection: String,
    DNT: f32,
    Referrer: String,
    Path: String,
    UserAgent: String,
    Accept:<vec>String,
    SecFetchMode:String,
    URLParams: String,
    JSONParams: String,
    ClientPlatform: String
}

should implement these functions

impl Server {


}

impl Router {
    <vec>String,fn()
    AddRoute()
    DelRoute()
    AddBlockList()
    DenyBlockList()
    
}

*/

fn handle_write(mut stream: TcpStream, fp: String) {
    // let route = warp::path("/").and(warp::fs::dir("www/static"));
    let filepath = format!("C:/Users/davey/Downloads/FrontendJs-PhishWorkz/{}", fp);
    let mut file = File::open(filepath).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}\r\n", contents);
    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    let mut fp = String::from("");
    fp = handle_read(&stream);
    handle_write(stream, fp);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening for connections on port {}", 8080);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}