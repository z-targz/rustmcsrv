
use std::collections::HashMap;
use std::future::IntoFuture;
use tokio::sync::RwLock;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::error::Error;

use tokio::net::TcpStream;

use futures::stream::Stream;

use futures::{AsyncReadExt, FutureExt, StreamExt, TryStreamExt};
use uuid::Uuid;


use server_util::ConnectionState;

//use crate::data::read_var_int_async;
use crate::player::Player;
use crate::packet;


type ConnectionID = usize;

type PlayerID = usize;
pub struct Server<'a> {
    //Server services all connections, including status and login
    connections: Vec<RwLock<Connection>>,
    //Only used in Play state
    player_names: HashMap<String, &'a Player<'a>>,
    player_uuids: HashMap<Uuid, &'a Player<'a>>,
    players: Vec<Player<'a>>,
}

impl<'a> Server<'static> {
    pub fn new(max_players: usize) -> Self {
        Server { 
            connections : Vec::with_capacity(max_players * 2), //we have no guarantee of this, but it should avoid excess reallocation
            player_names : HashMap::with_capacity(max_players), 
            player_uuids : HashMap::with_capacity(max_players), 
            players : Vec::with_capacity(max_players)
        }
    }

    pub fn add_connection(&mut self, stream: TcpStream) -> usize {
        self.connections.push(RwLock::new(Connection::new(stream)));
        self.connections.len()-1
    }

    pub fn get_connections(&self) -> &Vec<RwLock<Connection>> {
        &self.connections
    }

    pub fn get_players(&self) -> &Vec<Player> {
        &self.players
    }
}



pub struct Connection {
    stream: TcpStream,
    state: ConnectionState,
    player: Option<PlayerID>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream : stream, state: ConnectionState::Handshake, player: None }
    }

    pub fn send_packet(&mut self, packet: impl packet::Clientbound) {
        
    }

    pub async fn read_next_packet(&mut self) -> Result<packet::SPacket, Box<dyn Error>> {
        let mut buf:[u8; crate::MTU] = [0u8; 1500];
        //let bytes = self.stream.read(&mut buf).await?;
        todo!()
    }
}

