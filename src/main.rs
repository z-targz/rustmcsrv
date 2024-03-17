#![feature(coroutines)]
#![feature(iter_advance_by)]
#![feature(non_null_convenience)]

#[macro_use]
extern crate serde_json;



use std::error::Error;
use std::fs::{OpenOptions, File};
use std::path::Path;
use std::collections::HashMap;
use std::io::{Write, BufReader};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use state::handshake_state::handshake_state;
use tokio::runtime::Runtime;
use tokio::net::TcpListener;

use lazy_static::lazy_static;

use base64::prelude::*;
use tokio::time::timeout;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use md5;

use crate::data::{Property, PropertyArray};
use crate::packet::login::*;
use crate::packet::status::*;
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

mod state;

//const MTU: usize = 1500;

//TODO: Read these from server.properties
const PORT: u16 = 25565;
//const TOTAL_THREADS: usize = 12;

const MAX_PLAYERS: usize = 32;

const ONLINE_MODE: bool = false;

const TIMEOUT: Duration = Duration::from_secs(10);

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

