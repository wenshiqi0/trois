use crate::req_parser::RequestParser;

#[derive(Clone)]
pub struct Request {
    pub raw_str: Option<String>,
    pub hostname: Option<String>,
    pub path: Option<String>,
    pub method: Option<String>,
    pub protocol: Option<String>,
}

impl Request {
    pub fn new() -> Request {
        Request {
            raw_str: None,
            hostname: None,
            path: None,
            method: None,
            protocol: None,
        }
    }

    pub fn parse(&mut self, req_buff: &[u8], size: usize) {
        let mut parser = RequestParser::new(req_buff, size);
        loop {
            match parser.parse_next(self) {
                Ok(_) => (),
                Err(_) => break,
            }
        }
    }

    pub fn set_field_value(&mut self, _field: String, _value: String) {
        // todo
        // println!("{} {}", field, value);
    }
}
