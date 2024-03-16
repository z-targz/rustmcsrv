#![feature(coroutines)]
#![feature(iter_advance_by)]
#![feature(non_null_convenience)]

#[macro_use]
extern crate serde_json;

use std::fs::{OpenOptions, File};
use std::path::Path;
use std::collections::HashMap;
use std::io::{Write, BufReader};
use std::net::SocketAddr;

use tokio::net::TcpListener;


use lazy_static::lazy_static;

use base64::prelude::*;
use tokio::runtime::Runtime;

use crate::packet::status::{CPingResponse_Status, CStatusResponse};
use crate::server::Server;
use crate::connection::Connection;
use crate::player::Player;

use packet::SPacket;

use server_util::ConnectionState;

mod player;
mod connection;
mod server;
mod data;
mod packet;

//const MTU: usize = 1500;

//TODO: Read these from server.properties
const PORT: u16 = 25565;
//const TOTAL_THREADS: usize = 12;

const MAX_PLAYERS: usize = 32;

lazy_static!{
    pub static ref MOTD: String = "A Minecraft Server (§cMade with Rust!§r)".to_string();
    pub static ref THE_SERVER: Server = Server::new(MAX_PLAYERS, &MOTD);
    pub static ref RUNTIME: Runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
}


#[tokio::main]
async fn main() {

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], PORT))).await.unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    loop {
        let stream = listener.accept().await;
        match stream {
            Ok(stream) => {
                let tcpstream = stream.0;
                let addr = stream.1;
                let _ = tcpstream.set_nodelay(true);
                let connection = Connection::new(tcpstream, addr);
                let future = handshake_state(connection);
                RUNTIME.spawn(future);
            },
            Err(_) => return
        }
    }
}
/// ## Handshake State
/// 
/// Wait for a single packet `SHandshake` and transition to appropriate state `Status` or `Login`
/// 
async fn handshake_state(connection: Connection) {
    let addr = connection.get_addr();
    let result = connection.read_next_packet().await;
    println!("Connection established: {}", addr);
    match result {
        Ok(s_packet) => {
            match s_packet {
                SPacket::SHandshake(packet) => {
                    println!("{addr} > Handshake Successful!");
                    println!("{addr} > Protocol Version: {}", packet.get_protocol_version());
                    println!("{addr} > Hostname used to connect: {}", packet.get_server_address());
                    println!("{addr} > Port used to connect: {}", packet.get_server_port());
                    match packet.get_next_state()
                    {
                        1 => {
                            connection.set_connection_state(ConnectionState::Status).await;
                            println!("{addr} > Next State: Status(1)");
                            status_state(connection).await;
                        }
                        2 => {
                            connection.set_connection_state(ConnectionState::Login).await;
                            println!("{addr} > Next State: Login(1)");
                            login_state(connection).await;
                        }
                        _ => {
                            //Invalid login state. 
                            //This will never happen with a vanilla client unless something goes terribly wrong.
                            connection.drop().await;
                            return
                        }
                    }
                }
                _ => {
                    //Incorrect packet
                    connection.drop().await;
                }
            }
        },
        Err(_) => {
            //Error reading packet
            connection.drop().await;
        }
    }
}
/// ## Status ping sequence:
/// 
/// __C -> S__ &nbsp; : &nbsp; SStatusRequest
/// 
/// __S -> C__ &nbsp; : &nbsp; CStatusResponse
/// 
/// __C -> S__ &nbsp; : &nbsp; SPingRequest_Status
/// 
/// __S -> C__ &nbsp; : &nbsp; SPingResponse_Status
/// 
async fn status_state(connection: Connection) {
    let addr = connection.get_addr();
    /*
        Listen for SStatusRequest
    */
    println!("{addr} > Listening for SStatusRequest...");
    match connection.read_next_packet().await {
        Ok(s_packet) => {
            match s_packet {
                SPacket::SStatusRequest(_) => (),
                _ => {
                    println!("{addr} > Incorrect packet...");
                    connection.drop().await;
                    return;
                }
            }
        },
        Err(_) => {
            connection.drop().await;
        }
    }
    println!("{addr} > Received SStatusRequest!");

    /*
        Send CStatusResponse
     */
    println!("{addr} > Sending CStatusResponse...");
    match connection.send_packet(generate_status_response().await).await {
        Ok(_) => (),
        Err(_) => {
            println!("{addr} > Error sending packet!");
            connection.drop().await;
            return;
        }
    };
    println!("{addr} > Sent CStatusResponse.");

    /*
        Listen for SPingRequest_Status
     */
    let payload: i64;
    match connection.read_next_packet().await {
        Ok(s_packet) => {
            match s_packet {
                SPacket::SPingRequest_Status(s_ping_request_status) => {
                    payload = s_ping_request_status.get_payload();
                },
                _ => {
                    //Incorrect packet
                    println!("{addr} > Incorrect packet");
                    connection.drop().await;
                    return;
                }
            }
        },
        Err(_) => {
            connection.drop().await;
            return;
        }
    }
    println!("{addr} > Received SPingRequest_Status!");

    /*
        Send SPingResponse_Status
    */
    match connection.send_packet(CPingResponse_Status::new(payload)).await {
        Ok(_) => (),
        Err(_) => {
            println!("{addr} > Unable to send packet SPingResponse_Status");
            connection.drop().await;
            return;
        }
    };

    connection.drop().await;
    println!("Connection Closed: {addr}.");
    return;
}

async fn generate_status_response() -> CStatusResponse {
    let player_count = THE_SERVER.get_players_iter().count();
    let max_players = THE_SERVER.get_max_players();
    let motd = THE_SERVER.get_motd().clone();

    let mut result = String::new();
    let data = std::fs::read(Path::new("server-icon.png"));
    match data {
        Ok(vec) => {
            BASE64_STANDARD.encode_string(vec, &mut result);
        }
        Err(_) => {
            let r: Result<&str, std::io::Error> = server_macros::base64_image!("favicon.png");
            result = r.unwrap().to_string();
        }
    }
    let favicon_str = format!("data:image/png;base64,{}", result);
    CStatusResponse::new(json!({
        "version": {
            "name": "1.20.1",
            "protocol": 765
        },
        "players": {
            "max": max_players as i32,
            "online": player_count,
            "sample": [] //TODO: make the sample
        },
        "description": {
            "text": motd
        },
        "favicon": favicon_str,
        "enforceSecureChat": false,
        //"previewsChat": false
    }).to_string())
}

/// ## Login Sequence:
/// 
/// __C -> S__ &nbsp; : &nbsp; SLoginStart
/// 
/// __S -> C__ &nbsp; : &nbsp; CEncryptionRequest //TODO: Not required for offline mode
/// 
/// __C -> S__ &nbsp; : &nbsp; SEncryptionResponse //Only if we sent the above packet
/// 
/// __Server auth step__ //TODO:
/// 
/// __S -> C__ CSetCompression //Optional
/// 
/// __S -> C__ CLoginSuccess
/// 
/// __C -> S__ SLoginAcknowledged
/// 
async fn login_state(connection: Connection) {
    /*
        Listen for SLoginStart
    */
    println!("Listening for SLoginStart...");
    match connection.read_next_packet().await {
        Ok(s_packet) => {
            match s_packet {
                SPacket::SLoginStart(packet) => {
                    let player_name = packet.get_name().clone();

                    #[warn(TEMPORARY)]
                    let player_uuid = packet.get_uuid(); //TODO: DO NOT RELY ON THIS VALUE! GENERATE THE UUID OURSELVES!

                    let player = Player::new(player_name, player_uuid, connection);
                    println!("Registering player...");    

                    let _player_id = THE_SERVER.register_player(player).await;

                    println!("Registered player!");              
                },
                _ => {
                    println!("Incorrect packet...");
                    connection.drop().await;
                    return;
                }
            }
        },
        Err(_) => {
            connection.drop().await;
            return;
        }
    }
}

fn handle_config() -> Result<HashMap<String, String>, std::io::Error> {
    if !Path::new("server.properties").exists()
    {
        let mut file = File::create("server.properties")?;
        file.write("server-port=25565".as_bytes())?;
    }
    let properties = HashMap::new();
    todo!("READ PROPERTIES");
    Ok(properties)
}

