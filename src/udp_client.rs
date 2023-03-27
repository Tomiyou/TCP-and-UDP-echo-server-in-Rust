use clap::Parser;
use std::io::{stdin, stdout, Write};
use std::net::UdpSocket;

#[derive(Parser)]
struct Arguments {
    /// Port to bind locally
    bind_port: String,

    /// Address of the other client [address:port]
    host_address: String,
}

fn main() -> std::io::Result<()> {
    // Parse server address from CLI arguments
    let stdin = stdin();
    let mut stdout = stdout();

    // Parse arguments
    let args = Arguments::parse();

    let mut bind_address = "[::]:".to_string();
    bind_address.push_str(&args.bind_port);
    println!("Bind port: {}", bind_address);
    let socket = UdpSocket::bind(bind_address)?;

    let host_address = args.host_address;
    println!("Host address: {}", host_address);

    // Get user input
    let mut user_input = String::new();
    print!("Input the text to send: ");
    stdout.flush().unwrap();
    stdin.read_line(&mut user_input).unwrap();
    let user_input = user_input.as_bytes();

    loop {
        // Read user input
        let mut send_count = String::new();
        print!("How many times to send: ");
        stdout.flush().unwrap();

        stdin.read_line(&mut send_count).unwrap();
        send_count.pop();
        let send_count: u32 = send_count.parse().unwrap_or(1);
        println!("Sending {} packets", send_count);

        // Write TCP stream
        for _ in 0..send_count {
            socket
                .send_to(user_input, &host_address)
                .unwrap();
        }
    }
}
