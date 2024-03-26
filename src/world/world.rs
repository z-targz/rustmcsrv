use std::error::Error;
use std::sync::Weak;

use crate::data_types::identifier::{Identifier, InvalidIdentifier};
use crate::player::Player;

pub struct World {
    players: Vec<Weak<Player>>,
    identifier: Identifier,
}

impl World {
    pub fn new(identifier: Identifier) -> Self  {
        World {
            players : Vec::with_capacity(crate::THE_SERVER.get_max_players() as usize),
            identifier : identifier,
        }
    }
}