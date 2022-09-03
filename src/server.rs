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
        println!("server started on: {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
            Ok((mut stream, client)) => {
                println!("socket: {:?}\nclient: {:?}", stream, client);
                let mut buffer: [u8; 1024] = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(_) => println!("received request: {}", String::from_utf8_lossy(&buffer)),
                    Err(e) => println!("failed to read from TcpStream: {:?}", e)

                }
            },
            Err(e) => println!("failed to establish a connection: {:?}", e),
            }
        }
    }
}
