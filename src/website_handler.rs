use std::fs::read_to_string;
use crate::server::Handler;
use crate::http::{Method, Request, Response, StatusCode};

pub struct WebsiteHandler {
    public_path: String,
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, req: &Request) -> Response {
        match req.method() {
            Method::GET => match self.read_file(req.path()) {
                Some(file_string) => Response::new(StatusCode::OK, Some(file_string)),
                None => Response::new(StatusCode::NotFound, None)
            },
            _ => Response::new(StatusCode::BadRequest, None)
        }
    }
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self {
            public_path
        }
    }

    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        read_to_string(path).ok()
    }
}
