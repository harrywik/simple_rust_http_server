use crate::http::{Request, Response, StatusCode, ParseError};
use std::net::TcpListener;
use std::io::Read;


pub trait Handler {
    fn handle_request(&mut self, req: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self, handler: &mut impl Handler) -> () {
        println!("Server started on: {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
            Ok((mut stream, client)) => {
                println!("Socket: {:?}\nClient: {:?}", stream, client);
                let mut buffer: [u8; 1024] = [0; 1024];

                match stream.read(&mut buffer) {
                    Ok(_) => {
                        let res = match Request::try_from(&buffer as &[u8]) {
                            Ok(req) => {
                                handler.handle_request(&req)
                            },
                            Err(e) => {
                                handler.handle_bad_request(&e)
                            }
                        };

                        res.send(&mut stream);

                    }

                    Err(e) => println!("Failed to read from TcpStream: {}", e)
                }
            },
            Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
