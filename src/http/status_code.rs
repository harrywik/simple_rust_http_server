use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn get_phrase(&self) -> &str {
        match self {
            Self::OK => "200 OK",
            Self::BadRequest => "400 BAD REQUEST",
            Self::NotFound => "404 NOT FOUND"
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
