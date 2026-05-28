# Rust TCP Server

A beginner-friendly TCP server built in Rust to learn low-level networking fundamentals, TCP streams, buffers, and client-server communication.

## What I Learned

This project helped me understand:

- TCP fundamentals
- Client ↔ Server communication
- `TcpListener`
- `TcpStream`
- Buffers
- Reading and writing data
- Error handling with `Result`
- `match` expressions
- Rust ownership and mutable borrowing

## Features

- TCP server running on localhost
- Accepts incoming client connections
- Reads messages from connected clients
- Prints received messages
- Sends responses back to clients

## Project Structure

```text
src/
└── main.rs
```

## Running the Project

Clone the repository:

```bash
git clone <repo-url>
```

Move into the project folder:

```bash
cd rust-tcp-server
```

Run the server:

```bash
cargo run
```

The server will run on:

```text
127.0.0.1:8080
```

## Example Flow

1. Client connects to server
2. Server creates a `TcpStream`
3. Server reads incoming bytes into a buffer
4. Bytes are converted into readable text
5. Server sends a response back

## Tech Stack

- Rust
- TCP Networking
- Rust Standard Library (`std::net`)

## Future Improvements

- Multi-client handling
- Threading
- Async networking with Tokio
- Custom messaging protocol
- Better error handling

## Author

Built by Great Osoba as part of a systems engineering and Rust learning journey.