```markdown
# 🦀 Rust TCP Server

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

A foundational, single-threaded TCP server built in Rust. This project was created to demystify low-level networking by implementing a core protocol from scratch using only the Rust standard library. It serves as a practical introduction to systems programming concepts like socket handling, byte buffers, and the request-response cycle.

## 🎯 Project Goals

This project was designed to provide a deep, hands-on understanding of:

- **Fundamental TCP/IP Concepts:** Connection establishment, data streaming, and socket lifecycles.
- **Rust's Core Networking Primitives:** Working directly with `TcpListener` and `TcpStream`.
- **Low-Level I/O:** Managing fixed-size byte buffers (`[u8; 1024]`) and performing read/write operations.
- **Core Rust Mechanics:** Applying ownership, mutable borrowing (`&mut`), error handling with `Result`, and pattern matching (`match`).

## ✨ Features

- **Single-Client Focus:** Handles one client connection at a time, making the request lifecycle easy to trace and understand.
- **Echo Protocol:** Receives a message from a client and responds with `You said: [message]`.
- **Console Logging:** Outputs client connection details and received messages to the server console for clear observability.
- **Graceful Error Handling:** Prints connection and read errors to the console without crashing the server on a single failed client.

## 🚀 Quick Start

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) and Cargo (version 1.70 or later).

### Run the Server
1.  **Clone the repository:**
    ```bash
    git clone https://github.com/os0b1/rust-tcp-server-v2.git
    cd rust-tcp-server-v2
    ```

2.  **Build and run the server:**
    ```bash
    cargo run
    ```

3.  **Connect and send a message:**
    Open a new terminal and use `nc` (netcat) to connect:
    ```bash
    nc localhost 8080
    ```
    Type any message and press Enter. The server will respond.

**Example Interaction:**
```text
# In the server terminal:
New client connected: Ok(127.0.0.1:49382)
Received: Hello from Netcat!

# In the netcat client terminal:
Hello from Netcat!
You said: Hello from Netcat!
```

🏗️ Architecture & Workflow

The server's request-handling flow is as follows:

1. Binding: The main function binds a TcpListener to 127.0.0.1:8080.
2. Accepting: It waits in a loop for incoming connections.
3. Handling: On a successful connection, the handle_client function is called.
4. Reading: The function reads the incoming bytes into a fixed-size stack buffer.
5. Processing: It converts the received bytes to a String and logs it.
6. Responding: A response string is formatted, converted to bytes, and written back to the TcpStream.
7. Cleanup: The connection is automatically closed when the stream goes out of scope.

🔧 Technical Stack

· Language: Rust
· Domain: Systems Programming, Networking
· Libraries: Rust Standard Library (std::net, std::io)

🗺️ Future Roadmap

This project is a foundation for exploring more advanced concepts. Planned improvements include:

· Multi-Client Handling: Using threads to handle multiple clients concurrently.
· Asynchronous I/O: Refactoring with Tokio for a non-blocking, high-performance server.
· Custom Protocol: Implementing a simple application-level protocol (e.g., a basic key-value store).
· Comprehensive Error Handling: Replacing unwrap() with robust, context-rich error propagation using anyhow or thiserror.
· Unit & Integration Testing: Adding tests for the server's core logic.

👤 Author

Great Osoba

· GitHub

This project was built as part of a dedicated journey to master systems engineering and Rust programming.

📄 License

This project is open source and available under the MIT License.

```

---

### 🔑 Key Improvements Explained

1.  **Professional Structure & Visuals:** The title, badges, and emojis make it instantly more inviting and organized. Clear section headers allow for quick scanning.
2.  **Focus on "Why" and Concepts:** It highlights the *learning goals* and *core concepts* right at the top. The "Architecture & Workflow" section uses simple language to explain the technical process, which is great for demonstrating your understanding.
3.  **Complete User Guide:** The "Quick Start" includes every command needed to clone, run, and test the project. The "Example Interaction" section clearly shows what to expect, eliminating any confusion for a user.
4.  **Enhanced Technical Credibility:** The "Technical Stack" is clearly defined. The "Future Roadmap" lists specific, relevant technologies, showing you have a clear plan for advancing your skills.
5.  **Polished Professional Details:** It includes a proper license mention, correct formatting of code blocks, and clear attribution, making your project look complete and serious.



