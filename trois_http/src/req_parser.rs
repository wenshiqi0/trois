use crate::{
    ascii::{from_u8, ASCII},
    request::Request,
};

pub struct RequestParser {
    first_line: bool,
    index: usize,
    offset: usize,
    size: usize,
    current_field: Option<String>,
    buffer: Vec<u8>,
}

impl RequestParser {
    pub fn new(buff: &[u8], size: usize) -> RequestParser {
        RequestParser {
            first_line: true,
            index: 0,
            offset: 0,
            size,
            current_field: None,
            buffer: buff.to_vec(),
        }
    }

    fn record_raw_req_str(&mut self, req: &mut Request, ch_code: u8) {
        let ch = char::from(ch_code);
        match req.raw_str.clone() {
            Some(mut str) => {
                str.push(ch);
                req.raw_str = Some(str);
            }
            None => {
                let mut init_str = "".to_owned();
                init_str.push(ch);
                req.raw_str = Some(init_str);
            }
        }
    }

    fn parse_first_line(&mut self, req: &mut Request) -> Result<(), String> {
        loop {
            let index = self.index + self.offset;
            let ch_code = self.buffer[index];

            if index == self.size {
                return Err("end".to_owned());
            }

            self.record_raw_req_str(req, ch_code);

            match from_u8(ch_code) {
                Some(ASCII::Whitespace) => {
                    match std::str::from_utf8(&self.buffer[self.index..index]) {
                        Ok(frag) => {
                            let field = frag.trim();
                            match req.method {
                                None => req.method = Some(field.to_owned()),
                                Some(_) => req.path = Some(field.to_owned()),
                            };
                            self.index = index + 1;
                            self.offset = 0;
                        }
                        Err(e) => {
                            return Err(format!("{}", e));
                        }
                    }
                }
                Some(ASCII::Return) => match std::str::from_utf8(&self.buffer[self.index..index]) {
                    Ok(frag) => {
                        let field = frag.trim();
                        req.protocol = Some(field.to_owned());
                        self.first_line = false;
                        self.index = index + 1;
                        self.offset = 0;
                        break;
                    }
                    Err(e) => {
                        return Err(format!("{}", e));
                    }
                },
                None => {
                    return Err("failed to parse request!".to_string());
                }
                _ => {
                    self.offset += 1;
                }
            }
        }

        Ok(())
    }

    fn parse_key_value(&mut self, req: &mut Request) -> Result<(), String> {
        loop {
            let index = self.index + self.offset;
            let ch_code = self.buffer[index];

            if index == self.size {
                return Err("end".to_owned());
            } else {
                self.record_raw_req_str(req, ch_code);

                match from_u8(ch_code) {
                    Some(ASCII::Colon) => {
                        match std::str::from_utf8(&self.buffer[self.index..index]) {
                            Ok(frag) => match self.current_field.clone() {
                                Some(_) => self.offset += 1,
                                None => {
                                    let field = frag.trim();
                                    self.current_field = Some(field.to_owned());
                                    self.index = index + 1;
                                    self.offset = 0;
                                }
                            }
                            Err(e) => {
                                return Err(format!("{}", e));
                            }
                        }
                    }
                    Some(ASCII::Return) => {
                        match std::str::from_utf8(&self.buffer[self.index..index]) {
                            Ok(frag) => match self.current_field.clone() {
                                Some(field) => {
                                    let value = frag.trim();
                                    req.set_field_value(field, value.to_owned());
                                    self.index = index + 1;
                                    self.offset = 0;
                                    self.current_field = None;
                                }
                                None => {
                                    return Err("request format error!".to_owned());
                                }
                            }
                            Err(e) => {
                                return Err(format!("{}", e));
                            }
                        }
                    }
                    _ => self.offset += 1,
                }

                return Ok(());
            }
        }
    }

    pub fn parse_next(&mut self, req: &mut Request) -> Result<(), String> {
        if self.first_line {
            self.parse_first_line(req)
        } else {
            self.parse_key_value(req)
        }
    }
}
