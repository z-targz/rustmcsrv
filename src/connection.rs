use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Weak};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};

use server_util::ConnectionState;
use tokio::time::error::Elapsed;
use tokio::time::timeout;

use crate::packet::{self, Clientbound, CreatePacketError};
use crate::{data::*, TIMEOUT};
use crate::player::Player;

#[derive(Debug)]
pub enum ConnectionError {
    ConnectionClosed,
    PacketCreateError(String),
    Timeout,
    Other(String),
}

impl Error for ConnectionError {}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_type = match self {
            ConnectionError::ConnectionClosed => {"Connection Closed.".to_string()}
            ConnectionError::PacketCreateError(s) => {format!("Error Creating Packet ({s}).")}
            ConnectionError::Timeout => {"Timed out.".to_string()}
            ConnectionError::Other(s) => {format!("({s})")}
        };
        write!(f, "ConnectionError: {err_type}.")
    }
}

impl From<Box<dyn Error + Send + Sync>> for ConnectionError {
    fn from(value: Box<dyn Error + Send + Sync>) -> Self {
        ConnectionError::Other(value.to_string())
    }
}

impl From<CreatePacketError> for ConnectionError {
    fn from(value: CreatePacketError) -> Self {
        ConnectionError::PacketCreateError(value.to_string())
    }
}

impl From<Elapsed> for ConnectionError {
    fn from(_: Elapsed) -> Self {
        ConnectionError::Timeout
    }
}

pub struct Connection {
    read: Mutex<OwnedReadHalf>,
    write: Mutex<OwnedWriteHalf>,
    state: RwLock<ConnectionState>,
    compressed: Mutex<bool>,
    addr: SocketAddr,
    owner: Mutex<Option<Weak<Player>>>, //we will never access this from outside this struct, so it doesn't need to be thread-safe
}

impl Connection {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        let (read, write) = stream.into_split();
        Connection {read : Mutex::new(read), write : Mutex::new(write), state : RwLock::new(ConnectionState::Handshake), compressed: Mutex::new(false), addr : addr, owner : Mutex::new(None) }
    }

    pub async fn set_owner(&self, owner: Arc<Player>) {
        let mut owner_lock = self.owner.lock().await;
        let weak_owner = Arc::downgrade(&owner);
        *owner_lock = Some(weak_owner);
    }

    pub fn get_owner(&mut self) -> Option<Weak<Player>> {
        self.owner.get_mut().clone()
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }

    pub async fn is_compressed(&self) -> bool {
        let compressed_lock = self.compressed.lock().await;
        *compressed_lock
    }

    pub async fn set_compressed(&self, compressed: bool) {
        let mut compressed_lock = self.compressed.lock().await;
        *compressed_lock = compressed;
        drop(compressed_lock);
    }

    pub async fn set_connection_state(&self, state: ConnectionState) {
        let mut lock = self.state.write().await;
        *lock = state;
        drop(lock);
    }

    pub async fn get_connection_state(&self) -> ConnectionState {
        let lock = self.state.read().await;
        lock.clone()
    } //lock is dropped

    pub async fn drop(&self) {
        let mut lock = self.write.lock().await;
        let _ = lock.shutdown();
        drop(lock);
    }

    pub async fn send_packet(&self, packet: impl Clientbound) -> Result<(), tokio::io::Error> {
        let mut socket_w = self.write.lock().await;
        match socket_w.write_all(packet.to_be_bytes().as_slice()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn read_next_packet(&self) -> Result<packet::SPacket, ConnectionError> {
        let mut buff: [u8; 5] = [0u8; 5];
        let mut socket_ro_peek = timeout(TIMEOUT, self.read.lock()).await?;
        let num_bytes = socket_ro_peek.peek(&mut buff).await;
        drop(socket_ro_peek);
        match num_bytes {
            Ok(0) => return Err(ConnectionError::ConnectionClosed)?,
            Err(_) => return Err(ConnectionError::ConnectionClosed)?,
            _ => ()
        }
        println!("Raw packet data (first 10): {:?}", buff);
        
        let mut header_iter = buff.to_vec().into_iter();

        let packet_size_bytes = read_var_int(&mut header_iter)? as usize;
        println!("Packet size: {packet_size_bytes}");

        let header_size = 5 - header_iter.count();

        println!("Reading packet data...");
        let mut buf = Box::new(Vec::with_capacity(packet_size_bytes));
        buf.resize(header_size + packet_size_bytes, 0u8);

        let mut socket_ro = self.read.lock().await;
            let Ok(bytes) = timeout(TIMEOUT, socket_ro.read_exact(buf.as_mut_slice())).await? else {return Err(ConnectionError::ConnectionClosed)?};
        drop(socket_ro);
        println!("Read {bytes} bytes.");
        match bytes {
            0=> return Err(ConnectionError::ConnectionClosed)?,
            _=>()
        }
        println!("Packet data: {:?}.", buf);

        let mut iter = buf.into_iter();
        let _ = iter.advance_by(header_size); //This *might* be the cause of a bug in the future. Keep your eyes peeled.

        let packet_id = read_var_int(&mut iter)?;
        println!("Packet id: {packet_id}");

        println!("Creating packet...");
        let lock = self.state.read().await;
        Ok(packet::create_packet(packet_id, *lock, &mut iter)?)
        //drop(lock)
    }

}

impl Drop for Connection {
    fn drop(&mut self) {
        match self.owner.get_mut() {
            Some(owner) => match owner.upgrade() {
                Some(parent) => {
                    crate::THE_SERVER.drop_player_by_id(parent.get_id());
                    let name = parent.get_name();
                    println!("Dropped player: {name}");
                },
                None => ()
            },
            None =>()
        }
    }
}