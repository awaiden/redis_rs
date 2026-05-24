# Mini-Redis

A minimal Redis-like in-memory key-value store implementation in Rust, built with async/await using Tokio.

### IMPORTANT

This project is a learning exercise and is not intended for production use.

## Features

- **Asynchronous TCP Server** - Built with Tokio for high-performance async I/O
- **Key-Value Storage** - In-memory HashMap-based storage with thread-safe access
- **TTL Support** - Optional expiration times for keys
- **Automatic Cleanup** - Background task that removes expired keys every second
- **Interactive Client** - Simple command-line client for testing

## Supported Commands

- `SET <key> <value> [ttl_seconds]` - Store a key-value pair with optional TTL
- `GET <key>` - Retrieve a value by key

## Installation

Make sure you have Rust installed. Then clone and build:

```bash
git clone <repository-url>
cd redis_rs
cargo build --release
```

## Usage

### Starting the Server

```bash
cargo run --bin server
```

The server will start listening on `127.0.0.1:8080`.

### Running the Client

In a separate terminal:

```bash
cargo run --bin client
```

### Example Session

```
> SET mykey hello
OK

> GET mykey
hello

> SET tempkey world 5
OK

> GET tempkey
world

# Wait 5 seconds...

> GET tempkey
-1

> exit
Goodbye!
```

## Architecture

### Server (`server.rs`)

- Binds to TCP port 8080 and accepts multiple concurrent connections
- Uses `Arc<Mutex<HashMap>>` for thread-safe shared state
- Spawns a background cleanup task that runs every second to remove expired keys
- Each client connection is handled in its own async task

### Client (`client.rs`)

- Connects to the server via TCP
- Provides an interactive REPL for sending commands
- Displays server responses in real-time

## Implementation Details

- **Concurrency**: Uses Tokio's async runtime for handling multiple clients simultaneously
- **Expiration**: Keys with TTL are stored with an `Instant` timestamp and cleaned up by a background task
- **Protocol**: Simple text-based protocol with `\r\n` line endings
- **Error Handling**: Returns `-1` for missing keys or invalid commands

## Dependencies

- [Tokio](https://tokio.rs/) - Async runtime for Rust

## Future Improvements

- Add more Redis commands (DEL, EXISTS, INCR, etc.)
- Implement proper Redis RESP protocol
- Add persistence to disk
- Support for data structures (lists, sets, hashes)
- Authentication and access control
- Clustering and replication

## License

MIT
