use std::{
    fs, 
    io::{prelude::*, BufReader}, 
    net::{TcpListener, TcpStream, UdpSocket},
    thread
};

const PLAYER1: &str = "7878";
const PLAYER2: &str = "7879";

fn main() {
    thread::spawn(|| {
        server_init("0.0.0.0", PLAYER1);
    });
    thread::spawn(|| {
        server_init("0.0.0.0", PLAYER2);
    });
    loop{};
}

fn server_init(ip: &str, port: &str) {
    let addr = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(addr).unwrap();
    match listener.accept() {
        Ok((_socket, addr)) => {
            if port == "7878" {
                thread::spawn(|| {
                    server_listener(listener, &PLAYER1);
                });
            } else {
                thread::spawn(||{
                    server_listener(listener, &PLAYER2);
                });
            }
            println!("new client: {addr:?}\n\n");
        },
        Err(e) => println!("couldn't get client: {e:?}"),
    }
}

fn server_listener(listener: TcpListener, port: &'static &str) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, if *port == "7878" { "./res/player1.html" } else { "./res/player2.html" });
    }
}

fn handle_connection(stream: TcpStream, filename: &str) {
    let buf_reader = BufReader::new(&stream);
    let request_line: String;
    match buf_reader.lines().next() {
        Some(t) => request_line = t.unwrap(),
        None => {
            print!("Request line error\n");
            return;
        },
    };
    // print!("request: {}\n", request_line);
    match &request_line[..] {
        "GET / HTTP/1.1" => tcp_respond("HTTP/1.1 200 OK", filename, &stream),
        "GET /style.css HTTP/1.1" => tcp_respond("HTTP/1.1 200 OK", "./res/style.css", &stream),
        "GET /parser.js HTTP/1.1" => tcp_respond("HTTP/1.1 200 OK", "./res/parser.js", &stream),
        _ => {
            if let Some(aux) = request_line.split_once("/ HTTP/1.1"){
                if let Some(value) = aux.0.split_once('.') {
                    // print!("obtained  {} and {} \n", &value.0, &value.1.trim());
                    let udp_socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
                    udp_socket.connect("10.42.0.113:7880").expect("connect function failed");
                    
                    udp_socket.send(format!("{} {}", value.0, value.1).as_bytes()).unwrap();
                }
            }
        },
    };
}

fn tcp_respond(status_line: &str, filename: &str, mut stream: &TcpStream) {
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    
    // print!("responded with {response}");
    
    stream.write(response.as_bytes()).unwrap();

}