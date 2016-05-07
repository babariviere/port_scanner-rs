# A simple port scanner to find which port is open

There is 4 functions provided

# scan_port

It is used to scan a single port and see if it's open.

## Example

```rust
use port_scanner::scan_port;
println!("Port 8000 open? {}", scan_port(8000));
```

# scan_ports

This function scan all ports specified and return all open ports.

## Example

```rust
use port_scanner::scan_ports;
for open_port in scan_ports(vec![8000, 9000, 8888]) {
    println!("Port {} is open", open_port);
}
```

# scan_ports_range

This function is like scan_ports but it scan a range of ports.

## Example

```rust
use port_scanner::scan_ports_range;
for open_port in scan_ports_range(8000..9000) {
    println!("Port {} is open", open_port);
}
```

# request_open_port

This function ask to the os and open port and return it.

## Example

```rust
use port_scanner::request_open_port;
println!("Port {}", request_open_port().unwrap_or(0));
```
