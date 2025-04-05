use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Handle incoming client messages
fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<HashMap<String, TcpStream>>>) {
    let mut buffer = [0; 512];
    let mut name = String::new();
    
    // Reading the client's name
    if let Err(e) = stream.read(&mut buffer) {
        println!("Failed to read from stream: {}", e);
        return;
    }

    name = String::from_utf8_lossy(&buffer).to_string();
    
    let mut clients = clients.lock().unwrap();
    clients.insert(name.clone(), stream.try_clone().unwrap());

    loop {
        let n = match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                println!("Failed to read from stream: {}", e);
                break;
            }
        };

        let msg = String::from_utf8_lossy(&buffer[..n]).to_string();
        for (client_name, client_stream) in clients.iter() {
            if client_name != &name {
                let _ = client_stream.write(msg.as_bytes());
            }
        }
    }

    println!("{} disconnected.", name);
    clients.lock().unwrap().remove(&name);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let clients = Arc::new(Mutex::new(HashMap::<String, TcpStream>::new()));

    println!("Server started on port 8080");

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
