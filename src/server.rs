
use std::collections::HashMap;
use std::cmp::min;

use std::net::SocketAddrV4;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::error::Error;

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use uuid::Uuid;


use server_util::ConnectionState;

//use crate::data::read_var_int_async;
use crate::player::Player;
use crate::packet;
use crate::data::{read_var_int, read_var_int_async};

use async_stream::stream;



struct Connections<'a> {
    connections: HashMap<usize, Connection<'a>>,
    idx: usize,
}

impl<'a> Connections<'a> {
    pub fn new(max_players: usize) -> Self {
        Connections { connections : HashMap::with_capacity(max_players * 2), idx : 0 }
    }
    pub fn add(&mut self, tcpstream: TcpStream) -> usize {
        self.connections.insert(self.idx, Connection::new(tcpstream));
        self.idx += 1;
        self.idx-1
    }
    pub fn get_by_id(&self, id: usize) -> Result<&'a Connection, ConnectionError> {
        match self.connections.get(&id) {
            Some(connection) => Ok(connection),
            None => Err(ConnectionError::ConnectionClosed)
        }
    }
}

#[derive(Debug)]
enum ConnectionError {
    ConnectionClosed,
    PacketCreateError(String),
}


impl Error for ConnectionError {}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_type = match self {
            ConnectionError::ConnectionClosed => {"Connection Closed".to_string()}
            ConnectionError::PacketCreateError(s) => {format!("Error Creating Packet ({s})")}
        };
        write!(f, "ConnectionError: {err_type}.")
    }
}

pub struct Connection<'a> {
    read: Mutex<OwnedReadHalf>,
    write: Mutex<OwnedWriteHalf>,
    state: ConnectionState,
    addr: SocketAddrV4,
    player: Option<&'a Player<'a>>,
}

impl<'a> Connection<'a> {
    pub fn new(stream: TcpStream) -> Self {
        let (read, write) = stream.into_split();
        Connection {read : Mutex::new(read), write : Mutex::new(write),  state: ConnectionState::Handshake, player: None }
    }

    pub fn send_packet(&mut self, packet: impl packet::Clientbound) {
        
    }

    pub async fn read_next_packet(&self) -> Result<packet::SPacket, Box<dyn Error>> {
        //let mut iter = PacketLengthIterator{reader: &mut self.read};

        let reader = &self.read;
        let packet_stream = stream! {
            let mut socket_ro = reader.blocking_lock();
            match socket_ro.read_u8().await {
                Ok(val) => yield val,
                Err(_) =>()
            }
        };
        
        let Ok(packet_size_bytes) = read_var_int_async(Box::pin(packet_stream)).await else {return Err(ConnectionError::ConnectionClosed)?};
        let packet_size_bytes = packet_size_bytes as usize;
        let mut buf = Vec::with_capacity(packet_size_bytes);
        let mut socket_ro = self.read.blocking_lock();
            let Ok(bytes) = socket_ro.read_exact(&mut buf).await else {return Err(ConnectionError::ConnectionClosed)?};
        drop(socket_ro);
        match bytes {
            0=> return Err(ConnectionError::ConnectionClosed)?,
            _=>()
        }
        let mut iter = buf.into_iter();
        let Ok(packet_id) = read_var_int(&mut iter) else {return Err(ConnectionError::ConnectionClosed)?};
        Ok(packet::create_packet(packet_id, self.state, &mut iter)?)
    }

}

pub struct Server<'a> {
    //Server services all connections, including status and login
    connections: Connections<'a>,
    
    //Only used in Play state
    player_names: HashMap<String, &'a Player<'a>>,
    player_uuids: HashMap<Uuid, &'a Player<'a>>,
    players: Vec<Player<'a>>,
}

impl<'a> Server<'static> {
    pub fn new(max_players: usize) -> Self {
        Server { 
            connections: Connections::new(max_players), //we have no guarantee of this, but it should avoid excess reallocation
            player_names : HashMap::with_capacity(max_players), 
            player_uuids : HashMap::with_capacity(max_players), 
            players : Vec::with_capacity(max_players)
        }
    }

    pub fn add_connection(&mut self, tcpstream: TcpStream) -> usize {
        self.connections.add(tcpstream)
    }

    pub fn get_connections(&self) -> &Connections {
        &self.connections
    }

    pub fn get_players(&self) -> &'static Vec<Player> {
        &self.players
    }
}
