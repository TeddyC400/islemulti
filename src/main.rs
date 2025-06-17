use islemulti::{server::Server, tcp_server::TCPServer};

fn main() {
    println!("Starting islemulti...");

    let server = TCPServer::new("127.0.0.1:9001");
    server.start();
}
