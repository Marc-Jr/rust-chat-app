use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::thread;

fn handle_input(mut stream: TcpStream) {
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        stream.write(input.trim().as_bytes()).unwrap();
    }
}

fn handle_output(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                println!("{}", String::from_utf8_lossy(&buffer[..n]));
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    
    println!("Enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    // Send name to server
    stream.write(name.as_bytes()).unwrap();

    // Start threads for input and output
    let stream_clone = stream.try_clone().unwrap();
    thread::spawn(move || handle_input(stream_clone));
    handle_output(stream);
}
