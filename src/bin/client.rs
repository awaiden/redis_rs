use std::io::{self, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:8080").await {
        Ok(s) => {
            println!("Successfully connected to Mini-Redis server on 127.0.0.1:8080");
            s
        }
        Err(_) => {
            println!("Error: Could not connect to server. Is the server running?");
            return;
        }
    };

    println!("Commands: SET <key> <value> | GET <key> | exit\n");

    let mut input = String::new();
    let mut buf = [0u8; 1024];

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        // Çıkış kontrolü
        if trimmed.to_lowercase() == "exit" {
            println!("Goodbye!");
            break;
        }

        if trimmed.is_empty() {
            continue;
        }

        if let Err(e) = stream.write_all(trimmed.as_bytes()).await {
            println!("Failed to send data to server: {}", e);
            break;
        }

        match stream.read(&mut buf).await {
            Ok(0) => {
                println!("Server closed the connection.");
                break;
            }
            Ok(bytes_read) => {
                let server_response = String::from_utf8_lossy(&buf[..bytes_read]);
                print!("{}", server_response);
            }
            Err(e) => {
                println!("Error reading from server: {}", e);
                break;
            }
        }
    }
}
