
use std::collections::HashMap;
use std::net::TcpStream;
use uuid::Uuid;

use crate::player::Player;
use crate::packet::Packet;

type ConnectionID = usize;
pub struct Server {
    //Server services all connections, including status and login
    connections: HashMap<ConnectionID, Connection>,
    //Only used in Play state
    uuids: HashMap<String, Uuid>,
    players: HashMap<Uuid, ConnectionID>
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