extern crate dashmap;

use dashmap::DashMap;
use regex::Regex;
use server_util::ConnectionState;
use tokio::time::timeout;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Debug;
use std::sync::MutexGuard;
use std::sync::OnceLock;
//use std::error::Error;
use std::sync::Arc;
use std::sync::Weak;

use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;
use crate::connection::ConnectionError;
use crate::data_types::text_component::Nbt;
use crate::data_types::text_component::TextComponentBuilder;
use crate::data_types::TextComponent;
use crate::entity::entities::player::EntityPlayer;
use crate::packet;
use crate::packet::configuration::CDisconnect_Config;
use crate::packet::play::CDisconnect_Play;
use crate::packet::play::CSystemChatMessage;
use crate::packet::Clientbound;
use crate::packet::SPacket;

use crate::TIMEOUT;
use crate::connection::Connection;
use crate::packet::login::CDisconnect_Login;


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
    name: String,
    uuid: Uuid,
    connection: Mutex<Connection>,
    data: RwLock<Option<EntityPlayer>>,
    recv_queue: Mutex<VecDeque<SPacket>>,
    send_queue: Mutex<VecDeque<Vec<u8>>>,
    permissions: std::sync::RwLock<Permissions>,
}
pub type Permissions = Vec<Regex>;


impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("connected", &self.connected)
            .field("id", &self.id)
            .field("name", &self.name)
            .field("uuid", &self.uuid)
            .field_with("connection",
            |f| { 
                let lock = &self.connection.blocking_lock();
                f.debug_struct("Connection") 
                    .field("state", &lock.get_connection_state())
                    .field("compressed", &lock.is_compressed())
                    .field("addr", &lock.get_addr())
                    .finish()
                }
            )
            .field("data", &self.data)
            .field("recv_queue", &self.recv_queue)
            .field("send_queue", &self.send_queue)
            .field("permissions", &self.permissions)
            .finish()
    }
}

impl Player {
    pub fn new(name: String, uuid: Uuid, connection: Connection) -> Self {
        Player { 
            connected : Mutex::new(true),
            id : OnceLock::new(), //temp value is changed quickly
            name : name, 
            uuid : uuid, 
            connection : Mutex::new(connection),
            data : RwLock::new(None),
            recv_queue : Mutex::new(VecDeque::new()),
            send_queue : Mutex::new(VecDeque::new()),
            permissions : std::sync::RwLock::new(Vec::new()),
        }
    }



    pub fn get_connection(&self) -> &Mutex<Connection> {
        &self.connection
    }

    pub fn is_connected(&self) -> &Mutex<bool> {
        &self.connected
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



    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub async fn read_next_packet(&self) -> Result<packet::SPacket, ConnectionError> {
        let mut connection_lock = self.connection.lock().await;
            connection_lock.read_next_packet().await
    }

    pub async fn queue_packet(&self, packet: packet::SPacket) {
        let mut lock = self.recv_queue.lock().await;
        lock.push_back(packet);
    }

    pub async fn send_packet(&self, packet: impl Clientbound) -> Result<(), ConnectionError> {
        let mut connection_lock = self.connection.lock().await;
            connection_lock.send_packet(packet).await
    }

    pub async fn queue_send_packet(&self, packet: impl Clientbound) {
        let mut lock = self.send_queue.lock().await;
        lock.push_back(packet.to_be_bytes());
    }

    pub async fn get_connection_state(&self) -> ConnectionState {
        let connection_lock = self.connection.lock().await;
            connection_lock.get_connection_state()
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
                        CDisconnect_Config::new(reason)
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

    pub fn has_permission(&self, permission: &str) -> bool {
        let perms_lock = self.permissions.read().unwrap();
        for regex in perms_lock.iter() {
            if regex.is_match(permission) {
                return true
            }
        }
        false
    }

    pub fn add_permission(&self, permission: &str) -> Result<(), PermissionError> {
        let is_valid_permission_regex = Regex::new("^([a-zA-Z]+\\.)*(([a-zA-Z]+)|\\*)").unwrap();
        if !is_valid_permission_regex.is_match(permission) {
            return Err(PermissionError)
        }

        let copy = String::from("^") + permission.replace(".", "\\.").as_str();
        let re = match Regex::new(copy.as_str()) {
            Ok(regex) => regex,
            Err(_) => return Err(PermissionError),
        };
        let perms_lock = self.permissions.read().unwrap();
        for regex in perms_lock.iter() {
            if regex.as_str() == re.as_str() {
                return Ok(());
            }
        }
        drop(perms_lock);
        let mut perms_lock = self.permissions.write().unwrap();
        perms_lock.push(re);
        Ok(())
    }

    pub async fn send_message(&self, message: String) -> bool {
        match self.get_connection_state().await {
            
            ConnectionState::Play => {
                self.queue_send_packet(CSystemChatMessage::new(
                    TextComponent::builder().text(message.as_str()).build(), 
                    false)).await;
                true
            },
            _ => false,
        }
    }
    
    pub fn get_data(&self) -> &RwLock<Option<EntityPlayer>> {
        &self.data
    }
    
    pub fn get_permissions(&self) -> &std::sync::RwLock<Permissions> {
        &self.permissions
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

        let mut connection_lock = player_arc.get_connection().lock().await;
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


