use std::net::TcpListener;
use std::ops::Range;

/// Scan a port and return true if it's open
///
/// # Example
/// ```no_run
/// use port_scanner::scan_port;
/// println!("Port 8000 open? {}", scan_port(8000));
/// ```
pub fn scan_port(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Scan specified ports and return a list of all open ports
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

/// Scan a port range and return a list of all open ports
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
