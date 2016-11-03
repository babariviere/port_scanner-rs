extern crate port_scanner;

use port_scanner::local_ports_available;

fn main() {
    for available in local_ports_available(vec![8000, 8100]) {
        println!("Port {} is available", available);
    }
}
