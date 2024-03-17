extern crate dashmap;

use dashmap::DashMap;
use tokio::time::timeout;

use std::error::Error;
use std::sync::Arc;
use std::sync::Weak;

use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;
use crate::connection::ConnectionError;
use crate::packet;
use crate::packet::Clientbound;
use crate::TIMEOUT;
use crate::{connection::Connection, packet::login::CDisconnect};

#[derive(Debug)]
pub enum PlayerError {
    PlayerNotFound
}

impl Error for PlayerError {}

impl std::fmt::Display for PlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_type = match self {
            PlayerError::PlayerNotFound => {"Player Not Found".to_string()}

        };
        write!(f, "PlayerError: {err_type}.")
    }
}

pub struct Player {
    id: usize,
    name: String,
    uuid: Uuid,
    connection: Connection,
    data: RwLock<Option<PlayerData>>,
}

impl<'a> Player {
    pub fn new(name: String, uuid: Uuid, connection: Connection) -> Self {
        Player { 
            id : usize::MAX, //temp value is changed quickly
            name : name, 
            uuid : uuid, 
            connection : connection,
            data : RwLock::new(None),
        }
    }

    pub(in crate::player) fn get_connection(&self) -> &Connection {
        &self.connection
    }

    pub(in crate::player) fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub async fn set_player_data(&self, data: PlayerData) {
        let mut lock = self.data.write().await;
            *lock = Some(data);
        drop(lock);
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub async fn read_next_packet(&self) -> Result<packet::SPacket, ConnectionError> {
        self.connection.read_next_packet().await
    }

    pub async fn send_packet(&self, packet: impl Clientbound) -> Result<(), tokio::io::Error> {
        self.connection.send_packet(packet).await
    }

    pub async fn disconnect(&self, reason: &String) {
        crate::THE_SERVER.drop_player_by_id(self.id);
        match timeout(TIMEOUT, self.connection.get_connection_state()).await {
            Ok(connection_state) => match connection_state {
                server_util::ConnectionState::Login => self.connection.send_packet(CDisconnect::new(reason.clone())).await.unwrap_or(()),
                server_util::ConnectionState::Configuration => (), //TODO: fill this once this state is implemented
                server_util::ConnectionState::Play => (), //TODO: fill this once this state is implemented
                _ => ()
            },
            Err(_) => ()
        }
    }
}

//TODO: Move all of this stuff inside server.rs
///The struct that holds the players
pub struct Players {
    players: DashMap<usize, Arc<Player>>,
    idx: Mutex<usize>,
}

impl Players {
    pub fn new(max_players: usize) -> Self {
        Players { 
            players : DashMap::with_capacity(max_players), 
            idx : Mutex::new(0),
        }
    }
    pub async fn add(&self, mut player: Player) -> Arc<Player>
    {
        let mut idx_lock = self.idx.lock().await;
        let idx_value = *idx_lock;

        player.set_id(idx_value);
        
        let player_arc = Arc::new(player);

        player_arc.get_connection().set_owner(player_arc.clone()).await;

        self.players.insert(idx_value, player_arc.clone());

        *idx_lock += 1;
        drop(idx_lock);
        player_arc
    }
    /// The reference 
    pub fn get_by_id(&self, idx: usize) -> Option<Weak<Player>> {
        match self.players.get(&idx) {
            Some(x) => Some(Arc::downgrade(&x)),
            None => None
        }
    }
    
    pub fn drop_by_id(&self, id: usize) {
        if self.players.contains_key(&id) {
            self.players.remove(&id);
        }
    }

    pub fn drop_by_uuid(&self, uuid: Uuid) {
        match self.players.iter().find(|x| x.get_uuid() == uuid) {
            Some(ref_multi) => { self.players.remove(ref_multi.key()); } ,
            None => (),
        }
    }

    pub fn players_iter(&self) -> impl Iterator<Item = Weak<Player>> + '_ {
        self.players.iter().map(|x| Arc::downgrade(x.value()))
    }

    pub fn get_by_name(&self, name: &String) -> Option<Weak<Player>> {
        match self.players.iter().find(|x| x.get_name() == name) {
            Some(ref_multi) => Some(Arc::downgrade(&ref_multi)),
            None => None,
        }
    }

    pub fn get_by_uuid(&self, uuid: Uuid) -> Option<Weak<Player>> {
        match self.players.iter().find(|x| x.get_uuid() == uuid) {
            Some(ref_multi) => Some(Arc::downgrade(&ref_multi)),
            None => None,
        }
    }
}

pub struct PlayerData {
    test: String,
    test2: i32,
}

impl PlayerData {

}