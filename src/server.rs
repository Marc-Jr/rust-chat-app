use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Function to authenticate the client
fn authenticate(stream: &mut TcpStream) -> Option<String> {
    let mut buffer = Vec::new();
    
    // Read the credentials (username:password) from the client
    match stream.read_to_end(&mut buffer) {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to read from stream: {}", e);
            return None;
        }
    }

    // Convert to a string and trim any extra null characters or whitespace
    let credentials = String::from_utf8_lossy(&buffer).to_string();

    // Debugging print
    println!("Received credentials: {}", credentials);

    // Strip null bytes and extra whitespace from username and password
    let credentials = credentials.trim_end_matches(char::from(0));

    // Split the credentials into username and password
    let mut parts = credentials.splitn(2, ':');
    let username = parts.next();
    let password = parts.next();

    println!("Parsed username: {:?}, password: {:?}", username, password);  // Debug print

    // Simulated user database (could be replaced with a real database)
    let user_db = HashMap::from([
        ("user1", "password1"),
        ("user2", "password2"),
    ]);

    // Handle the case where we don't have a valid username and password
    if let (Some(username), Some(password)) = (username, password) {
        // Check if the username exists and if the password matches
        if let Some(valid_password) = user_db.get(username) {
            if password == *valid_password {
                return Some(username.to_string()); // Return the username if authentication is successful
            }
        }
    }
    None // Return None if authentication fails or if the input was not valid
}

// Handle incoming client messages
fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<HashMap<String, TcpStream>>>) {
    // Authenticate the client first
    match authenticate(&mut stream) {
        Some(username) => {
            println!("Client authenticated as: {}", username);
            
            // If authentication is successful, continue with the regular communication
            let mut buffer = [0; 512];
            let name = username.clone();

            // Lock the mutex before accessing the clients
            {
                let mut clients = clients.lock().unwrap();
                clients.insert(name.clone(), stream.try_clone().unwrap());
            }

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

                // Lock the mutex for broadcasting the message
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
            let mut clients = clients.lock().unwrap();
            clients.remove(&name);
        },
        None => {
            // If authentication fails, send a failure message and disconnect the client
            let msg = "Authentication failed. Disconnecting...\n";
            let _ = stream.write(msg.as_bytes());
            println!("Authentication failed for client.");
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap(); // "0.0.0.0" allows connections from any IP address

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
