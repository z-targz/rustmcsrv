#![feature(coroutines)]
#![feature(iter_advance_by)]

#[macro_use]
extern crate serde_json;


use core::mem::drop;

use std::fs::{OpenOptions, File};
use std::path::Path;
use std::collections::HashMap;
use std::io::{Write, BufReader};
use std::net::{SocketAddr};


use futures::executor::{ThreadPool, ThreadPoolBuilder};

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

use lazy_static::lazy_static;

use crate::packet::status::{CPingResponse_Status, CStatusResponse};
use crate::server::Server;
use crate::server::Connection;

use packet::{SPacket};

use server_util::ConnectionState;

mod server;
mod packet;
mod player;


mod data;

const MTU: usize = 1500;

//TODO: Read these from server.properties
const PORT: u16 = 25565;
const TOTAL_THREADS: usize = 12;
const MOTD: &str = "A Minecraft Server (Made with Rust!)";
const MAX_PLAYERS: usize = 32;

lazy_static!{
    pub static ref THE_SERVER: RwLock<Server<'static>> = RwLock::new(Server::new(MAX_PLAYERS));
}

#[tokio::main]
async fn main() {
    let threadpool = tokio::runtime::Builder::new_multi_thread().worker_threads(TOTAL_THREADS - 3).build().unwrap();


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
                let mut server = THE_SERVER.write().await;
                let id = server.add_connection(tcpstream, addr);
                drop(server);
                let connection = handle_connection(id);
                threadpool.spawn(connection);
            },
            Err(_) => return
        }
    }
}

/// Helper function
/// 
/// Drops the connection with the given `Connection ID` (see `server::Connections`)
/// 
async fn drop_by_id(id: usize) {
    let mut server = THE_SERVER.write().await;
    server.drop_connection_by_id(id);
    drop(server);
}

async fn handle_connection(id: usize) {
    let server = THE_SERVER.read().await;
    let connection = server.get_connection_by_id(id).unwrap(); //TODO: make this better
    let addr = connection.get_addr();
    let result = connection.read_next_packet().await;
    drop(server);
    println!("Connection established: {}", addr);
    match result {
        Ok(s_packet) => {
            match s_packet {
                SPacket::SHandshake(packet) => {
                    println!("Handshake Successful!");
                    println!("Protocol Version: {}", packet.get_protocol_version());
                    println!("Hostname used to connect: {}", packet.get_server_address());
                    println!("Port used to connect: {}", packet.get_server_port());
                    match packet.get_next_state()
                    {
                        1 => {
                            
                            let mut mut_server = THE_SERVER.write().await;
                            if mut_server.set_connection_state_by_id(id, ConnectionState::Status).is_err() {
                                //Connection not found
                                println!("Connection not found...");
                                return;
                            }
                            drop(mut_server);
                            println!("Next State: Status(1)");
                            status_state(id).await;
                        }
                        2 => {
                            let mut mut_server = THE_SERVER.write().await;
                            if mut_server.set_connection_state_by_id(id, ConnectionState::Login).is_err() {
                                //Connection not found
                                println!("Connection not found...");
                                return;
                            }
                            drop(mut_server);
                            println!("Next State: Login(1)");

                        }
                        
                        _ => {
                            //Incorrect packet, close stream and clean up
                            drop_by_id(id).await;
                            return
                        }
                    }
                }
                _ => {
                    drop_by_id(id).await;
                }
            }
        },
        Err(_) => {
            drop_by_id(id).await;
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
async fn status_state(id: usize) {
    
    let server = THE_SERVER.read().await;
    let connection = server.get_connection_by_id(id).unwrap(); //TODO: make this better
    
    //Listen for SStatusRequest
    println!("Listening for SStatusRequest...");
    match connection.read_next_packet().await {
        Ok(s_packet) => {
            match s_packet {
                SPacket::SStatusRequest(_) => (),
                _ => {
                    println!("Incorrect packet...");
                    drop_by_id(id).await;
                    return;
                }
            }
        },
        Err(_) => {
            drop_by_id(id).await;
        }
    }
    drop(server);

    println!("Received SStatusRequest!");

    println!("Sending CStatusResponse...!");

    //Send CStatusResponse
    let server = THE_SERVER.read().await;
    let connection = server.get_connection_by_id(id).unwrap(); //TODO: make this better
    let online_players = server.get_players();
    let c_status_response = CStatusResponse::new(json!(
        {
            "version": {
                "name": "1.20.1",
                "protocol": 763
            },
            "players": {
                "max": MAX_PLAYERS as i32,
                "online": online_players.len(),
                "sample": [] //TODO: make the sample
            },
            "description": {
                "text": MOTD
            },
            //TODO: "favicon": "data:image/png;base64,<data>" //where <data> is the base64 encoding of the image
            "enforceSecureChat": false,
            //"previewsChat": false
        }
    ).to_string());
    
    println!("Constructed JSON");

    match connection.send_packet(c_status_response).await {
        Ok(_) => (),
        Err(_) => {
            println!("Error sending packet!");
            drop_by_id(id).await;
            return;
        }
    };
    drop(server);
    println!("Sent CStatusResponse.");


    //Listen for SPingRequest_Status
    let payload: i64;
    let server = THE_SERVER.read().await;
    let connection = server.get_connection_by_id(id).unwrap(); //TODO: make this better
    match connection.read_next_packet().await {
        Ok(s_packet) => {
            match s_packet {
                SPacket::SPingRequest_Status(s_ping_request_status) => {
                    payload = s_ping_request_status.get_payload();
                },
                _ => {
                    println!("Incorrect packet");
                    //Incorrect packet, close stream and clean up
                    drop_by_id(id).await;
                    return;
                }
            }
        },
        Err(_) => {
            drop_by_id(id).await;
            return;
        }
    }
    println!("Received SPingRequest_Status!");
    drop(server);

    //Send SPingResponse_Status
    let server = THE_SERVER.read().await;
    let connection = server.get_connection_by_id(id).unwrap(); //TODO: make this better
    match connection.send_packet(CPingResponse_Status::new(payload)).await {
        Ok(_) => (),
        Err(_) => {
            drop_by_id(id).await;
            return;
        }
    };
    drop(server);

    drop_by_id(id).await;
    println!("Connection Closed.");
    return;
    

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

