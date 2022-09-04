use crate::server::Handler;
use crate::http::{Method, Request, Response, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, req: &Request) -> Response {
        match req.method() {
            Method::GET => match req.path() {
                "/" => Response::new(StatusCode::OK, Some("<h1>Main Page</h1>".to_string())),
                "/greet" => {
                    Response::new(StatusCode::OK, Some("<h1>Hello!</h1>".to_string()))
                },
                _ => Response::new(StatusCode::NotFound, None)
            },
            _ => Response::new(StatusCode::BadRequest, None)
        }
    }
}
