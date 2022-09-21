use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::{context::Context, Middleware};

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
        }
    }

    pub fn handle(&mut self, middlewares: Vec<Middleware>) {
        let mut ctx = Context::new();
        let mut buff = [0; 1024];
        match self.stream.read(&mut buff) {
            Ok(size) => {
                ctx.set_req(&buff, size);
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

        self.stream.write_all(&ctx.build_res()).unwrap();
        self.stream.flush().unwrap();
    }
}
