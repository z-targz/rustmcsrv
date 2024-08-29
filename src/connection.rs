use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Weak};
use std::fmt::Debug;

use log::{debug, trace};
use server_util::error::ProtocolError;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

use server_util::ConnectionState;
use tokio::sync::Mutex;
use tokio::time::error::Elapsed;
use tokio::time::timeout;

use crate::packet::{self, Clientbound, CreatePacketError};
use crate::{data_types::*, TIMEOUT};
use crate::player::Player;

#[derive(Debug)]
pub enum ConnectionError {
    ConnectionClosed,
    PacketCreateError(String),
    Timeout,
    IO(String),
    ProtocolError(String),
    Other(String),
}

impl Error for ConnectionError {}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_type = match self {
            ConnectionError::ConnectionClosed => {"Connection Closed".to_string()}
            ConnectionError::PacketCreateError(s) => {format!("Error Creating Packet ({s})")}
            ConnectionError::Timeout => {"Timed out".to_string()}
            ConnectionError::IO(s) => {format!("I/O error ({s})")},
            ConnectionError::ProtocolError(s) => {format!("{s}")}
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

impl From<ProtocolError> for ConnectionError {
    fn from(value: ProtocolError) -> Self {
        ConnectionError::ProtocolError(value.to_string())
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

impl From<tokio::io::Error> for ConnectionError {
    fn from(value: tokio::io::Error) -> Self {
        ConnectionError::IO(value.to_string())
    }
}

pub struct Connection {
    read: OwnedReadHalf,
    write: OwnedWriteHalf,
    state: ConnectionState,
    compressed: bool,
    addr: SocketAddr,
    owner: Option<Weak<Player>>, //we will never access this from outside this struct, so it doesn't need to be thread-safe
}

impl Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection")
        .field("state", &self.state)
        .field("compressed", &self.compressed)
        .field("addr", &self.addr)
        .field_with("owner", |f| match &self.owner {
            Some(weak) => match Weak::upgrade(weak) {
                Some(player) => {
                    
                    f.debug_struct("Player")
                        .field("connected", player.is_connected())
                        .field("id", &player.get_id())
                        .field("name", &player.get_name())
                        .field("uuid", &player.get_uuid())
                        .field("data", player.get_data())
                        .field("permissions", player.get_permissions())
                        .finish()
                },
                None => Ok(()),
            }
            None => Ok(()),
        })
        .finish()
    }
}

impl Connection {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        let (read, write) = stream.into_split();
        Connection {read : read, write : write, state : ConnectionState::Handshake, compressed: false, addr : addr, owner : None }
    }

    pub fn set_owner(&mut self, owner: Arc<Player>) {
        let weak_owner = Arc::downgrade(&owner);
        self.owner = Some(weak_owner);
    }

    pub fn get_owner(&mut self) -> Option<Weak<Player>> {
        match &self.owner {
            Some(some_owner) => Some(some_owner.clone()),
            None => None
        }
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn is_compressed(&self) -> bool {
        self.compressed
    }

    pub fn set_compressed(&mut self, compressed: bool) {
        self.compressed = compressed;
    }

    pub async fn set_connection_state(&mut self, state: ConnectionState) {
        self.state = state;
    }

    pub fn get_connection_state(&self) -> ConnectionState {
        self.state
    } //lock is dropped

    pub async fn drop(&mut self) {
        let _ = tokio::time::timeout(crate::TIMEOUT * 3, self.write.shutdown());
    }

    pub async fn send_packet(&mut self, packet: impl Clientbound) -> Result<(), ConnectionError> {
        match timeout(TIMEOUT, self.write.write_all(packet.to_be_bytes().as_slice())).await {
            Ok(result) => match result {
                Ok(_) => Ok(()),
                Err(e) => Err(e)?
            },
            Err(e) => Err(e)?
        }
    }

    pub async fn read_next_packet(&mut self) -> Result<packet::SPacket, ConnectionError> {
        let mut buff: [u8; 5] = [0u8; 5];
        let num_bytes = self.read.peek(&mut buff).await;
        match num_bytes {
            Ok(0) => return Err(ConnectionError::ConnectionClosed)?,
            Err(_) => return Err(ConnectionError::ConnectionClosed)?,
            _ => ()
        }
        trace!("Raw packet data (first 10): {:?}", buff);
        
        let mut header_iter = buff.to_vec().into_iter();

        let packet_size_bytes = VarInt::from_protocol_iter(&mut header_iter)?.get() as usize;
        trace!("Packet size: {packet_size_bytes}");

        let header_size = 5 - header_iter.count();

        trace!("Reading packet data...");
        let mut buf = Vec::with_capacity(packet_size_bytes);
        buf.resize(header_size + packet_size_bytes, 0u8);

        let Ok(bytes) = timeout(TIMEOUT, self.read.read_exact(buf.as_mut_slice())).await? else {return Err(ConnectionError::ConnectionClosed)?};
        trace!("Read {bytes} bytes.");
        match bytes {
            0=> return Err(ConnectionError::ConnectionClosed)?,
            _=>()
        }
        trace!("Packet data: {:?}.", buf);

        let mut iter = buf.into_iter();
        let _ = iter.advance_by(header_size); //This *might* be the cause of a bug in the future. Keep your eyes peeled.

        let packet_id: i32 = VarInt::from_protocol_iter(&mut iter)?.into();
        trace!("Packet id: {packet_id}");

        trace!("Creating packet...");
        Ok(packet::create_packet(packet_id, self.state, &mut iter)?)
        //drop(lock)
    }

}

impl Drop for Connection {
    fn drop(&mut self) {
        match &self.owner {
            Some(owner) => match owner.upgrade() {
                Some(parent) => {
                    crate::THE_SERVER.drop_player_by_id(parent.get_id());
                    let name = parent.get_name();
                    debug!("Dropped player: {name}");
                },
                None => ()
            },
            None =>()
        }
    }
}