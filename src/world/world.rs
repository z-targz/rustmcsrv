use std::error::Error;
use std::sync::Weak;

use dashmap::DashMap;
use uuid::Uuid;

use crate::data_types::identifier::Identifier;
use crate::entity::entity_base::EntityBase;
use crate::packet::play;
use crate::player::Player;

pub struct World {
    players: DashMap<i32, Weak<Player>>,
    loaded_entities: DashMap<i32, Weak<dyn EntityBase>>,
    identifier: Identifier,
}

impl World {
    pub fn new(identifier: Identifier) -> Self  {
        World {
            players : DashMap::with_capacity(crate::THE_SERVER.get_max_players() as usize),
            loaded_entities: DashMap::new(),
            identifier : identifier,
        }
    }

    pub fn add_player(&self, player_id: i32, weak: Weak<Player>) {
        self.players.insert(player_id, weak);
    }

    pub fn remove_player_by_id(&self, player_id: i32) {
        self.players.remove(&player_id);
    }
}