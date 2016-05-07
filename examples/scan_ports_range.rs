extern crate port_scanner;

use port_scanner::scan_ports_range;

fn main() {
    for open_port in scan_ports_range(8000..9000) {
        println!("Port {} is open", open_port);
    }
}
