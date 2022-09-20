#[derive(Clone)]
pub struct Request {
    pub raw_str: String,
    pub hostname: String,
    pub path: String,
}

impl Request {
    pub fn new() -> Request {
        Request {
            raw_str: "".to_owned(),
            hostname: "".to_owned(),
            path: "".to_owned(),
        }
    }

    pub fn set_raw_str(&mut self, req_str: &str) {
        self.raw_str = req_str.to_owned();
    }
}
