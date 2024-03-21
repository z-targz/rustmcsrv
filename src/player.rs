extern crate dashmap;

use dashmap::DashMap;
use server_util::ConnectionState;
use tokio::time::timeout;

//use std::error::Error;
use std::sync::Arc;
use std::sync::Weak;

use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;
use crate::connection::ConnectionError;
use crate::data::CJSONTextComponent;
use crate::packet;
use crate::packet::configuration::CDisconnect_Config;
use crate::packet::Clientbound;
use crate::TIMEOUT;
use crate::connection::Connection;
use crate::packet::login::CDisconnect_Login;

/*
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
}*/

pub struct Player {
    id: usize,
    name: String,
    uuid: Uuid,
    connection: Mutex<Connection>,
    data: RwLock<Option<PlayerData>>,
}

impl<'a> Player {
    pub fn new(name: String, uuid: Uuid, connection: Connection) -> Self {
        Player { 
            id : usize::MAX, //temp value is changed quickly
            name : name, 
            uuid : uuid, 
            connection : Mutex::new(connection),
            data : RwLock::new(None),
        }
    }

    pub(in crate) fn get_connection(&self) -> &Mutex<Connection> {
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
        let mut connection_lock = self.connection.lock().await;
            connection_lock.read_next_packet().await
    }

    pub async fn send_packet(&self, packet: impl Clientbound) -> Result<(), ConnectionError> {
        let mut connection_lock = self.connection.lock().await;
            connection_lock.send_packet(packet).await
    }

    pub async fn get_connection_state(&self) -> ConnectionState {
        let connection_lock = self.connection.lock().await;
            connection_lock.get_connection_state().await
    }


    pub async fn disconnect(&self, reason: &str) {
        crate::THE_SERVER.drop_player_by_id(self.id);
        let json_text_component = CJSONTextComponent::from_str(reason).color(0x4);
        //println!("Raw text component: {}", json_text_component.to_string());
        match timeout(TIMEOUT, self.get_connection_state()).await {
            Ok(connection_state) => match connection_state {
                server_util::ConnectionState::Login => {
                    timeout(TIMEOUT, self.send_packet(
                        CDisconnect_Login::new(json_text_component)
                    )).await.unwrap_or(Ok(())).unwrap_or(())
                },
                server_util::ConnectionState::Configuration => {
                    timeout(TIMEOUT, self.send_packet(
                        CDisconnect_Config::new(json_text_component)
                    )).await.unwrap_or(Ok(())).unwrap_or(())
                },
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

        let mut connection_lock = player_arc.get_connection().lock().await;
            connection_lock.set_owner(player_arc.clone());
        drop(connection_lock);

        self.players.insert(idx_value, player_arc.clone());

        *idx_lock += 1;
        drop(idx_lock);
        player_arc
    }
    /// The reference 
    pub fn get_by_id(&self, id: usize) -> Option<Weak<Player>> {
        match self.players.get(&id) {
            Some(player_ref) => Some(Arc::downgrade(&player_ref)),
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

pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator
}

pub struct PlayerData {
    gamemode: Gamemode,
}

impl PlayerData {

}