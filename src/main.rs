#![feature(coroutines)]
#![feature(iter_advance_by)]
#![feature(non_null_convenience)]
#![feature(test)]
#![recursion_limit = "256"]


#[macro_use]
extern crate serde_json;

extern crate test;




use std::net::SocketAddr;
use std::time::Duration;


use tokio::runtime::Runtime;
use tokio::net::TcpListener;

use lazy_static::lazy_static;

use crate::server::server_properties::ServerProperties;
use crate::server::Server;
use crate::connection::Connection;
use crate::packet::SPacket;
use crate::state::handshake_state::handshake_state;

mod player;
mod connection;
mod server;
mod data;
mod packet;
mod state;
mod chat;
mod world;
mod game;

//const MTU: usize = 1500;

//TODO: Read these from server.properties
//const TOTAL_THREADS: usize = 12;



const TIMEOUT: Duration = Duration::from_secs(10);

lazy_static!{
    pub static ref MOTD: String = "A Minecraft Server (§cMade with Rust!§r)".to_string();
    pub static ref THE_SERVER: Server = Server::new(ServerProperties::load_server_properties().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    }));
    pub static ref RUNTIME: Runtime = tokio::runtime::Builder::new_multi_thread().enable_time().enable_io().build().unwrap();
    pub static ref REGISTRY_NBT: Vec<u8> = data::registry::get_registry_nbt().unwrap();
}


#[tokio::main]
async fn main() {

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], THE_SERVER.get_properties().get_server_port()))).await.unwrap_or_else(|e| {
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





