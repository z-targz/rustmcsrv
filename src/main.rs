#![feature(coroutines)]
use core::mem::drop;

use std::arch::x86_64::_CMP_EQ_OS;
use std::fs;
use std::fs::{OpenOptions, File};
use std::path::Path;
use std::collections::HashMap;
use std::io::{Write, BufReader};
use std::net::{SocketAddr};


use futures::executor::{ThreadPool, ThreadPoolBuilder};
use futures::task::FutureObj;
use futures::future::Lazy;
use futures::StreamExt;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

use lazy_static::lazy_static;

use crate::packet::handshake::SHandshake;
use crate::server::Server;
use crate::server::Connection;


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
                let mut tcpstream = stream.0;
                let _ = tcpstream.set_nodelay(true);
                {
                    
                    let mut w = THE_SERVER.write().await;
                    let n = w.add_connection(tcpstream);
                    drop(w);
                    let connection = handle_connection(n);
                    threadpool.spawn(connection);
                }
            },
            Err(_) => return
        }
    }
}

pub async fn handle_connection(n: usize) {
    let r = THE_SERVER.read().await;
    let w = r.get_connections().get(n).unwrap();
    let result = w.read_next_packet().await;
    drop(r);
    match result {
        Ok(s_packet) => {
            match s_packet {
                packet::SPacket::SHandshake(packet) => {
                    
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
    
    
    todo!()
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

