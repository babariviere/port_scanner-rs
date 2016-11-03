# A port scanner library

This is a simple library to scan open port and to scan available port on localhost.

[Documentation](https://docs.rs/port_scanner)

## Features

- Scan port with a different ip.
- Scan available port on localhost.

## Usage

Add `port_scanner` dependency to `Cargo.toml`.

```toml
[depedencies]
port_scanner = "*"
```

Add `extern crate port_scanner;` to your `main.rs` or `lib.rs`.

Now run `cargo build` and you can use the library.
