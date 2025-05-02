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
    let udp_socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    udp_socket.connect("10.42.0.113:7880").expect("connect function failed");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, if *port == "7878" { "./res/player1.html" } else { "./res/player2.html" }, &udp_socket);
    }
}

fn handle_connection(stream: TcpStream, filename: &str, udp_socket: &UdpSocket) {
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
    match &request_line[..] {
        "GET / HTTP/1.1" => tcp_respond("HTTP/1.1 200 OK", filename, &stream),
        "GET /style.css HTTP/1.1" => tcp_respond("HTTP/1.1 200 OK", "./res/style.css", &stream),
        "GET /parser1.js HTTP/1.1" => tcp_respond("HTTP/1.1 200 OK", "./res/parser1.js", &stream),
        "GET /parser2.js HTTP/1.1" => tcp_respond("HTTP/1.1 200 OK", "./res/parser2.js", &stream),
        _ => {
            tcp_respond("kkkkk", filename, &stream);
            if let Some(aux) = request_line.split_once("/ HTTP/1.1"){
                let value = aux.0.trim();
                print!("obtained  [{}]\n", value);
                match value {
                    "Up" => udp_socket.send("w".as_bytes()).unwrap(),
                    "Down" => udp_socket.send("s".as_bytes()).unwrap(),
                    "Left" => udp_socket.send("a".as_bytes()).unwrap(),
                    "Right" => udp_socket.send("d".as_bytes()).unwrap(),
                    "Up2" => udp_socket.send("u".as_bytes()).unwrap(),
                    "Down2" => udp_socket.send("j".as_bytes()).unwrap(),
                    "Left2" => udp_socket.send("h".as_bytes()).unwrap(),
                    "Right2" => udp_socket.send("k".as_bytes()).unwrap(),
                    "Select" => udp_socket.send("e".as_bytes()).unwrap(),
                    "Back" => udp_socket.send("q".as_bytes()).unwrap(),
                    _ => 0,
                };
            }
        },
    };
}

fn tcp_respond(status_line: &str, filename: &str, mut stream: &TcpStream) {
    print!("reequested file {}\n", filename);
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    
    // print!("responded with {response}");
    
    stream.write(response.as_bytes()).unwrap();

}