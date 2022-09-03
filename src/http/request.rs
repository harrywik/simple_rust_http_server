use super::QueryString;
use super::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::{fmt, str};
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buffer: &'buf [u8]) -> Result<Self, Self::Error> {
        let req_str = str::from_utf8(buffer)?;

        let (method, req_str) = get_next_word(req_str).ok_or(ParseError::InvalidRequest)?;
        let (full_path, req_str) = get_next_word(req_str).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(req_str).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol)
        }

        let method = Method::try_from(method)?; 
        let mut path = full_path;
        let mut query_string = None;

        if let Some(i) = full_path.find('?') {
            path = &full_path[..i];
            query_string = Some(QueryString::from(&full_path[i + 1..]));
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(req_str: &str) -> Option<(&str, &str)> {
    for (i, c) in req_str.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&req_str[..i], &req_str[i + 1..]))
        }
    }
    None
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
            Self::InvalidMethod => "Invalid method, please see Method enum in http/method.rs",
            Self::InvalidProtocol => "Only HTTP/1.1 is supported",
            Self::InvalidEncoding => "Only utf8 encoding accepted",
            Self::InvalidRequest => "Invalid request" 
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
