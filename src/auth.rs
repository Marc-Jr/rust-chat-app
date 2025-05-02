use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn authenticate(stream: &mut TcpStream) -> Option<String> {
    let mut buffer = [0; 512];  // Buffer for reading client data
    let n = stream.read(&mut buffer).ok()?;
    if n == 0 {
        return None;  // If no data was read, return None
    }

    let input = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
    let mut parts = input.splitn(2, ':');
    let username = parts.next()?;
    let password = parts.next()?;

    // Hardcoded user database for authentication
    let user_db = HashMap::from([
        ("user1", "password1"),
        ("user2", "password2"),
        ("user3", "password3"),
    ]);

    if user_db.get(username) == Some(&password) {
        let _ = stream.write(b"authenticated\n");
        Some(username.to_string())  // Return the username on successful authentication
    } else {
        let _ = stream.write(b"Authentication failed\n");
        None  // Return None if authentication fails
    }
}
