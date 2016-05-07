extern crate port_scanner;

use port_scanner::scan_port;

fn main() {
    let is_open = scan_port(8123);
    match is_open {
        true => println!("Port 8123 is open"),
        false => println!("Port 8123 is close"),
    }
}
