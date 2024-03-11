use std::fs;
use std::fs::{OpenOptions, File};
use std::path::Path;
use std::collections::HashMap;
use std::io::{Write, BufReader};
use std::net::{SocketAddr, TcpListener, TcpStream};
use futures::executor::{ThreadPool, ThreadPoolBuilder};
use futures::task::FutureObj;

mod player;
mod server;
mod packet;
mod data;

const MTU: usize = 1500;

//TODO: Read these from server.properties
const PORT: u16 = 25565;
const TOTAL_THREADS: usize = 8;
const MOTD: &str = "A Minecraft Server (Made with Rust!)";

fn main() {
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], PORT))).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    let mut thread_pool_builder = ThreadPoolBuilder::new();
    thread_pool_builder.pool_size(TOTAL_THREADS - 3);
    let pool = thread_pool_builder.create().unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let connection = handle_connection(stream);
        pool.spawn_ok(connection);
    }
}

async fn handle_connection(mut stream: TcpStream) {
    //let buffer: [u8; MTU] = [0; MTU];
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

