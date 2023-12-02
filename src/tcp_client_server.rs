use clap::{ArgGroup, Parser};
use std::error;
use std::io::{stdin, stdout, Read, Write, Stdin};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::process::exit;
use std::str::from_utf8;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
            ArgGroup::new("mode")
                .required(true)
                .args(&["server", "client"]),
        ))]
struct Arguments {
    /// Start as server bound to address:port
    #[arg(short, long)]
    server: Option<String>,

    /// Start as client bound to address:port
    #[arg(short, long)]
    client: Option<String>,
}

enum TcpEvent {
    ReceivedData,
    ConnectionClosed,
}

type DynResult<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() {
    // Parse CLI arguments
    let args = Arguments::parse();
    let is_server = args.server.is_some();

    // Create snychronization channel
    let (tx, rx): (Sender<TcpEvent>, Receiver<TcpEvent>) = mpsc::channel();

    // Get reference to stdin and stdout
    let stdin = stdin();

    // Get user input
    let user_text = get_user_text(&stdin);

    // Spawn thread which listens for user input
    let write_thread_tx = tx.clone();
    thread::spawn(move || {
        let mut unused_input = String::new();

        loop {
            // Wait for enter press
            stdin.read_line(&mut unused_input).unwrap();
            unused_input.clear();

            // Notify sending thread using channel
            if let Err(_) = write_thread_tx.send(TcpEvent::ReceivedData) {
                return;
            }
        }
    });

    // Start server if --server argument is given
    if let Some(bind_address) = args.server {
        // Check bind address
        let bind_address: SocketAddr = bind_address
            .parse()
            .expect("Bad server bind address given, expected \"--server ip_addres:port\"");

        // Start TCP server
        let listener = TcpListener::bind(&bind_address).unwrap();

        // Accept connections and process them, spawning a new thread for each one
        println!("Server listening on {}", bind_address);
        for connection in listener.incoming() {
            match connection {
                Ok(connection) => {
                    // Connection succeeded
                    let read_thread_tx = tx.clone();
                    start_tcp_stream(connection, read_thread_tx, &rx, &user_text, is_server);
                }
                Err(e) => {
                    // Connection failed
                    println!("Connection closed due to error: {}", e);
                }
            }
        }
    }
    // Start client if --client argument is given
    else if let Some(server_address) = args.client {
        // Check bind address
        let server_address: SocketAddr = server_address
            .parse()
            .expect("Bad server address given, expected \"--client ip_addres:port\"");

        // Connect to server
        let connection = TcpStream::connect(&server_address).unwrap();
        start_tcp_stream(connection, tx, &rx, &user_text, is_server);
    }
}

fn start_tcp_stream(
    connection: TcpStream,
    read_tx: Sender<TcpEvent>,
    rx: &Receiver<TcpEvent>,
    user_text: &String,
    is_server: bool,
) {
    let peer_address = connection.peer_addr().unwrap();

    if is_server {
        println!("Client {} connected", peer_address);
    } else {
        println!("Successfully connected to server {}", peer_address);
    }
    println!("Press ENTER to send text to peer");

    // Set timeout (None means the connection never times out)
    connection.set_read_timeout(None).unwrap();

    // Create separate read thread
    let read_conn = connection.try_clone().unwrap();
    let read_thread = thread::spawn(move || {
        if let Err(err) = read_tcp_stream(read_conn) {
            println!("Error reading {} data: {}", peer_address, err);
        }

        // Let the sending thread know reading has finished
        read_tx.send(TcpEvent::ConnectionClosed).unwrap();
    });

    // Write to client on main thread
    if let Err(err) = write_tcp_stream(connection, &rx, &user_text) {
        println!("Error writing to {}: {}", peer_address, err);
    }

    read_thread.join().unwrap();

    println!("{} {} closed the connection", if is_server { "Server" } else { "Client" }, peer_address);
}

fn read_tcp_stream(mut connection: TcpStream) -> DynResult<()> {
    let mut client_data = [0 as u8; 1024];

    loop {
        let bytes_read = connection.read(&mut client_data)?;
        if bytes_read == 0 {
            // Opposite side closed the connection
            return Ok(());
        }

        let client_data = from_utf8(&client_data)?;
        let (hours, mins, secs, milis) = get_time();
        println!(
            "{}:{}:{}.{} - Client data: {}",
            hours, mins, secs, milis, client_data
        );
    }
}

fn write_tcp_stream(
    mut connection: TcpStream,
    rx: &Receiver<TcpEvent>,
    user_text: &String,
) -> DynResult<()> {
    let data = user_text.as_bytes();

    loop {
        let msg = rx.recv()?;
        if let TcpEvent::ConnectionClosed = msg {
            return Ok(());
        }

        // Write TCP stream
        connection.write(data)?;
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

fn get_user_text(stdin: &Stdin) -> String {
    let mut stdout = stdout();
    let mut user_text = String::new();

    print!("Input some text to send: ");
    stdout.flush().unwrap();

    // Read a max 1024 character long string
    stdin.read_line(&mut user_text).unwrap();
    let mut user_text = user_text.replace('\n', "");
    user_text.truncate(100);
    user_text.push('\n');
    user_text
}
