use core::net::SocketAddr;


use std::collections::HashMap;
use std::cmp::min;

use std::net::SocketAddrV4;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::error::Error;

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::io::BufReader;

use uuid::Uuid;

use futures::StreamExt;
use tokio::stream;

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
    pub fn add(&mut self, tcpstream: TcpStream, addr: SocketAddr) -> usize {
        self.connections.insert(self.idx, Connection::new(tcpstream, addr));
        self.idx += 1;
        self.idx-1
    }
    pub fn get_by_id(&self, id: usize) -> Result<&'a Connection, ConnectionError> {
        match self.connections.get(&id) {
            Some(connection) => Ok(connection),
            None => Err(ConnectionError::ConnectionClosed)
        }
    }

    pub fn get_mut_by_id(&mut self, id: usize) -> Result<&'a mut Connection, ConnectionError> {
        match self.connections.get_mut(&id) {
            Some(connection) => Ok(connection),
            None => Err(ConnectionError::ConnectionClosed)
        }
    }

    pub fn set_connection_state_by_id(&mut self, id: usize, state: ConnectionState) -> Result<(), ConnectionError> {
        match self.connections.get_mut(&id) {
            Some(connection) => connection.set_connection_state(state),
            None => return Err(ConnectionError::ConnectionClosed)
        }
        Ok(())
    }

    pub fn drop_by_id(&mut self, id:usize) {
        self.connections.remove(&id);
    }
}

#[derive(Debug)]
pub enum ConnectionError {
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
    addr: SocketAddr,
    player: Option<&'a Player<'a>>,
}

impl<'a> Connection<'a> {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        let (read, write) = stream.into_split();
        Connection {read : Mutex::new(read), write : Mutex::new(write),  state : ConnectionState::Handshake, addr : addr, player : None}
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn set_connection_state(&mut self, state: ConnectionState) {
        self.state = state;
    }

    pub fn send_packet(&mut self, packet: impl packet::Clientbound) {
        
    }

    pub async fn read_next_packet(&self) -> Result<packet::SPacket, Box<dyn Error + Send + Sync>> {
        let mut buff: [u8; 5] = [0u8; 5];
        let mut socket_ro_peek = self.read.lock().await;
        let num_bytes = socket_ro_peek.peek(&mut buff).await;
        drop(socket_ro_peek);
        match num_bytes {
            Ok(0) => return Err(ConnectionError::ConnectionClosed)?,
            Err(_) => return Err(ConnectionError::ConnectionClosed)?,
            _ => ()
        }
        //println!("Raw packet data (first 10): {:?}", buff);
        
        let mut header_iter = buff.to_vec().into_iter();

        let packet_size_bytes = read_var_int(&mut header_iter)? as usize;
        //println!("Packet size: {packet_size_bytes}");

        let header_size = 5 - header_iter.count();

        //println!("Reading packet data...");
        let mut buf = Box::new(Vec::with_capacity(packet_size_bytes));
        buf.resize(header_size + packet_size_bytes, 0u8);

        let mut socket_ro = self.read.lock().await;
            
            let Ok(bytes) = socket_ro.read_exact(buf.as_mut_slice()).await else {return Err(ConnectionError::ConnectionClosed)?};
        drop(socket_ro);
        //println!("Read {bytes} bytes.");
        match bytes {
            0=> return Err(ConnectionError::ConnectionClosed)?,
            _=>()
        }
        //println!("Packet data: {:?}.", buf);

        let mut iter = buf.into_iter();
        let _ = iter.advance_by(header_size); //This *might* be the cause of a bug in the future. Keep your eyes peeled.

        let packet_id = read_var_int(&mut iter)?;
        //println!("Packet id: {packet_id}");

        //println!("Creating packet...");
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

    pub fn add_connection(&mut self, tcpstream: TcpStream, addr: SocketAddr) -> usize {
        self.connections.add(tcpstream, addr)
    }

    fn get_connections(&self) -> &Connections {
        &self.connections
    }

    pub fn get_players(&self) -> &'static Vec<Player> {
        &self.players
    }

    pub fn get_connection_by_id(&self, id: usize) -> Result<&'a Connection, ConnectionError> {
        self.connections.get_by_id(id)
    }

    pub fn set_connection_state_by_id(&mut self, id: usize, state: ConnectionState) -> Result<(), ConnectionError> {
        self.connections.set_connection_state_by_id(id, state)
    }

    pub fn drop_connection_by_id(&mut self, id:usize) {
        self.connections.drop_by_id(id);
    }
}
