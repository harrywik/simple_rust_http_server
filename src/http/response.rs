use std::io::{Result as IoResult, Write};
use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self {
            status_code,
            body,
        }
    }

    pub fn send(&self, writable: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };

        write!(
            writable,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.get_phrase(),
            body
        )
    }
}
