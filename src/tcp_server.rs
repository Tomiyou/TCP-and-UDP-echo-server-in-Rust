use clap::Parser;
use std::str::from_utf8;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::time::Duration;

#[derive(Parser)]
struct Arguments {
    /// Address to bind the server to [address:port]
    bind_address: String,
}

fn handle_client(mut connection: TcpStream) -> Result<(), std::io::Error> {
    let mut client_data = [0 as u8; 1024];
    let ten_millis = Duration::from_millis(10);

    loop {
        let bytes_read = connection.read(&mut client_data)?;
        if bytes_read > 0 {
            println!("Client data: {}", from_utf8(&client_data).unwrap());

            // Sleep some milis
            thread::sleep(ten_millis);

            // Echo everything
            connection.write(&client_data)?;
        }
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
                thread::spawn(move|| {
                    // Connection succeeded
                    let client_addr = connection.peer_addr().unwrap();
                    println!("New connection from {}", client_addr);

                    // Set TCP stream read timeout
                    connection.set_read_timeout(Some(Duration::new(120, 0))).unwrap();

                    let exit = handle_client(connection).unwrap_err();
                    println!("Closed connection to client {} ({})", client_addr, exit);
                });
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
