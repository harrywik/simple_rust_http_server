#![allow(dead_code)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {

    env::set_var("RUST_BACKTRACE", "1");
    let public_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(&mut WebsiteHandler::new(public_path));
}
