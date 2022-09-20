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

    pub fn set_req(&mut self, req: String) {
        self.req.set_raw_str(&req);
    }

    pub fn get_raw_req(&self) -> String {
        self.req.raw_str.clone()
    }

    pub fn get_hostname(&self) -> String {
        self.req.hostname.clone()
    }

    pub fn get_path(&self) -> String {
        self.req.path.clone()
    }

    pub fn parse_req(&self) {

    }

    pub fn build_res(&self) -> String {
        "".to_owned()
    }
}
