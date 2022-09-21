use crate::request::Request;

#[derive(Clone)]
pub struct Response {
    pub code: u16,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new() -> Response {
        Response {
            code: 200,
            body: vec![],
        }
    }

    fn get_status(&self) -> String {
        match self.code {
            200 => "OK".to_owned(),
            _ => "NOT FOUND".to_owned(),
        }
    }

    fn build_status_line(&self, req: &Request) -> String {
        match req.protocol.clone() {
            Some(protocol) => format!("{} {} {}", protocol, self.code, self.get_status()),
            None => "".to_owned(),
        }
    }

    pub fn build(&self, req: &Request) -> Vec<u8> {
        let status_line = self.build_status_line(req);
        let length = self.body.len();
        let status = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
        let data = [status.as_bytes(), &self.body.clone()].concat();
        data
    }
}
