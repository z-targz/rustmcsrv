#![feature(coroutines)]
#![feature(iter_advance_by)]
#![feature(test)]
#![feature(hash_extract_if)]
#![recursion_limit = "256"]
#![feature(allocator_api)]
#![feature(async_closure, async_fn_traits)]
#![feature(specialization)]
#![feature(trivial_bounds)]


#[macro_use]
extern crate serde_json;
extern crate test;
extern crate lru;
extern crate itertools;
extern crate convert_case;



use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::net::SocketAddr;
use std::time::Duration;

use command::{Command, CommandMap, CommandMapBuilder};
use data_types::registry::{registry, NBTifiedRegistryEntry, RegistryEntry};
use data_types::tag::TagRegistry;
use event::events::on_disable::EventOnDisable;
use event::events::on_enable::EventOnEnable;
use event::{EventHandler, EventPriority, EventResult};
use tokio::runtime::Runtime;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};



use log::{debug, error, info};

use crate::console::Console;
use crate::server::server_properties::ServerProperties;
use crate::server::Server;
use crate::connection::Connection;
use crate::packet::SPacket;
use crate::state::handshake_state::handshake_state;
use crate::world::World;
use std::sync::{LazyLock, OnceLock};

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
mod command;
mod permission;
mod event;
mod plugins;

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


pub static CONSOLE: OnceLock<Console> = OnceLock::new();

pub static COMMANDS: LazyLock<Vec<Command>> = LazyLock::new(|| {
    Vec::new()
});


pub(crate) static STOP_SIGNAL: OnceLock<broadcast::Sender<bool>> = OnceLock::new();

pub static COMMAND_MAP: LazyLock<tokio::sync::Mutex<MaybeUninit<CommandMap>>> = 
    LazyLock::new(|| tokio::sync::Mutex::new(MaybeUninit::uninit()));

#[tokio::main]
async fn main() {
    info!("Hello, World!");
    debug!("Hello, World!");

    let (logger_tx, console_rx) = mpsc::channel::<String>(16);

    let (stop_signal_tx, stop_signal_rx) = broadcast::channel::<bool>(1);

    let _ = STOP_SIGNAL.get_or_init(move || {
        stop_signal_tx.clone()
    });

    let _ = CONSOLE.get_or_init(|| Console::new(logger_tx.clone()).unwrap());

    RUNTIME.spawn(connection_listener());

    RUNTIME.spawn(chat::chat_thread());
    
    RUNTIME.spawn(console(console_rx));

    RUNTIME.spawn(scheduler(stop_signal_rx)).await.unwrap();
    
}

async fn enable() {
    let command_map_builder = CommandMapBuilder::new();
    //TODO: load plugins, load commands from plugins, register events
    THE_SERVER.get_event_manager().register_event_handler::<EventOnEnable>(
        EventHandler::new(EventPriority::Normal, test_on_enable)
    ).await;
    let mut map_lock = COMMAND_MAP.lock().await;
    map_lock.write(command_map_builder.build());
    event::listen::<EventOnEnable>(THE_SERVER.get_event_manager(), &EventOnEnable::new()).await;
}

async fn scheduler(mut stop: broadcast::Receiver<bool>) {
    let mut interval = tokio::time::interval(Duration::from_millis(50));
    enable().await;
    loop {
        match stop.try_recv() {
            Ok(signal) => if signal == true {
                disable().await;
                break
            } else {
                //TODO: Reload: save, reload plugins, then continue
                disable().await;
                enable().await;
            },
            Err(err) => match err {
                broadcast::error::TryRecvError::Empty => (),
                _ => break
            },
        }

        THE_SERVER.tick_worlds().await;
        interval.tick().await;
        //Tick each world thread
    }

    THE_SERVER.save_worlds().await;
}

fn test_on_enable(e: EventOnEnable) -> EventResult {
    log::log!(log::Level::Info, "OnEnable triggerd");
    EventResult::Default
}



async fn disable() {
    event::listen::<EventOnDisable>(THE_SERVER.get_event_manager(), &EventOnDisable::new()).await;
    let mut map_lock = COMMAND_MAP.lock().await;
    unsafe { map_lock.assume_init_drop(); };

    //TODO: Save Worlds
}

async fn console(mut rx: mpsc::Receiver<String>) {
    
    CONSOLE.get().unwrap().init().unwrap_or_else(|e| {
        error!("{e}");
        std::process::exit(1);
    });

    RUNTIME.spawn( async move {
        while let Some(message) = rx.recv().await {
            CONSOLE.get().unwrap().println(message).await;
        }
    });

    RUNTIME.spawn(CONSOLE.get().unwrap().start()).await
    .unwrap().unwrap_or_else(|e| {
        error!("{e}");
        std::process::exit(1);
    });
}



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





