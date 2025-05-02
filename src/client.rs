use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::str;
use std::thread;

pub fn start() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Couldn't connect to the server");

    loop {
        let mut username = String::new();
        let mut password = String::new();

        print!("Enter your username: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut username).unwrap();
        username = username.trim().to_string();

        print!("Enter your password: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut password).unwrap();
        password = password.trim().to_string();

        let credentials = format!("{}:{}\n", username, password);
        stream.write(credentials.as_bytes()).expect("Failed to send credentials");

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).expect("Failed to read response");
        let response = str::from_utf8(&buffer[..n]).expect("Failed to parse response");
        println!("Server Response: {}", response);

        if response.contains("authenticated") {
            break;
        } else {
            println!("Please try again.\n");
            stream = TcpStream::connect("127.0.0.1:8080").expect("Couldn't reconnect to the server");
        }
    }

    // Spawn thread to listen for messages
    let mut read_stream = stream.try_clone().unwrap();
    thread::spawn(move || loop {
        let mut buffer = [0; 512];
        match read_stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let response = str::from_utf8(&buffer[..n]).unwrap();
                println!("\n>> {}", response.trim());
                print!("Enter message (or @user): ");
                io::stdout().flush().unwrap();
            }
            _ => break,
        }
    });

    loop {
        let mut message = String::new();
        print!("Enter message (or @username): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut message).unwrap();

        message.push('\n');
        stream.write(message.as_bytes()).expect("Failed to send message");
    }
}
