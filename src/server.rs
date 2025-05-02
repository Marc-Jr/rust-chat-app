use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::auth::authenticate; // Authentication logic
use crate::messaging::handle_messages; // Messaging logic

pub fn start() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let clients = Arc::new(Mutex::new(HashMap::new()));  // Shared state for clients
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

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<HashMap<String, TcpStream>>>) {
    if let Some(username) = authenticate(&mut stream) {  // Authentication logic
        println!("Client authenticated as: {}", username);
        {
            let mut clients = clients.lock().unwrap();
            clients.insert(username.clone(), stream.try_clone().unwrap());
        }
        handle_messages(username, stream, clients);  // Handle messages once authenticated
    } else {
        let _ = stream.write(b"Authentication failed. Disconnecting...\n");
        println!("Authentication failed for client.");
    }
}
