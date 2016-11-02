# A simple port scanner to find which port is open

There is 6 functions provided

## scan_port

It is used to scan a single port and see if it's open.

### Example

```rust
use port_scanner::scan_port;
println!("Port 8000 open? {}", scan_port(8000));
```

## scan_port_addr

It is used to scan a single port with a specific ip and see if it's open like the previous one.

### Example

```rust
use port_scanner::scan_port_addr;
println!("IP 192.168.1.1 has port 8000 open? {}", scan_port_addr("192.168.1.1:8000"));
```

## scan_ports

This function scan all ports specified and return all open ports.

### Example

```rust
use port_scanner::scan_ports;
for open_port in scan_ports(vec![8000, 9000, 8888]) {
    println!("Port {} is open", open_port);
}
```

## scan_ports_addrs

This function scan ports of all addresses and return a list of all addresses with open port.

### Example

```rust
use port_scanner::scan_ports_addrs;
for open_addr in scan_ports_addrs(vec!["192.168.1.1:8000", "192.168.1.2:8000"]) {
    println!("IP {} has port {} open.", open_addr.ip(), open_addr.port());
}
```

## scan_ports_range

This function is like scan_ports but it scan a range of ports.

### Example

```rust
use port_scanner::scan_ports_range;
for open_port in scan_ports_range(8000..9000) {
    println!("Port {} is open", open_port);
}
```

## request_open_port

This function ask to the os and open port and return it.

### Example

```rust
use port_scanner::request_open_port;
println!("Port {}", request_open_port().unwrap_or(0));
```
