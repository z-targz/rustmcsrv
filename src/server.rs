
use std::collections::HashMap;
use std::future::IntoFuture;
use std::sync::RwLock;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_std::net::TcpStream;
use async_std::io::ReadExt;
use futures::stream::Stream;

use futures::{FutureExt, StreamExt, TryStreamExt};
use uuid::Uuid;

use crate::data::read_var_int_async;
use crate::player::Player;
use crate::packet::{Packet, Serverbound};

type ConnectionID = usize;
pub struct Server {
    //Server services all connections, including status and login
    connections: Vec<RwLock<Connection>>,
    //Only used in Play state
    uuids: HashMap<String, Uuid>,
    players: HashMap<Uuid, ConnectionID>,
}

impl Server {
    pub fn new(max_players: usize) -> Self {
        Server { connections : Vec::with_capacity(max_players * 2), uuids : HashMap::with_capacity(max_players), players : HashMap::with_capacity(max_players)}
    }

    pub fn add_connection(&mut self, stream: TcpStream) -> usize {
        self.connections.push(RwLock::new(Connection::new(stream)));
        self.connections.len()-1
    }

    pub fn get_connections(&self) -> &Vec<RwLock<Connection>> {
        &self.connections
    }
}



pub struct Connection {
    stream: TcpStream,
    player: Option<Player>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream : stream, player: None }
    }

    pub fn send_packet(packet: impl Packet) {
        
    }

    pub fn read_next_packet() -> Box<dyn Serverbound> {
        todo!()
    }
}

