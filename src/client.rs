use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Couldn't connect to the server");

    loop {
        // Prompt for username and password
        let mut username = String::new();
        let mut password = String::new();

        print!("Enter your username: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut username).unwrap();
        username = username.trim().to_string(); // Remove newline

        print!("Enter your password: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut password).unwrap();
        password = password.trim().to_string(); // Remove newline

        // Send the credentials to the server
        let credentials = format!("{}:{}", username, password);
        stream.write(credentials.as_bytes()).expect("Failed to send credentials");

        // Wait for server response
        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).expect("Failed to read response");

        // Convert to UTF-8 and display the server's response
        let response = str::from_utf8(&buffer[..n]).expect("Failed to parse response");
        println!("Server Response: {}", response); // Print server response

        // Check if authentication was successful
        if response.contains("authenticated") {
            break; // Exit the loop on successful authentication
        } else {
            println!("Please try again.\n");
            // Reset the connection and try again (client will reconnect on next loop)
            stream = TcpStream::connect("127.0.0.1:8080").expect("Couldn't reconnect to the server");
        }
    }
}
