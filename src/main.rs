use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream};
fn handle_client(mut stream: TcpStream) {
    println!("New client connected: {:?}", stream.peer_addr());

let mut buffer=[0; 1024];
match stream.read(&mut buffer) {
    Ok(bytes_read) => {
        let message =
         String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Recieved: {}", message);

        let message_string = message.to_string();
        let response = format!("You said: {}",message_string);
        stream.write_all(response.as_bytes()).unwrap();

        stream.write_all(response.as_bytes()).unwrap();
        stream 
        .write_all(b"Server received your message!")
        .unwrap();
    }
    Err(error) => {
    println!("Error reading stream: {}", error);
    }
}
}

fn main() {
    let listener
     =TcpListener::bind("127.0.0.1:8080")
     .unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(error) => {
                println!("Connection failed: {}", error);
            }
        }
    }
}