use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream, UdpSocket}, str::from_utf8, thread
};

const PLAYER1: &str = "7878";
const PLAYER2: &str = "7879";
static mut LAST: u8 = 0;
static mut CURRENT: u8 = 0;
fn main() {
    thread::spawn(|| {
        server_init("0.0.0.0", PLAYER1);
    });
    thread::spawn(|| {
        server_init("0.0.0.0", PLAYER2);
    });
    thread::spawn(|| {
        udp_receive_loop(); // Add this line to start receiving UDP data on port 7881
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

fn udp_receive_loop() {
    let socket = UdpSocket::bind("0.0.0.0:7881").expect("could not bind to UDP port 7881");
    println!("Listening for UDP packets on port 7881...");

    let mut buf = [0; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                println!("Received {}", &buf[0]);
                unsafe {
                    if *&buf[0] != CURRENT {
                        LAST = CURRENT;
                        CURRENT = *&buf[0];
                    }
                }
            }
            Err(e) => {
                eprintln!("UDP receive error: {}", e);
            }
        }
    }
}


fn server_listener(listener: TcpListener, port: &'static &str) {
    let udp_socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    udp_socket.connect("10.42.0.113:7880").expect("connect function failed");
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let file_to_serve = unsafe {
            match port {
                &"7878" => {
                    match CURRENT {
                        0 => "./res/menu.html",
                        1 => "./res/snake-1.html",
                        2 => "./res/spaceinvaders-1.html",
                        3 => "./res/sokoban-1.html",
                        4 => "./res/breakout-1.html",    // replace with atari
                        _ => "./res/menu.html",
                    }
                }
                &"7879" => {
                    match CURRENT {
                        0 => "./res/menu.html",
                        1 => "./res/snake-2.html",
                        2 => "./res/spaceinvaders-2.html",
                        3 => "./res/sokoban-2.html",
                        4 => "./res/breakout-2.html",
                        _ => "./res/menu.html",
                    }
                }
                _ => {
                   match CURRENT {
                        0 => "./res/menu.html",
                        1 => "./res/snake-1.html",
                        2 => "./res/spaceinvaders-1.html",
                        3 => "./res/sokoban-1.html",
                        4 => "./res/breakout-1.html",
                        _ => "./res/menu.html",
                    } 
                }

            }
        };
        handle_connection(stream, file_to_serve, &udp_socket);

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
        "GET /state HTTP/1.1" => {
            // Respond with current state as plain text
            unsafe {
                if LAST != CURRENT {
                    let curr = unsafe { CURRENT }; // or use AtomicU8 if you switched
                    LAST = CURRENT;
                    let state_str = curr.to_string();
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                        state_str.len(),
                        state_str
                    );
                    send_state(response, &stream);
                }
            }
        }
        "GET /style.css HTTP/1.1" => tcp_respond("HTTP/1.1 200 OK", "./res/style.css", &stream),
        "GET /universal_parser.js HTTP/1.1" => tcp_respond("HTTP/1.1 200 OK", "./res/universal_parser.js", &stream),
        "GET /slow_parser.js HTTP/1.1" => tcp_respond("HTTP/1.1 200 OK", "./res/slow_parser.js", &stream),
        "GET /breakout_parser.js HTTP/1.1" => tcp_respond("HTTP/1.1 200 OK", "./res/breakout_parser.js", &stream),
        "GET /sokoban_parser.js HTTP/1.1" => tcp_respond("HTTP/1.1 200 OK", "./res/sokoban_parser.js", &stream),
        "GET /poll.js HTTP/1.1" => tcp_respond("HTTP/1.1 200 OK", "./res/poll.js", &stream),
        _ => {
            tcp_respond("HTTP/1.1 200 OK", filename, &stream);
            if let Some(aux) = request_line.split_once("/ HTTP/1.1"){
                let value = aux.0.trim();
                print!("obtained  [{}]\n", value);
                match value {
                    "Up" => udp_socket.send("w".as_bytes()).unwrap(),
                    "Down" => udp_socket.send("s".as_bytes()).unwrap(),
                    "Left" => udp_socket.send("a".as_bytes()).unwrap(),
                    "Right" => udp_socket.send("d".as_bytes()).unwrap(),
                    "Left_Shoot" => udp_socket.send("g".as_bytes()).unwrap(),
                    "Right_Shoot" => udp_socket.send("f".as_bytes()).unwrap(),
                    "Up2" => udp_socket.send("u".as_bytes()).unwrap(),
                    "Down2" => udp_socket.send("j".as_bytes()).unwrap(),
                    "Left2" => udp_socket.send("h".as_bytes()).unwrap(),
                    "Right2" => udp_socket.send("k".as_bytes()).unwrap(),
                    "Left2_Shoot" => udp_socket.send("p".as_bytes()).unwrap(),
                    "Right2_Shoot" => udp_socket.send("o".as_bytes()).unwrap(),
                    "LeftLeft" => udp_socket.send("1".as_bytes()).unwrap(),
                    "RightLeft" => udp_socket.send("2".as_bytes()).unwrap(),
                    "LeftRight" => udp_socket.send("3".as_bytes()).unwrap(),
                    "RightRight" => udp_socket.send("4".as_bytes()).unwrap(),
                    "Select" => udp_socket.send("e".as_bytes()).unwrap(),
                    "Back" => udp_socket.send("q".as_bytes()).unwrap(),
                    _ => 0,
                };
            }
        },
    };
}

fn send_state(response: String, mut stream: &TcpStream) {
    stream.write(response.as_bytes()).unwrap();
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