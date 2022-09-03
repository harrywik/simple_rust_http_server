use crate::http::Request;
use std::net::TcpListener;
use std::io::Read;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) -> () {
        println!("Server started on: {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
            Ok((mut stream, client)) => {
                println!("Socket: {:?}\nClient: {:?}", stream, client);
                let mut buffer: [u8; 1024] = [0; 1024];

                match stream.read(&mut buffer) {
                    Ok(_) => {
                        println!("Received request: {}", String::from_utf8_lossy(&buffer));
                        match Request::try_from(&buffer as &[u8]) {
                            Ok(req) => {

                            },
                            Err(e) => println!("Could not convert buffer into Request. Error: {}", e)
                        }
                    }

                    Err(e) => println!("Failed to read from TcpStream: {}", e)
                }
            },
            Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
