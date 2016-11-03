extern crate port_scanner;

use port_scanner::local_ports_available_range;

fn main() {
    for port in local_ports_available_range(8000..8800) {
        println!("Port {} is available", port);
    }
}
