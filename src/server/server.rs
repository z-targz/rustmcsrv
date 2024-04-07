use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Weak;

use uuid::Uuid;

use crate::player::Player;
use crate::player::Players;

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
    worlds: Vec<World>,
    players: Players,
    entity_id_cap: Mutex<i32>,
}

impl Server {
    pub fn new(properties: ServerProperties) -> Self {
        let max_players = properties.get_max_players();
        Server { 
            properties : properties,
            worlds : Vec::with_capacity(3),
            players : Players::new(max_players),
            entity_id_cap : Mutex::new(0)
        }
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
        if self.players.get_num_players() == self.get_max_players() {
            player.disconnect("Server is full!").await;
            return Err(ServerFullError);
        }
        Ok(self.players.add(player).await)
    }

    pub fn get_max_players(&self) -> i32 {
        self.get_properties().get_max_players()
    }

    pub fn get_motd(&self) -> &String {
        &self.get_properties().get_motd()
    }

    pub fn get_players_iter(&self) -> impl Iterator<Item = Weak<Player>> + '_ {
        self.players.players_iter()
    }

    pub fn get_player_by_id(&self, id: i32) -> Option<Weak<Player>> {
        self.players.get_by_id(id)
    }

    pub fn get_player_by_uuid(&self, uuid: Uuid) -> Option<Weak<Player>> {
        self.players.get_by_uuid(uuid)
    }

    pub fn get_player_by_name(&self, name: &String) -> Option<Weak<Player>> {
        self.players.get_by_name(name)
    }

    pub fn drop_player_by_id(&self, id: i32) {
        self.players.drop_by_id(id);
    }

    pub fn drop_player_by_uuid(&self, uuid: Uuid) {
        self.players.drop_by_uuid(uuid);
    }

}