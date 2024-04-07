extern crate dashmap;

use dashmap::DashMap;
use server_util::ConnectionState;
use tokio::time::timeout;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::sync::OnceLock;
//use std::error::Error;
use std::sync::Arc;
use std::sync::Weak;

use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;
use crate::connection::ConnectionError;
use crate::data_types::text_component::Nbt;
use crate::data_types::TextComponent;
use crate::entity::entity_base::EntityBase;
use crate::packet;
use crate::packet::configuration::CDisconnect_Config;
use crate::packet::play::CDisconnect_Play;
use crate::packet::Clientbound;
use crate::packet::SPacket;
use crate::TIMEOUT;
use crate::connection::Connection;
use crate::packet::login::CDisconnect_Login;
use crate::player_data::PlayerData;


#[derive(Debug)]
pub struct PermissionError;

impl Error for PermissionError {}

impl std::fmt::Display for PermissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Permission does not exist.")
    }
}

pub struct Player {
    connected: Mutex<bool>,
    id: OnceLock<i32>,
    eid: OnceLock<i32>,
    name: String,
    uuid: Uuid,
    connection: RwLock<Connection>,
    data: RwLock<Option<PlayerData>>,
    recv_queue: std::sync::Mutex<VecDeque<SPacket>>,
    permissions: std::sync::RwLock<Box<HashMap<String, bool>>>,
}

impl<'a> Player {
    pub fn new(name: String, uuid: Uuid, connection: Connection) -> Self {
        Player { 
            connected : Mutex::new(true),
            id : OnceLock::new(), //temp value is changed quickly
            eid : OnceLock::new(),
            name : name, 
            uuid : uuid, 
            connection : RwLock::new(connection),
            data : RwLock::new(None),
            recv_queue : std::sync::Mutex::new(VecDeque::new()),
            permissions : std::sync::RwLock::new(Box::new(HashMap::new())),
        }
    }

    pub(in crate) fn get_connection(&self) -> &RwLock<Connection> {
        &self.connection
    }

    pub async fn is_connected(&self) -> bool {
        let lock = self.connected.lock().await;
        *lock
    }

    pub(in crate::player) fn set_id(&self, id: i32) -> Result<(), i32> {
        self.id.set(id)
    }

    pub fn get_id(&self) -> i32 {
        match self.id.get() {
            Some(some) => *some,
            None => -1
        }
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
        let connection_lock = self.connection.read().await;
            connection_lock.read_next_packet().await
    }

    pub async fn queue_packet(&self, packet: packet::SPacket) {
        let mut lock = self.recv_queue.lock().unwrap();
        lock.push_back(packet);
    }

    pub async fn send_packet(&self, packet: impl Clientbound) -> Result<(), ConnectionError> {
        let connection_lock = self.connection.read().await;
            connection_lock.send_packet(packet).await
    }

    pub async fn get_connection_state(&self) -> ConnectionState {
        let connection_lock = self.connection.read().await;
            connection_lock.get_connection_state().await
    }

    pub async fn disconnect(&self, reason: &str) {
        self.disconnect_tc(TextComponent::builder().text(reason).build()).await
    }

    pub async fn disconnect_tc(&self, reason: TextComponent<Nbt>) {
        *self.connected.lock().await = false;
        let player_id : i32;
        match self.id.get() {
            Some(some) => player_id = *some,
            None => return,
        }
        crate::THE_SERVER.drop_player_by_id(player_id);
        match timeout(TIMEOUT, self.get_connection_state()).await {
            Ok(connection_state) => match connection_state {
                server_util::ConnectionState::Login => {
                    timeout(TIMEOUT, self.send_packet(
                        CDisconnect_Login::new(format!("'{}'", reason.get_text().unwrap()))
                    )).await.unwrap_or(Ok(())).unwrap_or(())
                },
                server_util::ConnectionState::Configuration => {
                    timeout(TIMEOUT, self.send_packet(
                        CDisconnect_Config::new(format!("'{}'", reason.get_text().unwrap()))
                    )).await.unwrap_or(Ok(())).unwrap_or(())
                },
                server_util::ConnectionState::Play => {
                    timeout(TIMEOUT, self.send_packet(
                        CDisconnect_Play::new(reason)
                    )).await.unwrap_or(Ok(())).unwrap_or(())
                }
                _ => ()
            },
            Err(_) => ()
        }
    }

    pub fn has_permission(&self, permission: String) -> Result<bool, PermissionError> {
        let perms_lock = self.permissions.read().unwrap();
        match perms_lock.get(&permission) {
            Some(val) => Ok(*val),
            None => Err(PermissionError),
        }
    }
}

impl EntityBase for Player {
    fn get_eid(&self) -> i32
        where Self: Sized {
        match self.eid.get() {
            Some(some) => *some,
            None => -1,
        }
    }

    fn get_position(&self) -> crate::data_types::vec_3d::Vec3d
        where Self: Sized {
        todo!()
    }

    fn is_on_fire(&self) -> bool
        where Self: Sized {
        false
    }

    fn get_look(&self) -> crate::data_types::Rotation
        where Self: Sized {
        todo!()
    }

    fn get_world(&self) -> Option<Weak<crate::world::World>>
        where Self: Sized {
        todo!()
    }
    
    fn is_crouching(&self) -> bool
        where Self: Sized {
        false
    }
    
    fn is_sprinting(&self) -> bool
        where Self: Sized {
        false
    }
    
    fn is_swimming(&self) -> bool
        where Self: Sized {
        false
    }
    
    fn is_invisible(&self) -> bool
        where Self: Sized {
        false
    }
    
    fn is_glowing(&self) -> bool
        where Self: Sized {
        false
    }
    
    fn is_using_elytra(&self) -> bool
        where Self: Sized {
        false
    }
}


//TODO (maybe): Move all of this stuff inside server.rs
///The struct that holds the players
pub struct Players {
    players: DashMap<i32, Arc<Player>>,
    idx: Mutex<i32>,
}

impl Players {
    pub fn new(max_players: i32) -> Self {
        Players { 
            players : DashMap::with_capacity(max_players as usize), 
            idx : Mutex::new(0),
        }
    }

    pub async fn add(&self, player: Player) -> Arc<Player>
    {
        let mut idx_lock = self.idx.lock().await;
        let idx_value = *idx_lock;

        let _ = player.set_id(idx_value);
        
        let player_arc = Arc::new(player);

        let mut connection_lock = player_arc.get_connection().write().await;
            connection_lock.set_owner(player_arc.clone());
        drop(connection_lock);

        self.players.insert(idx_value, player_arc.clone());

        *idx_lock += 1;
        drop(idx_lock);
        player_arc
    }
    /// The reference 
    pub fn get_by_id(&self, id: i32) -> Option<Weak<Player>> {
        match self.players.get(&id) {
            Some(player_ref) => Some(Arc::downgrade(&player_ref)),
            None => None
        }
    }
    
    pub fn drop_by_id(&self, id: i32) {
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

    pub fn get_num_players(&self) -> i32 {
        self.players.len() as i32
    }
}


