use std::fs;
use std::io::Write;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn main() {
    match TcpListener::bind("127.0.0.1:9888") {
        Ok(listener) => {
            for res in listener.incoming() {
                match res {
                    Ok(stream) => {
                        handle_connection(stream);
                    }
                    Err(_) => panic!("create connection failed!"),
                }
            }
        }
        Err(_) => panic!("create server failed!"),
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            let get = b"GET / HTTP/1.1\r\n";

            let (status_line, filename) = if buffer.starts_with(get) {
                ("HTTP/1.1 200 OK\r\n\r\n", "example/hello.html")
            } else {
                ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "example/404.html")
            };
            let contents = fs::read_to_string(filename).unwrap();

            // Write response back to the stream,
            // and flush the stream to ensure the response is sent back to the client
            let response = format!("{status_line}{contents}");

            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(_) => println!("connection read failed!"),
    }
}
