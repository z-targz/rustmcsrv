extern crate dashmap;

use dashmap::DashMap;

use std::borrow::Borrow;
use std::{borrow::BorrowMut, error::Error};
use std::sync::Arc;

use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;
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
    pub async fn new(name: String, uuid: Uuid, mut connection: Connection) -> Arc<Self> {
        let mut player = Player { 
            id : usize::MAX, //temp value is changed quickly
            name : name, 
            uuid : uuid, 
            connection : connection,
            data : RwLock::new(None),
        };
        let arc = Arc::new(player);
        arc.set_connection_owner(arc.clone()).await;
        arc
    }

    async fn set_connection_owner(&self, arc: Arc<Player>) -> Arc<Player> {
        let mut_connection = &self.connection;
        let arc2 = arc.clone();
        mut_connection.set_owner(arc).await;
        arc2
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

    pub async fn disconnect(&self, reason: &String) -> Result<(), tokio::io::Error> {
        match self.connection.get_connection_state().await {
            server_util::ConnectionState::Login => self.connection.send_packet(CDisconnect::new(reason.clone())).await?,
            server_util::ConnectionState::Configuration => (), //TODO: fill this once this state is implemented
            server_util::ConnectionState::Play => (), //TODO: fill this once this state is implemented
            _ => ()
        }
        crate::THE_SERVER.drop_player_by_uuid(self.uuid);
        Ok(())
    }
}



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
    pub async fn add(&self, player: Arc<Player>) -> usize
    {
        let mut idx_lock = self.idx.lock().await;
        let idx_value = *idx_lock;

        self.players.insert(idx_value, player);

        *idx_lock += 1;
        drop(idx_lock);
        idx_value
    }
    /// The reference 
    pub fn get_by_id(&self, idx: usize) -> Option<Arc<Player>> {
        match self.players.get(&idx) {
            Some(x) => Some(x.clone()),
            None => None
        }
    }
    
    pub fn drop_by_idx(&self, idx: usize) {
        if self.players.contains_key(&idx) {
            self.players.remove(&idx);
        }
    }

    pub fn drop_by_uuid(&self, uuid: Uuid) {
        match self.players.iter().find(|x| x.get_uuid() == uuid) {
            Some(ref_multi) => { self.players.remove(ref_multi.key()); } ,
            None => (),
        }
    }

    pub fn players_iter(&self) -> impl Iterator<Item = Arc<Player>> + '_ {
        self.players.iter().map(|x| x.value().clone())
    }

    pub fn get_by_name(&self, name: &String) -> Option<Arc<Player>> {
        match self.players.iter().find(|x| x.get_name() == name) {
            Some(ref_multi) => Some(ref_multi.clone()),
            None => None,
        }
    }

    pub fn get_by_uuid(&self, uuid: Uuid) -> Option<Arc<Player>> {
        match self.players.iter().find(|x| x.get_uuid() == uuid) {
            Some(ref_multi) => Some(ref_multi.clone()),
            None => None,
        }
    }

}

pub struct PlayerData {
    test: String,
    test2: i32,
}

impl PlayerData {
    fn get_test(&self) -> &String {
        &self.test
    }
    fn get_test2(&self) -> &i32 {
        &self.test2
    }
    fn asdf(data: PlayerData) {
        data.get_test2().to_owned();
    }
}