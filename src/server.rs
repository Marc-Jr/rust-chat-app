use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<HashMap<String, TcpStream>>>) {
    let mut buffer = [0; 512];
    let mut name = String::new();
    
    // Reading the client's name
    if let Err(e) = stream.read(&mut buffer) {
        println!("Failed to read from stream: {}", e);
        return;
    }

    name = String::from_utf8_lossy(&buffer).to_string();

    // Lock the clients mutex to insert the new client
    {
        let mut clients = clients.lock().unwrap();
        clients.insert(name.clone(), stream.try_clone().unwrap());
    }

    // Keep listening for incoming messages
    loop {
        let n = match stream.read(&mut buffer) {
            Ok(0) => break, // Connection closed
            Ok(n) => n,
            Err(e) => {
                println!("Failed to read from stream: {}", e);
                break;
            }
        };

        let msg = String::from_utf8_lossy(&buffer[..n]).to_string();
        
        // Lock the mutex only once and use it for the duration of the loop
        {
            let clients = clients.lock().unwrap(); // Lock the mutex for broadcasting
            for (client_name, mut client_stream) in clients.iter() {
                if client_name != &name {
                    let _ = client_stream.write(msg.as_bytes());
                }
            }
        }
    }

    // Client disconnected, remove from the list
    println!("{} disconnected.", name);
    {
        let mut clients = clients.lock().unwrap();
        clients.remove(&name);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let clients = Arc::new(Mutex::new(HashMap::<String, TcpStream>::new()));

    println!("Server started on port 8080");

    // Accept incoming client connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                thread::spawn(move || handle_client(stream, clients));
            }
            Err(e) => println!("Failed to accept client: {}", e),
        }
    }
}
