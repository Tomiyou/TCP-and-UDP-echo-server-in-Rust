use std::str::from_utf8;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::time::Duration;

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
    // Parse server address from CLI arguments
    let mut address = String::new();

    let ip = match std::env::args().nth(2) {
        Some(address) => address, 
        None => "0.0.0.0".to_string(),
    };
    let port = match std::env::args().nth(1) {
        Some(port) => port, 
        None => "3333".to_string(),
    };
    address.push_str(ip.as_str());
    address.push(':');
    address.push_str(port.as_str());

    // Start TCP server
    let listener = TcpListener::bind(address).unwrap();

    // Accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
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
