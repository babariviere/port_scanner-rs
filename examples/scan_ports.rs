extern crate port_scanner;

use port_scanner::scan_ports;

fn main() {
    let open_ports = scan_ports(vec![8000, 8384, 8500, 8654]);
    for open_port in open_ports {
        println!("Port {} is open", open_port);
    }
}
