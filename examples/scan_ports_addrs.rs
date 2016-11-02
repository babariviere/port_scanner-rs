extern crate port_scanner;

use port_scanner::scan_ports_addrs;

fn main() {
    for open_addr in scan_ports_addrs(vec!["192.168.1.1:8000", "192.168.1.2:8000"]) {
        println!("IP {} has port {} open.", open_addr.ip(), open_addr.port());
    }
}
