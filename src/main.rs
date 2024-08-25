#![feature(coroutines)]
#![feature(iter_advance_by)]
#![feature(test)]
#![feature(hash_extract_if)]
#![recursion_limit = "256"]
#![feature(allocator_api)]
#![feature(async_closure, async_fn_traits)]
#![feature(specialization)]


#[macro_use]
extern crate serde_json;
extern crate test;
extern crate lru;
extern crate itertools;
extern crate convert_case;



use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;

use data_types::registry::{registry, NBTifiedRegistryEntry, RegistryEntry};
use data_types::tag::TagRegistry;
use data_types::NBT;
use server_macros::pack_registry_json_files;
use tokio::runtime::Runtime;
use tokio::net::TcpListener;
use tokio::sync::mpsc;



use log::{debug, error, info};

use crate::console::Console;
use crate::data_types::Identifier;
use crate::server::server_properties::ServerProperties;
use crate::server::Server;
use crate::connection::Connection;
use crate::packet::SPacket;
use crate::state::handshake_state::handshake_state;
use crate::world::World;
use std::sync::LazyLock;

mod player;
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
mod nbt;
mod item;

//const MTU: usize = 1500;

//TODO: Read these from server.properties
//const TOTAL_THREADS: usize = 12;



const TIMEOUT: Duration = Duration::from_secs(10);
pub const REGISTRIES: [&str;9] = [
    "worldgen/biome",
    "chat_type",
    "trim_pattern", 
    "trim_material", 
    "wolf_variant",
    "dimension_type",
    "damage_type",
    "banner_pattern",
    "painting_variant",
];

const TAGS: [&str;5] = [
    "block",
    "item",
    "fluid",
    "entity_type",
    "game_event",
];


pub static REGISTRIES_JSON: LazyLock<String> = LazyLock::new(|| {
    include_str!("../generated/reports/registries.json").to_owned()
});

pub static SERVER_REGISTRY: LazyLock<HashMap<String, HashMap<String, RegistryEntry>>> = LazyLock::new(|| {
    registry::get_registry().unwrap()
});

pub static REGISTRY_NBT: LazyLock<HashMap<String, Vec<NBTifiedRegistryEntry>>> = LazyLock::new(|| {
    REGISTRIES.iter().map(|registry_name| {
        (
            registry_name.to_string(), 
            data_types::registry::get_registry_nbt(
                SERVER_REGISTRY.get(*registry_name).unwrap()
                    .iter()
                    .map(|(_, v)| v.clone())
                    .collect()).unwrap())
    }).collect()
});

pub static REGISTRY_TAGS: LazyLock<Vec<TagRegistry>> = LazyLock::new(|| {
    TAGS.iter().map(|tags| TagRegistry::new(tags)).collect()
});

pub static THE_SERVER: LazyLock<Server> = LazyLock::new(|| {
    Server::new(ServerProperties::load_server_properties().unwrap())
});

pub static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread().enable_time().enable_io().build().unwrap()
});



pub static CONSOLE: LazyLock<Console> = LazyLock::new(|| {
    Console::new().unwrap()
});


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





