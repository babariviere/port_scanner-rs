use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::ops::Range;

/// Scan a port and return true if it's open.
///
/// # Example
/// ```no_run
/// use port_scanner::scan_port;
/// println!("Port 8000 open? {}", scan_port(8000));
/// ```
pub fn scan_port(port: u16) -> bool {
    match TcpStream::connect(("0.0.0.0", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Scan port of an address and return true if it's open.
///
/// # Example
/// ```no_run
/// use port_scanner::scan_port_addr;
/// println!("IP 192.168.1.1 has port 8000 open? {}", scan_port_addr("192.168.1.1:8000"));
/// ```
pub fn scan_port_addr<A: ToSocketAddrs>(addr: A) -> bool {
    match TcpStream::connect(addr) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Scan specified ports and return a list of all open ports.
///
/// # Example
/// ```no_run
/// use port_scanner::scan_ports;
/// for open_port in scan_ports(vec![8000, 9000, 8888]) {
///     println!("Port {} is open", open_port);
/// }
/// ```
pub fn scan_ports(ports: Vec<u16>) -> Vec<u16> {
    let mut open_ports = Vec::new();
    for port in ports {
        if scan_port(port) {
            open_ports.push(port);
        }
    }
    open_ports
}

/// Scan specified ports of addresses and return a list of all addresses with open ports.
///
/// # Example
/// ```no_run
/// use port_scanner::scan_ports_addrs;
/// for open_addr in scan_ports_addrs(vec!["192.168.1.1:8000", "192.168.1.2:8000"]) {
///     println!("IP {} has port {} open.", open_addr.ip(), open_addr.port());
/// }
/// ```
pub fn scan_ports_addrs<A: ToSocketAddrs>(addrs: Vec<A>) -> Vec<SocketAddr> {
    let mut open_addrs = Vec::new();
    for addr in addrs {
        if scan_port_addr(&addr) {
            let addr = addr.to_socket_addrs().unwrap().next().unwrap();
            open_addrs.push(addr);
        }
    }
    open_addrs
}

/// Scan a port range and return a list of all open ports.
///
/// # Example
/// ```no_run
/// use port_scanner::scan_ports_range;
/// for open_port in scan_ports_range(8000..9000) {
///     println!("Port {} is open", open_port);
/// }
/// ```
pub fn scan_ports_range(port_range: Range<u16>) -> Vec<u16> {
    let mut open_ports = Vec::new();
    for port in port_range {
        if scan_port(port) {
            open_ports.push(port);
        }
    }
    open_ports
}

/// Request to the os the next open port.
///
/// # Example
/// ```no_run
/// use port_scanner::request_open_port;
/// println!("Port {}", request_open_port().unwrap_or(0));
/// ```
pub fn request_open_port() -> Option<u16> {
    match TcpListener::bind("0.0.0.0:0") {
        Ok(a) => {
            match a.local_addr() {
                Ok(a) => Some(a.port()),
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}
