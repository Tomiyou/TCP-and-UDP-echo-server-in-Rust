# TCP echo server written in Rust
Client sends user input of up to 1024 bytes to the server, which responds with an echo.

## Compilation

Intended for compilation on x86_64 machines. Built for both x86_64 (statically linked) and cross-compiled to armv7 (statically linked).

```
# Compile for x86_64
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu

# Cross-compile to armv7 (using cargo cross/docker)
cargo install cross
cross build --release --target armv7-unknown-linux-gnueabihf
```
