#![allow(dead_code)]

use server::Server;
use std::env;

mod server;
mod http;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
