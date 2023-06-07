use clap::Parser;
use std::str::from_utf8;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read, stdin, stdout};
use std::time::Duration;

#[derive(Parser)]
struct Arguments {
    /// Address to bind the server to [address:port]
    bind_address: String,
}

fn read_client(mut connection: TcpStream) -> Result<(), std::io::Error> {
    let mut client_data = [0 as u8; 1024];

    loop {
        let bytes_read = connection.read(&mut client_data)?;
        if bytes_read > 0 {
            println!("Client data: {}", from_utf8(&client_data).unwrap());
        } else {
            return Ok(())
        }
    }
}

fn write_client(mut connection: TcpStream) -> Result<(), std::io::Error> {
    let stdin = stdin();
    let mut stdout = stdout();

    print!("Input some text to send: ");
    stdout.flush().unwrap();

    let mut user_text = String::new();
    stdin.read_line(&mut user_text).unwrap();

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

    // Start TCP server
    let listener = TcpListener::bind(&args.bind_address).unwrap();

    // Accept connections and process them, spawning a new thread for each one
    println!("Server listening on {}", args.bind_address);
    for connection in listener.incoming() {
        match connection {
            Ok(connection) => {
                // Connection succeeded
                let client_addr = connection.peer_addr().unwrap();
                println!("New connection from {}", client_addr);

                // Set timeout
                connection.set_read_timeout(Some(Duration::new(120, 0))).unwrap();

                // Create separate read/write handles
                let read_conn = connection.try_clone().unwrap();
                let write_conn = connection;

                let read_thread = thread::spawn(move|| {
                    read_client(read_conn).unwrap_err()
                });

                let write_thread = thread::spawn(move|| {
                    write_client(write_conn).unwrap_err()
                });

                let read_exit = read_thread.join().unwrap();
                let write_exit = write_thread.join().unwrap();

                println!("Closed connection to client {} ({} | {})", client_addr, read_exit, write_exit);
            }
            Err(e) => {
                // Connection failed
                println!("Connection closed due to error: {}", e);
            }
        }
    }

    // Close the socket server
    drop(listener);
}
