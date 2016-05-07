extern crate port_scanner;
use port_scanner::request_open_port;

fn main() {
    println!("Port {}", request_open_port().unwrap_or(0));
}
