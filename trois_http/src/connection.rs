use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::{context::Context, Middleware};

pub struct Connection {
    buffer: [u8; 1024],
    stream: TcpStream,
}

fn get_request(buff: &[u8; 1024], size: usize) -> String {
    let mut raw_req_str: String = "".to_owned();

    for index in 0..=size {
        raw_req_str.push(char::from(buff[index]));
    }

    raw_req_str
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            buffer: [0; 1024],
        }
    }

    pub fn handle(&mut self, middlewares: Vec<Middleware>) {
        let mut ctx = Context::new();
        let buff = &mut self.buffer;

        match self.stream.read(buff) {
            Ok(size) => {
                let req = get_request(buff, size);
                ctx.set_req(req);

                for middleware in middlewares {
                    match middleware(&mut ctx) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("{}", e);
                            break;
                        }
                    }
                }
            }
            Err(_) => (),
        }

        self.stream.write_all(ctx.build_res().as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}
