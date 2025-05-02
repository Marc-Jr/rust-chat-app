use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use chrono::Local;  // Add chrono import

pub fn handle_messages(username: String, mut stream: TcpStream, clients: Arc<Mutex<HashMap<String, TcpStream>>>) {
    let mut buffer = [0; 512];

    loop {
        let n = match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };

        let msg = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

        // If it's a command, handle it
        if msg.starts_with('/') {
            match msg.as_str() {
                "/users" => {
                    let users = list_users(&clients);
                    let _ = stream.write(format!("Online Users: {}\n", users).as_bytes());
                }
                "/quit" => {
                    let _ = stream.write(b"Disconnecting...\n");
                    break;  // End the connection
                }
                "/help" => {
                    let help_msg = "/users - List online users\n/quit - Exit chat\n/help - Show available commands\n";
                    let _ = stream.write(help_msg.as_bytes());
                }
                _ => {
                    let _ = stream.write(b"Unknown command. Type /help for a list of commands.\n");
                }
            }
        } else {
            // If not a command, process as a normal message
            // Get current timestamp
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            if msg.starts_with('@') {
                let parts: Vec<&str> = msg.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    let recipient = parts[0].trim_start_matches('@');
                    let message = parts[1];

                    let clients = clients.lock().unwrap();
                    if let Some(mut recipient_stream) = clients.get(recipient) {
                        let full_msg = format!("{} (Private) {}: {}\n", timestamp, username, message);
                        let _ = recipient_stream.write(full_msg.as_bytes());
                    } else {
                        let _ = stream.write(format!("User {} not found.\n", recipient).as_bytes());
                    }
                }
            } else {
                let msg = format!("{} {}: {}\n", timestamp, username, msg);
                let clients = clients.lock().unwrap();
                for (user, mut client_stream) in clients.iter() {
                    if user != &username {
                        let _ = client_stream.write(msg.as_bytes());
                    }
                }
            }
        }
    }

    println!("Client {} disconnected", username);
    clients.lock().unwrap().remove(&username);
}

fn list_users(clients: &Arc<Mutex<HashMap<String, TcpStream>>>) -> String {
    let clients = clients.lock().unwrap();
    clients.keys().cloned().collect::<Vec<String>>().join(", ")
}
