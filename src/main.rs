use core::mem::drop;

use std::fs;
use std::fs::{OpenOptions, File};
use std::path::Path;
use std::collections::HashMap;
use std::io::{Write, BufReader};
use std::net::{SocketAddr};
use std::sync::RwLock;

use futures::executor::{ThreadPool, ThreadPoolBuilder};
use futures::task::FutureObj;
use futures::future::Lazy;
use futures::StreamExt;

use async_std::net::{TcpListener, TcpStream};

use lazy_static::lazy_static;

use crate::server::Server;
use crate::server::Connection;

mod player;
mod server;
mod packet;
mod data;

const MTU: usize = 1500;

//TODO: Read these from server.properties
const PORT: u16 = 25565;
const TOTAL_THREADS: usize = 8;
const MOTD: &str = "A Minecraft Server (Made with Rust!)";
const MAX_PLAYERS: usize = 32;


lazy_static!{
    pub static ref THE_SERVER: RwLock<Server> = RwLock::new(Server::new(MAX_PLAYERS));
}

#[async_std::main]
async fn main() {
    //let mut the_server = Server::new(MAX_PLAYERS);

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], PORT))).await.unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    /*let mut thread_pool_builder = ThreadPoolBuilder::new();
    thread_pool_builder.pool_size(TOTAL_THREADS - 3);
    let pool = thread_pool_builder.create().unwrap();*/
    
    let mut thread_pool_builder = async_executors::ThreadPool::builder();
    thread_pool_builder.pool_size(TOTAL_THREADS - 3);
    let pool = thread_pool_builder.create().unwrap();

    listener.incoming().for_each_concurrent(None, |stream| async {
        match stream {
            Ok(tcpstream) => {
                {
                    let _ = tcpstream.set_nodelay(true);
                    let mut w = THE_SERVER.write().unwrap();
                    let n = w.add_connection(tcpstream);
                    drop(w);
                    let connection = handle_connection(n);
                    pool.spawn_ok(connection);
                }
            },
            Err(_) => return
        }  
    }).await;
}

use std::error::Error;
pub async fn handle_connection(n: usize) {
    let r = THE_SERVER.read().unwrap();
    let w = r.get_connections().get(n).unwrap().write().unwrap();
    {
        
    }
    drop(w);
    drop(r);
    
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

