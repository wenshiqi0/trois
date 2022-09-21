use std::net::TcpListener;

use connection::Connection;
use context::Context;

mod ascii;
pub mod connection;
pub mod context;
pub mod request;
pub mod response;
pub mod req_parser;

pub type Middleware = fn(&mut Context) -> Result<(), String>;

pub struct Application {
    middlewares: Vec<Middleware>,
}

impl Application {
    pub fn new() -> Application {
        Application {
            middlewares: vec![],
        }
    }

    pub fn listen(&mut self, addr: &str) -> Result<String, String> {
        match TcpListener::bind(addr) {
            Ok(listener) => {
                for res in listener.incoming() {
                    match res {
                        Ok(stream) => {
                            let middlewares = self.middlewares.clone();
                            Connection::new(stream).handle(middlewares);
                        }
                        Err(_) => (),
                    }
                }
                Ok("done".to_owned())
            }
            Err(e) => Err(format!("create server failed!\n{e}")),
        }
    }

    pub fn middleware(mut self, func: fn(&mut Context) -> Result<(), String>) -> Self {
        self.middlewares.push(func);
        self
    }
}
