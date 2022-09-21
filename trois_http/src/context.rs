use crate::{request::Request, response::Response};

pub struct Context {
    req: Request,
    res: Response,
}

impl Context {
    pub fn new() -> Context {
        Context {
            req: Request::new(),
            res: Response::new(),
        }
    }

    pub fn set_req(&mut self, req: &[u8; 1024], size: usize) {
        self.req.parse(&req.to_vec(), size);
    }

    pub fn set_body(&mut self, body: &[u8]) {
        self.res.body = body.to_vec();
    }

    pub fn get_raw_req(&self) -> String {
        match self.req.raw_str.clone() {
            Some(res) => res,
            None => "".to_owned(),
        }
    }

    pub fn get_method(&self) -> String {
        match self.req.method.clone() {
            Some(res) => res,
            None => "".to_owned(),
        }
    }

    pub fn get_protocol(&self) -> String {
        match self.req.protocol.clone() {
            Some(res) => res,
            None => "".to_owned(),
        }
    }

    pub fn get_hostname(&self) -> String {
        match self.req.hostname.clone() {
            Some(res) => res,
            None => "".to_owned(),
        }
    }

    pub fn get_path(&self) -> String {
        match self.req.path.clone() {
            Some(res) => res,
            None => "".to_owned(),
        }
    }

    pub fn build_res(&self) -> Vec<u8> {
        self.res.build(&self.req)
    }
}
