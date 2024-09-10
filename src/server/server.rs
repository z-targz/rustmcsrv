use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::Weak;


use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;
use uuid::Uuid;

use crate::event::EventHandler;
use crate::event::EventManager;
use crate::event::HandlerList;
use crate::event::TraitEvent;
use crate::player::Player;
use crate::player::Players;

use crate::world::chunk_loader::Loader;
use crate::world::World;
use crate::ServerProperties;

#[derive(Debug)]
pub struct ServerFullError;

impl Error for ServerFullError {}

impl std::fmt::Display for ServerFullError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: Server is Full")
    }
}

pub struct Server {
    //const
    properties: ServerProperties,
    worlds: HashMap<String, Arc<tokio::sync::Mutex<World>>>,
    players: Players,
    entity_id_cap: Mutex<i32>,
    event_manager: EventManager,
    is_running: bool,
}

impl Server {
    pub fn new(properties: ServerProperties) -> Self {
        let max_players = properties.get_max_players();
        Server { 
            properties: properties,
            worlds: HashMap::with_capacity(3),
            players: Players::new(max_players),
            entity_id_cap: Mutex::new(0),
            event_manager: EventManager::new(),
            is_running: false,
        }
    }

    pub fn set_running(&mut self, value: bool) {
        self.is_running = value;
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub async fn get_next_eid(&self) -> i32 {
        let mut block_lock = self.entity_id_cap.lock().unwrap();   
        let idx_value = *block_lock;
        *block_lock += 1;
        drop(block_lock);
        idx_value
    }

    pub fn get_properties(&self) -> &ServerProperties {
        &self.properties
    }

    pub async fn register_player(&self, player: Player) -> Result<Arc<Player>, ServerFullError> {
        if self.players.get_num_players().await == self.get_max_players() {
            player.disconnect("Server is full!").await;
            return Err(ServerFullError);
        }
        Ok(self.players.add(player).await)
    }

    pub fn get_max_players(&self) -> i32 {
        self.get_properties().get_max_players()
    }

    pub async fn get_num_players_async(&self) -> i32 {
        self.players.get_num_players().await
    }

    pub fn get_num_players(&self) -> i32 {
        crate::RUNTIME.block_on(self.get_num_players_async())
    }

    pub fn get_motd(&self) -> &str {
        &self.get_properties().get_motd()
    }

    pub async fn get_players_async(&self) -> Vec<Weak<Player>> {
        self.players.get_players().await
    }

    pub fn get_players(&self) -> Vec<Weak<Player>> {
        crate::RUNTIME.block_on(self.get_players_async())
    }

    pub async fn get_player_by_id_async(&self, id: i32) -> Option<Weak<Player>> {
        self.players.get_by_id(id).await
    }

    pub fn get_player_by_id(&self, id: i32) -> Option<Weak<Player>> {
        crate::RUNTIME.block_on(self.get_player_by_id_async(id))
    }

    pub async fn get_player_by_uuid_async(&self, uuid: Uuid) -> Option<Weak<Player>> {
        self.players.get_by_uuid(uuid).await
    }

    pub fn get_player_by_uuid(&self, uuid: Uuid) -> Option<Weak<Player>> {
        crate::RUNTIME.block_on(self.get_player_by_uuid_async(uuid))
    }

    pub async fn get_player_by_name_async(&self, name: &str) -> Option<Weak<Player>> {
        self.players.get_by_name(name).await
    }

    pub fn get_player_by_name(&self, name: &str) -> Option<Weak<Player>> {
        crate::RUNTIME.block_on(self.get_player_by_name_async(name))
    }


    pub async fn drop_player_by_id_async(&self, id: i32) {
        self.players.drop_by_id(id).await;
    }

    pub fn drop_player_by_id(&self, id: i32) {
        crate::RUNTIME.block_on(self.drop_player_by_id_async(id))
    }

    pub async fn drop_player_by_uuid_async(&self, uuid: Uuid) {
        self.players.drop_by_uuid(uuid).await;
    }

    pub fn drop_player_by_uuid(&self, uuid: Uuid) {
        crate::RUNTIME.block_on(self.drop_player_by_uuid_async(uuid))
    }

    pub async fn tick_worlds(&'static self) {
        let mut handles = Vec::new();
        for (_, world) in &self.worlds {
            handles.push(tokio::spawn(async { world.lock().await.tick().await }))
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }

    pub async fn save_worlds(&'static self) {

    }

    pub fn get_event_manager(&self) -> &EventManager {
        &self.event_manager
    }

    pub fn register_event_handler<E: TraitEvent + PartialEq + Clone + 'static>(&self, handler: EventHandler<E>) {
        self.get_event_manager().register_event_handler::<E>(handler);
    }

}