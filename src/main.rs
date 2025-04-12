use std::{
    fs, 
    io::{prelude::*, BufReader}, 
    net::{TcpListener, TcpStream},
    thread
};

fn main() {
    thread::spawn(|| {
        server_init("127.0.0.1:7878");
    });
    thread::spawn(|| {
        server_init("127.0.0.1:7879");
    });
    loop{};
}

fn server_init(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
    match listener.accept() {
        Ok((_socket, addr)) => {
            thread::spawn(|| {
                server_listener(listener);
            });
            println!("new client: {addr:?}\n\n");
        },
        Err(e) => println!("couldn't get client: {e:?}"),
    }
}

fn server_listener(listener: TcpListener) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line: String;
    match buf_reader.lines().next() {
        Some(t) => request_line = t.unwrap(),
        None => {
            print!("Request line error\n");
            return;
        },
    };
    print!("request: {}\n", request_line);
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./res/hello.html"),
        "GET /style.css HTTP/1.1" => ("HTTP/1.1 200 OK", "./res/style.css"),
        "GET /parser.js HTTP/1.1" => ("HTTP/1.1 200 OK", "./res/parser.js"),
        _ => {
            if let Some(aux) = request_line.split_once("/ HTTP/1.1"){
                if let Some(value) = aux.0.split_once('.') {
                    print!("obtained  {} and {} \n", value.0, value.1.trim());
                }
            }
            ("HTTP/1.1 404 NOT FOUND", "./res/404.html")
        },
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    
    stream.write(response.as_bytes()).unwrap();
}