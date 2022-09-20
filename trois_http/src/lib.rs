use std::net::TcpListener;

use connection::Connection;
use context::Context;

pub mod connection;
pub mod context;
pub mod request;
pub mod response;

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

#[cfg(test)]
mod tests {
    use crate::context::Context;

    use super::Application;

    fn log_raw_req(ctx: &mut Context) -> Result<(), String> {
        println!("{}", ctx.get_raw_req());
        Ok(())
    }

    fn log_host(ctx: &mut Context) -> Result<(), String> {
        println!("hostname: {}", ctx.get_hostname());
        Ok(())
    }

    fn log_path(ctx: &mut Context) -> Result<(), String> {
        println!("path: {}", ctx.get_path());
        Ok(())
    }

    #[test]
    fn test_http_server() {
        let app = Application::new();
        match app
            .middleware(log_raw_req)
            .middleware(log_host)
            .middleware(log_path)
            .listen("127.0.0.1:9888")
        {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
    }
}
