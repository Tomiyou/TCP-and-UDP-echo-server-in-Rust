use clap::Parser;
use std::io::{stdin, stdout, Stdin, Write};
use std::net::{SocketAddr, UdpSocket};
use std::process::exit;
use std::str::from_utf8;
use std::sync::atomic::AtomicBool;
use std::thread;

#[derive(Parser)]
struct Arguments {
    /// Address to bind locally
    bind_address: String,

    /// Address of the other client [address:port]
    peer_address: String,
}

// Atomic newline indicator
static IS_NEWLINE: AtomicBool = AtomicBool::new(true);

fn main() -> std::io::Result<()> {
    // Parse arguments
    let args = Arguments::parse();

    let stdin = stdin();
    let mut stdout = stdout();

    // Check peer address is valid
    let peer_address: SocketAddr = args
        .peer_address
        .parse()
        .expect("Bad peer address given, expected \"ip_addres:port\"");

    // Check bind address is valid
    let bind_address: SocketAddr = args
        .bind_address
        .parse()
        .expect("Bad bind address given, expected \"ip_addres:port\"");

    // Bind to local address:port
    let socket = UdpSocket::bind(bind_address)?;

    // Get user input
    let user_input = get_user_text(&stdin, bind_address.port().to_string());
    let user_input = user_input.as_bytes();

    // Spawn receiving thread
    let recv_socket = socket.try_clone().unwrap();
    thread::spawn(move || {
        let mut buf = [0 as u8; 1024];
        loop {
            let bytes = recv_socket.recv(&mut buf).unwrap();
            if bytes == 0 {
                println!("Recevied 0 bytes!");
                exit(0);
            }

            let data = from_utf8(&buf);
            if let Ok(text) = data {
                let (hours, mins, secs, milis) = get_time();
                if IS_NEWLINE.swap(true, std::sync::atomic::Ordering::Relaxed) {
                    println!("{}:{}:{}.{} - Received from peer: {}", hours, mins, secs, milis, text);
                } else {
                    println!("\n{}:{}:{}.{} - Received from peer: {}", hours, mins, secs, milis, text);
                }
            }
        }
    });

    loop {
        // Read how many times to send
        let mut send_count = String::new();
        print!("Enter how many packets to send (default 1): ");
        IS_NEWLINE.swap(false, std::sync::atomic::Ordering::Relaxed);
        stdout.flush().unwrap();

        stdin.read_line(&mut send_count).unwrap();
        IS_NEWLINE.swap(true, std::sync::atomic::Ordering::Relaxed);

        send_count.pop();
        let send_count: u32 = send_count.parse().unwrap_or(1);
        println!("Sending {} packets", send_count);

        // Send via UDP stream
        for _ in 0..send_count {
            socket.send_to(user_input, &peer_address).unwrap();
        }
    }
}

fn get_time() -> (u8, u8, u8, u16) {
    let current_time = time::OffsetDateTime::now_utc();
    (
        current_time.hour(),
        current_time.minute(),
        current_time.second(),
        current_time.millisecond(),
    )
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
