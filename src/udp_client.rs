use clap::Parser;
use std::io::{stdin, stdout, Write};
use std::net::{UdpSocket, SocketAddr};

#[derive(Parser)]
struct Arguments {
    /// Port to bind locally
    bind_port: String,

    /// Address of the other client [address:port]
    peer_address: String,
}

fn main() -> std::io::Result<()> {
    // Parse arguments
    let args = Arguments::parse();

    let stdin = stdin();
    let mut stdout = stdout();

    // Check bind port is valid
    args.bind_port.parse::<u16>().expect("Bad bind port given");

    // Check peer address is valid
    let peer_address: SocketAddr = args.peer_address
        .parse()
        .expect("Bad peer address given, expected \"ip_addres:port\"");

    // Bind to local address:port
    let mut bind_address = "[::]:".to_string();
    bind_address.push_str(&args.bind_port);
    let socket = UdpSocket::bind(bind_address)?;

    // Get user input
    let mut user_input = String::new();
    print!("Input the text to send: ");
    stdout.flush().unwrap();
    stdin.read_line(&mut user_input).unwrap();
    let user_input = user_input.as_bytes();

    loop {
        // Read how many times to send
        let mut send_count = String::new();
        print!("How many times to send: ");
        stdout.flush().unwrap();

        stdin.read_line(&mut send_count).unwrap();
        send_count.pop();
        let send_count: u32 = send_count.parse().unwrap_or(1);
        println!("Sending {} packets", send_count);

        // Send via UDP stream
        for _ in 0..send_count {
            socket.send_to(user_input, &peer_address).unwrap();
        }
    }
}
