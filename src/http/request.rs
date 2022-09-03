use super::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::{fmt, str};
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let req_str = str::from_utf8(buffer)?;

        unimplemented!();
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidMethod => "invalid method, please see Method enum in http/method.rs",
            Self::InvalidProtocol => "only HTTP/1.1 is supported",
            Self::InvalidEncoding => "only utf8 encoding accepted",
            Self::InvalidRequest => "invalid request" 
        }
    }
}

impl Error for ParseError {}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}
