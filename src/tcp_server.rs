
use std::{collections::HashMap, io::{Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, sync::{Arc, Mutex}};
use crate::{player::{Player}, position::Position, server::Server, vector::Vector};

pub struct TCPServer {
    address: String,
    players: Arc<Mutex<HashMap<SocketAddr, Player>>>
}

impl TCPServer {
    pub fn new(address: &str) -> Self {
        TCPServer {
            address: address.to_string(),
            players: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn handle_client(mut stream: TcpStream, players: Arc<Mutex<HashMap<SocketAddr, Player>>>) {
        let peer_addr = stream.peer_addr().unwrap_or_else(|_| "0.0.0.0".parse().unwrap());
        let mut buffer = [0; 1024];
        
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Client {} disconnected", peer_addr);
                    players.lock().unwrap().remove(&peer_addr);
                    break; // Connection closed
                }
                Ok(n) => {
                    let input = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                    let mut response = String::new();

                    if input.starts_with("ping") {
                        response = "pong\n".to_string();
                    }
                    else if input.starts_with("join:") {
                        let username = input["join:".len()..].to_string();
                        players.lock().unwrap().insert(peer_addr, Player::new(&username.to_string(), Position::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)));

                        let msg = format!("{} joined the game\n", username);
                        response = msg.clone();
                        print!("{}", msg);
                    }
                    else if input.starts_with("move:") {
                        let parts: Vec<&str> = input["move:".len()..].split(',').collect();
                        if parts.len() == 3 {
                            if let (Ok(x), Ok(y), Ok(z)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>(), parts[2].parse::<f32>()) {
                                if let Some(player) = players.lock().unwrap().get_mut(&peer_addr) {
                                    player.position = Position::new(x, y, z);

                                    let msg = format!("{} moved to ({}, {}, {})\n", player.username, x, y, z);
                                    response = msg.clone();
                                    println!("{}", msg);
                                }
                            }
                        } else {
                            break; // Invalid move command format
                        }
                    }
                    
                    let _ = stream.write_all(response.as_bytes());
                }
                Err(e) => {
                    eprintln!("Failed to read from {}: {}", peer_addr, e);
                    players.lock().unwrap().remove(&peer_addr);
                    break; // Error occurred, close connection
                }
            }
        }
    }
}

impl Server for TCPServer {
    fn start(&self) {
        println!("Starting TCP server at {}", self.address);

        let listener = TcpListener::bind(&self.address).expect("Failed to bind TCP listener");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New client connected: {}", stream.peer_addr().unwrap());

                    let players = Arc::clone(&self.players);
                    std::thread::spawn(move || {
                        TCPServer::handle_client(stream, players);
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept client: {}", e);
                }
            }
        }
    }

    fn stop(&self) {
        println!("Stopping TCP server at {}", self.address);
        // Here you would add the actual logic to stop the server
    }

    fn restart(&self) {
        println!("Restarting TCP server at {}", self.address);
        // Here you would add the actual logic to restart the server
    }
}
