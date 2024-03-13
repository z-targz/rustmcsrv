
use std::collections::HashMap;
use std::cmp::min;

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
use crate::data::read_var_int_async;

use async_stream::stream;



type PlayerID = usize;
pub struct Server<'a> {
    //Server services all connections, including status and login
    connections: Vec<Connection<'a>>,
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

    pub fn add_connection(&mut self, tcpstream: TcpStream) -> usize {
        self.connections.push(Connection::new(tcpstream));
        self.connections.len()-1
    }

    pub fn get_connections(&self) -> &Vec<Connection> {
        &self.connections
    }

    pub fn get_players(&self) -> &'static Vec<Player> {
        &self.players
    }
}



pub struct Connection<'a> {
    read: Mutex<OwnedReadHalf>,
    write: Mutex<OwnedWriteHalf>,
    state: ConnectionState,
    player: Option<&'a Player<'a>>,
    internal_buffer: Box<Vec<u8>>,
}

#[derive(Debug)]
enum ConnectionError {
    ConnectionClosed,
}


impl Error for ConnectionError {}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_type = match self {
            ConnectionError::ConnectionClosed => {"Connection Closed"}
        };
        write!(f, "ConnectionError: {err_type}.")
    }
}

impl<'a> Connection<'a> {
    pub fn new(stream: TcpStream) -> Self {
        let (read, write) = stream.into_split();
        Connection {read : Mutex::new(read), write : Mutex::new(write),  state: ConnectionState::Handshake, player: None, internal_buffer: Vec::with_capacity() }
    }

    pub fn send_packet(&mut self, packet: impl packet::Clientbound) {
        
    }

    pub async fn read_next_packet(&mut self) -> Result<packet::SPacket, ConnectionError> {
        //let mut iter = PacketLengthIterator{reader: &mut self.read};

        let reader = &self.read;
        let packet_stream = stream! {
            /*let mut buf: [u8;1] = [0u8];
            let mut socket_ro = reader.blocking_lock();
            let Ok(bytes) = socket_ro.read_exact(&mut buf).await else { return; };
            drop(socket_ro);
            match bytes {
                0=> yield buf[0],
                _=> ()
            }*/
            let mut socket_ro = reader.blocking_lock();
            match socket_ro.read_u8().await {
                Ok(val) => yield val,
                Err(_) =>()
            }
        };
        
        let Ok(packet_size_bytes) = read_var_int_async(Box::pin(packet_stream)).await else {return Err(ConnectionError::ConnectionClosed)};
        let packet_size_bytes = packet_size_bytes as usize;
        let mut buf = Vec::with_capacity(packet_size_bytes);
        let mut socket_ro = self.read.blocking_lock();
            let Ok(bytes) = socket_ro.read_exact(&mut buf).await else {return Err(ConnectionError::ConnectionClosed)};
        drop(socket_ro);
        match bytes {
            0=> return Err(ConnectionError::ConnectionClosed),
            _=>()
        }
        
        //while buf.len() < packet_size_bytes {
            
        //}
        Err(ConnectionError::ConnectionClosed)
    }

}

struct PacketLengthIterator<'a> {
    reader: &'a Mutex<OwnedReadHalf>,
}

impl<'a> Iterator for PacketLengthIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ro_socket = self.reader.blocking_lock();
        let mut buf: [u8;1] = [0u8];
        let Ok(bytes) = ro_socket.read_exact(&mut buf);
        drop(ro_socket);
        match bytes {

        }
        Some(buf[0])
    }
}

