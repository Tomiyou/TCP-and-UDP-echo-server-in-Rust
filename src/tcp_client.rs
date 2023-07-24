use clap::Parser;
use std::net::{TcpStream};
use std::io::{Write, stdin, stdout, Read};
use std::str::from_utf8;
use std::thread;
use std::process::exit;

#[derive(Parser)]
struct Arguments {
    /// Address of the server [address:port]
    host_address: String,
}

fn read_server(mut connection: TcpStream) -> Result<(), std::io::Error> {
    let mut client_data = [0 as u8; 1024];

    loop {
        let bytes_read = connection.read(&mut client_data)?;
        if bytes_read > 0 {
            println!("Client data: {}", from_utf8(&client_data).unwrap());
        } else {
            println!("Server closed the connection");
            exit(0);
        }
    }
}

fn write_server(mut connection: TcpStream) -> Result<(), std::io::Error> {
    let stdin = stdin();
    let mut stdout = stdout();

    print!("Input some text to send: ");
    stdout.flush().unwrap();

    let mut user_text = String::new();
    stdin.read_line(&mut user_text).unwrap();
    user_text.truncate(1023);
    user_text.push('\n');

    println!("Press ENTER to send text to server");

    let mut unused_input = String::new();
    loop {
        // Wait for enter press
        stdin.read_line(&mut unused_input).unwrap();

        // Write TCP stream
        connection.write(user_text.as_bytes())?;
    }
}

fn main() {
    // Parse arguments
    let args = Arguments::parse();

    // Connect to server
    let connection = TcpStream::connect(&args.host_address).unwrap();
    println!("Successfully connected to server: {}", args.host_address);

    // Create separate read/write handles
    let read_conn = connection.try_clone().unwrap();
    let write_conn = connection;

    let read_thread = thread::spawn(move|| {
        read_server(read_conn).expect("Reading data from client failed")
    });

    let write_thread = thread::spawn(move|| {
        write_server(write_conn).expect("Writing data to client failed")
    });

    read_thread.join().unwrap();
    write_thread.join().unwrap();
    println!("Closed connection to server {}", args.host_address);
}
