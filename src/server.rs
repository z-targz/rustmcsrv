use std::sync::Arc;
use std::sync::Weak;

use uuid::Uuid;

use crate::player::Player;
use crate::player::Players;

pub struct Server {
    //const
    max_players: usize,
    motd: String,
    //nonconst
    players: Players,
}

impl Server {
    pub fn new(max_players: usize, motd: &String) -> Self {
        Server { 
            max_players : max_players,
            motd : motd.clone(),
            players : Players::new(max_players),
        }
    }

    pub async fn register_player(&self, player: Player) -> Arc<Player> {
        self.players.add(player).await
    }

    pub fn get_max_players(&self) -> usize {
        self.max_players
    }

    pub fn get_motd(&self) -> &String {
        &self.motd
    }

    pub fn get_players_iter(&self) -> impl Iterator<Item = Weak<Player>> + '_ {
        self.players.players_iter()
    }

    pub fn get_player_by_id(&self, id: usize) -> Option<Weak<Player>> {
        self.players.get_by_id(id)
    }

    pub fn get_player_by_uuid(&self, uuid: Uuid) -> Option<Weak<Player>> {
        self.players.get_by_uuid(uuid)
    }

    pub fn get_player_by_name(&self, name: &String) -> Option<Weak<Player>> {
        self.players.get_by_name(name)
    }

    pub fn drop_player_by_id(&self, id: usize) {
        self.players.drop_by_id(id);
    }

    pub fn drop_player_by_uuid(&self, uuid: Uuid) {
        self.players.drop_by_uuid(uuid);
    }

}