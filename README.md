# TCP and UDP text client/server
A simple implementation of TCP/UDP client and server, where both can send and receive user entered text. Used for debugging Linux network stack.

## Compilation

Intended for compilation on x86_64 machines. Built for both x86_64 (statically linked) and cross-compiled to armv7 (statically linked).

1. Install `cross` for cross compilation (uses docker under the hood)
```
cargo install cross
```

2. Build binaries
```
# Compile for x86_64
cargo build --release --target x86_64-unknown-linux-musl

# Cross-compile to armv7 (Raspbery PI)
cross build --release --target armv7-unknown-linux-musleabihf
```
