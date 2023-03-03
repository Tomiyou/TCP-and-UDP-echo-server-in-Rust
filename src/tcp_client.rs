use std::net::{TcpStream};
use std::io::{Write, stdin, stdout, Read};
use std::str::from_utf8;

fn main() {
    let mut user_input = String::new();
    let mut response = [0 as u8; 1024];

    let stdin = stdin();
    let mut stdout = stdout();

    // Parse server address from CLI arguments
    let mut address = String::new();

    let ip = match std::env::args().nth(2) {
        Some(address) => address, 
        None => "localhost".to_string(),
    };
    let port = match std::env::args().nth(1) {
        Some(port) => port, 
        None => "3333".to_string(),
    };
    address.push_str(ip.as_str());
    address.push(':');
    address.push_str(port.as_str());

    // Connect to server
    let mut stream = TcpStream::connect(address).unwrap();
    println!("Successfully connected to IP {} with port {}", ip, port);

    loop {
        // Read user input
        print!("Input some text: ");
        stdout.flush().unwrap();
        stdin.read_line(&mut user_input).unwrap();

        // Write TCP stream
        stream.write(user_input.as_bytes()).unwrap();
        user_input.truncate(0);

        // Read TCP stream
        println!("Sent, awaiting reply...");
        stream.read(&mut response).unwrap();

        // Server response
        println!("Server response {}", from_utf8(&response).unwrap());
    }
}
