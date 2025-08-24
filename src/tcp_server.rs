
use std::{collections::HashMap, io::{Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, sync::{Arc, Mutex}};
use crate::{player::{Player}, position::Position, server::Server, vector::Vector};

#[derive(Clone)]
pub struct TCPServer {
    address: String,
    players: Arc<Mutex<HashMap<SocketAddr, Player>>>,
    streams: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>,
}

impl TCPServer {
    pub fn new(address: &str) -> Arc<Self> {
        Arc::new(TCPServer {
            address: address.to_string(),
            players: Arc::new(Mutex::new(HashMap::new())),
            streams: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    fn handle_client(mut stream: TcpStream, server: &Arc<TCPServer>) {
        let peer_addr = stream.peer_addr().unwrap_or_else(|_| "0.0.0.0".parse().unwrap());
        let mut buffer = [0; 1024];
        
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Client {} disconnected", peer_addr);
                    server.players.lock().unwrap().remove(&peer_addr);
                    break; // Connection closed
                }
                Ok(n) => {
                    let input = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

                    if input.starts_with("ping") {
                        let response = "pong\n".to_string();
                        let _ = stream.write_all(response.as_bytes());
                    }
                    else if input.starts_with("join:") {
                        let username = input["join:".len()..].to_string();
                        server.players.lock().unwrap().insert(peer_addr, Player::new(&username.to_string(), Position::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)));
                        server.streams.lock().unwrap().insert(peer_addr, stream.try_clone().unwrap());

                        let msg = format!("{} joined the game\n", username);
                        let response = msg.clone();
                        print!("{}", msg);

                        server.broadcast(response.as_bytes(), None);
                    }
                    else if input.starts_with("move:") {
                        let parts: Vec<&str> = input["move:".len()..].split(',').collect();
                        if parts.len() == 6 {
                            if let (
                                Ok(pos_x),
                                Ok(pos_y),
                                Ok(pos_z),
                                Ok(dir_x),
                                Ok(dir_y),
                                Ok(dir_z)
                            ) = (
                                parts[0].parse::<f32>(),
                                parts[1].parse::<f32>(),
                                parts[2].parse::<f32>(),
                                parts[3].parse::<f32>(),
                                parts[4].parse::<f32>(),
                                parts[5].parse::<f32>()
                            ) {
                                if let Some(player) = server.players.lock().unwrap().get_mut(&peer_addr) {
                                    player.position = Position::new(pos_x, pos_y, pos_z);
                                    player.direction = Vector::new(dir_x, dir_y, dir_z);

                                    let msg = format!("{} moved to ({}, {}, {}), dir ({}, {}, {})\n",
                                        player.username,
                                        pos_x, pos_y, pos_z,
                                        dir_x, dir_y, dir_z
                                    );
                                    let response = msg.clone();
                                    println!("{}", msg);

                                    server.broadcast(response.as_bytes(), None);
                                }
                            }
                        } else {
                            break; // Invalid move command format
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from {}: {}", peer_addr, e);
                    server.players.lock().unwrap().remove(&peer_addr);
                    break; // Error occurred, close connection
                }
            }
        }
    }

    fn broadcast(&self, bytes: &[u8], exclude_client: Option<SocketAddr>) {
        // Default value is there to get a value if no exclude_client is provided (we know 0.0.0.0 is impossible to be a real client)
        let excluded = exclude_client.unwrap_or("0.0.0.0:0".parse().unwrap());

        let streams_guard = self.streams.lock().unwrap();
        for (_, mut stream) in streams_guard.iter() {
            if stream.peer_addr().unwrap() == excluded {
                continue;
            }
            let _ = stream.write_all(bytes);
        }
    }
}
impl Server for Arc<TCPServer> {
    fn start(&self) {
        println!("Starting TCP server at {}", self.address);

        let listener = TcpListener::bind(&self.address).expect("Failed to bind TCP listener");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New client connected: {}", stream.peer_addr().unwrap());

                    let server = Arc::clone(self);
                    std::thread::spawn(move || {
                        TCPServer::handle_client(stream, &server);
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
