use clap::Parser;
use std::io::{stdin, stdout, Write, Stdin};
use std::net::{UdpSocket, SocketAddr};
use std::process::exit;
use std::str::from_utf8;
use std::thread;

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
    let user_input = get_user_text(&stdin, args.bind_port);
    let user_input = user_input.as_bytes();

    let recv_socket = socket.try_clone().unwrap();
    thread::spawn(move || {
        let mut buf = [0 as u8; 1024];
        loop {
            let bytes = recv_socket.recv(&mut buf).unwrap();
            if bytes == 0 {
                println!("Recevied 0 bytes!");
                exit(0);
            }

            let text = from_utf8(&buf).unwrap();
            println!("Received from peer: {}", text);
        }
    });

    loop {
        // Read how many times to send
        let mut send_count = String::new();
        print!("Enter how many packets to send (default 1): ");
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

fn get_user_text(stdin: &Stdin, bind_port: String) -> String {
    let mut stdout = stdout();
    let mut user_text = String::new();
    let mut default_text = "socket ".to_string();
    default_text.push_str(&bind_port);

    print!("Input some text to send (default \"{}\"): ", default_text);
    stdout.flush().unwrap();

    // Read a max 1024 character long string
    stdin.read_line(&mut user_text).unwrap();
    let mut user_text = user_text.replace('\n', "");
    user_text.truncate(100);

    if user_text.is_empty() {
        default_text.to_string()
    } else {
        user_text
    }
}
