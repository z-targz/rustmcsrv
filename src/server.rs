
use std::collections::HashMap;
use async_std::net::TcpStream;
use uuid::Uuid;

use crate::player::Player;
use crate::packet::Packet;

type ConnectionID = usize;
pub struct Server {
    //Server services all connections, including status and login
    connections: Vec<Connection>,
    //Only used in Play state
    uuids: HashMap<String, Uuid>,
    players: HashMap<Uuid, ConnectionID>
}

impl Server {
    pub fn new(max_players: usize) -> Self {
        Server { connections : Vec::with_capacity(max_players * 2), uuids : HashMap::with_capacity(max_players), players : HashMap::with_capacity(max_players) }
    }

    pub fn add_connection(&mut self, stream: TcpStream) -> usize {
        self.connections.push(Connection::new(stream));
        self.connections.len()-1
    }

    pub fn get_connections_mut(&mut self) -> &mut Vec<Connection> {
        &mut self.connections
    }

    pub fn get_connections(&self) -> &Vec<Connection> {
        &self.connections
    }
}

pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play
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
}

//const HANDSHAKE_STATE_MAX_PACKET_SIZE: usize = 8;


impl Iterator for Connection {
    type Item = u8;
    
    fn next(&mut self) -> Option<Self::Item> {
        
        todo!()
        //TODO: Read from stream and block if None, await input using mio.
    }
}