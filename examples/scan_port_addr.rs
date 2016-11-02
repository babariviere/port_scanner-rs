extern crate port_scanner;

use port_scanner::scan_port_addr;

fn main() {
    println!("IP 192.168.1.1 has port 8000 open? {}",
             scan_port_addr("192.168.1.1:8000"));
}
