use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use tokio::time::{Duration, Instant, interval};

struct RedisValue {
    value: String,
    expires_at: Option<Instant>,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on 127.0.0.1:8080");

    let db = Arc::new(Mutex::new(HashMap::<String, RedisValue>::new()));

    let db_cleaner = Arc::clone(&db);

    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(1));
        loop {
            ticker.tick().await;
            let mut db_lock = db_cleaner.lock().await;
            let now = Instant::now();
            let expired_keys: Vec<String> = db_lock
                .iter()
                .filter(|(_, value)| {
                    value
                        .expires_at
                        .map_or(false, |expires_at| expires_at < now)
                })
                .map(|(key, _)| key.clone())
                .collect();
            for key in expired_keys {
                db_lock.remove(&key);
                println!("[CLEANUP] key: {}", key);
            }
        }
    });

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let db = Arc::clone(&db);

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];

            while let Ok(bytes_read) = socket.read(&mut buf).await {
                if bytes_read == 0 {
                    break;
                }

                let response = String::from_utf8_lossy(&buf[..bytes_read]);
                let args: Vec<&str> = response.trim().split_whitespace().collect();

                if args.is_empty() {
                    continue;
                }

                let method = args[0];
                let mut db_lock = db.lock().await;

                match method.to_uppercase().as_str() {
                    "GET" => {
                        if args.len() < 2 {
                            socket.write_all(b"-1\r\n").await.unwrap();
                            continue;
                        }

                        let key = args[1];
                        let value = db_lock.get(key).map(|v| v.value.clone());

                        match value {
                            Some(value) => {
                                socket.write_all(value.as_bytes()).await.unwrap();
                                socket.write_all(b"\r\n").await.unwrap();
                            }
                            None => {
                                socket.write_all(b"-1\r\n").await.unwrap();
                            }
                        }
                    }
                    "SET" => {
                        if args.len() < 3 {
                            socket.write_all(b"-1\r\n").await.unwrap();
                            continue;
                        }

                        let key = args[1];
                        let value = args[2];

                        let mut expires_at = None;
                        if args.len() > 3 {
                            expires_at = Some(
                                Instant::now() + Duration::from_secs(args[3].parse().unwrap_or(0)),
                            );
                        }

                        db_lock.insert(
                            key.to_string(),
                            RedisValue {
                                value: value.to_string(),
                                expires_at,
                            },
                        );
                        socket.write_all(b"OK\r\n").await.unwrap();
                    }

                    _ => socket.write_all(b"Unknown command\r\n").await.unwrap(),
                }
            }
        });
    }
}
