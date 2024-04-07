#![feature(coroutines)]
#![feature(iter_advance_by)]
#![feature(non_null_convenience)]
#![feature(test)]
#![feature(hash_extract_if)]
#![recursion_limit = "256"]
#![feature(allocator_api)]

#[macro_use]
extern crate serde_json;
extern crate test;
extern crate lru;
extern crate itertools;




use std::net::SocketAddr;
use std::time::Duration;

use tokio::runtime::Runtime;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

use lazy_static::lazy_static;

use log::{debug, error, info};

use crate::console::Console;
use crate::data_types::Identifier;
use crate::server::server_properties::ServerProperties;
use crate::server::Server;
use crate::connection::Connection;
use crate::packet::SPacket;
use crate::state::handshake_state::handshake_state;
use crate::world::World;

mod player;
mod player_data;
mod connection;
mod server;
mod data_types;
mod packet;
mod state;
mod chat;
mod world;
mod game;
mod console;
mod entity;

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
    pub static ref REGISTRY_NBT: Vec<u8> = data_types::registry::get_registry_nbt().unwrap();
    pub static ref CONSOLE: Console = Console::new().unwrap();
}
#[tokio::main]
async fn main() {

    CONSOLE.init().unwrap_or_else(|e| {
        error!("{e}");
        std::process::exit(1);
    });

    info!("Hello, World!");
    debug!("Hello, World!");

    RUNTIME.spawn(connection_listener());

    RUNTIME.spawn(chat::chat_thread());

    
    //TODO: Implement channels for this, so that this thread can call the tick method on the world threads.
    RUNTIME.spawn(scheduler());

    
    //TODO: Start thread for each world
    
    

    


    CONSOLE.start().unwrap_or_else(|e| {
        error!("{e}");
        std::process::exit(1);
    });
}

async fn scheduler() {
    let mut interval = tokio::time::interval(Duration::from_millis(50));
    loop {
        interval.tick().await;
        //Tick each world thread
        //for world in worlds {
        //    world.tick();    
        //}
    }
}

/// This function spends a lot of time `await`ing a new connection,
/// so it should be spawned in a tokio thread
async fn connection_listener() {
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





