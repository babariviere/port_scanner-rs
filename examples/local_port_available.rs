extern crate port_scanner;

use port_scanner::local_port_available;

fn main() {
    println!("Port 8000 is available? {}", local_port_available(8000));
}
