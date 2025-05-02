mod client;
mod server;
mod auth;
mod messaging;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "server" {
        server::start();
    } else {
        client::start();
    }
}
