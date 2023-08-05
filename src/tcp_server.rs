use clap::Parser;
use std::str::from_utf8;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::error;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read, stdin, stdout};

type DynResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Parser)]
struct Arguments {
    /// Address to bind the server to [address:port]
    bind_address: String,
}

enum Message {
    ReceivedData,
    ConnectionClosed,
}

fn read_client(mut connection: TcpStream) -> DynResult<()> {
    let mut client_data = [0 as u8; 1024];

    loop {
        let bytes_read = connection.read(&mut client_data)?;
        if bytes_read == 0 {
            // Client closed the connection
            return Ok(())
        }

        let client_data = from_utf8(&client_data)?;
        println!("Client data: {}", client_data);
    }
}

fn write_client(mut connection: TcpStream, rx: &Receiver<Message>, user_text: &String) -> DynResult<()> {
    let data = user_text.as_bytes();

    loop {
        let msg = rx.recv()?;
        if let Message::ConnectionClosed = msg {
            return Ok(())
        }

        // Write TCP stream
        connection.write(data)?;
    }
}

fn main() {
    // Parse arguments
    let args = Arguments::parse();

    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    // Get user input
    let stdin = stdin();
    let mut stdout = stdout();
    let mut user_text = String::new();

    print!("Input some text to send: ");
    stdout.flush().unwrap();
    stdin.read_line(&mut user_text).unwrap();
    user_text.truncate(1023);
    user_text.push('\n');

    println!("Press ENTER to send text to client");

    // Spawn thread which listens for user input
    let input_tx = tx.clone();
    thread::spawn(move || {
        let mut unused_input = String::new();

        loop {
            // Wait for enter press
            stdin.read_line(&mut unused_input).unwrap();
            unused_input.clear();

            // Notify sending thread using channel
            if let Err(_) = input_tx.send(Message::ReceivedData) {
                return
            }
        }
    });

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
                connection.set_read_timeout(None).unwrap();

                // Create separate read thread
                let read_conn = connection.try_clone().unwrap();
                let read_tx = tx.clone();
                let read_thread = thread::spawn(move || {
                    if let Err(err) = read_client(read_conn) {
                        println!("Error reading client data: {}", err);
                    }

                    // Let the sending thread know reading has finished
                    read_tx.send(Message::ConnectionClosed).unwrap();
                });

                // Write to client on main thread
                if let Err(err) = write_client(connection, &rx, &user_text) {
                    println!("Error writing to client: {}", err);
                }

                read_thread.join().unwrap();

                println!("Client {} closed the connection", client_addr);
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
