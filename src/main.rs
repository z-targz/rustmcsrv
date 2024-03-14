#![feature(coroutines)]
#![feature(iter_advance_by)]

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


use crate::server::Server;


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
    
    /*let mut thread_pool_builder = ThreadPoolBuilder::new();
    thread_pool_builder.pool_size(TOTAL_THREADS - 3);
    let pool = thread_pool_builder.create().unwrap();*/
    
    //let mut thread_pool_builder = async_executors::ThreadPool::builder();
    //thread_pool_builder.pool_size(TOTAL_THREADS - 3);
    //let pool = thread_pool_builder.create().unwrap();

    loop {
        let stream = listener.accept().await;
        match stream {
            Ok(stream) => {
                let tcpstream = stream.0;
                let addr = stream.1;
                let _ = tcpstream.set_nodelay(true);
                let mut server = THE_SERVER.write().await;
                let n = server.add_connection(tcpstream, addr);
                drop(server);
                let connection = handle_connection(n);
                threadpool.spawn(connection);
            },
            Err(_) => return
        }
    }
}

pub async fn handle_connection(n: usize) {
    let server = THE_SERVER.read().await;
    let connection = server.get_connection_by_id(n).unwrap(); //TODO: fix this
    let addr = connection.get_addr();
    let result = connection.read_next_packet().await;

    println!("Connection established: {}", addr);
    match result {
        Ok(s_packet) => {
            match s_packet {
                packet::SPacket::SHandshake(packet) => {
                    println!("Handshake Successful!");
                    println!("Protocol Version: {}", packet.get_protocol_version());
                    println!("Hostname used to connect: {}", packet.get_server_address());
                    println!("Port used to connect: {}", packet.get_server_port());
                    match packet.get_next_state()
                    {
                        1 => {
                            let mut mut_server = THE_SERVER.write().await;
                            match mut_server.set_connection_state_by_id(n, ConnectionState::Status) {
                                Ok(_) => (),
                                Err(_) => {
                                    //Connection not found
                                    return;
                                }
                            }
                            drop(mut_server);
                            println!("Next State: Status(1)");
                        }
                        2 => {
                            let mut server = THE_SERVER.write().await;
                            match server.set_connection_state_by_id(n, ConnectionState::Status) {
                                Ok(_) => (),
                                Err(_) => {
                                    //Connection not found
                                    return;
                                }
                            }
                            drop(server);
                            println!("Next State: Login(1)");
                        }
                        
                        _ => {
                            //Incorrect packet, close stream and clean up
                            let mut server = THE_SERVER.write().await;
                            server.drop_connection_by_id(n);
                            drop(server);
                            return
                        }
                    }
                    
                    
                }
                _ => {
                    //Incorrect packet, close stream and clean up
                }
            }
        },
        Err(_) => {
            //Something went wrong, close stream and clean up
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

