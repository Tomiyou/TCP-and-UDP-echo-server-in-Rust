use clap::Parser;
use std::net::{TcpStream};
use std::io::{Write, stdin, stdout, Read};
use std::str::from_utf8;

#[derive(Parser)]
struct Arguments {
    /// Address of the server [address:port]
    host_address: String,
}

fn main() {
    let mut user_input = String::new();
    let mut response = [0 as u8; 1024];

    // Parse arguments
    let args = Arguments::parse();

    let stdin = stdin();
    let mut stdout = stdout();

    // Connect to server
    let mut stream = TcpStream::connect(&args.host_address).unwrap();
    println!("Successfully connected to server: {}", args.host_address);

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
