use std::error::Error;
use std::sync::Weak;

use crate::player::Player;

#[derive(Debug)]
pub struct InvalidIdentifier;

impl Error for InvalidIdentifier {}

impl std::fmt::Display for InvalidIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "World name must be in the format \"minecraft:{{worldname}}\", \"[custom_namespace]:{{worldname}}\", or \"{{worldname}}\", where the namespace is implied to be `minecraft`.")
    }
}

pub struct World {
    players: Vec<Weak<Player>>,
    namespace: String,
    name: String,
}

impl World {
    pub fn new(name: &str) -> Result<Self, InvalidIdentifier>  {
        let world_name: Vec<&str> = name.split(":").collect();
        let namespace = 
        if world_name.len() > 1 { 
            world_name[0] 
        } else if world_name.len() == 1 { 
            "minecraft" 
        } else { 
            return Err(InvalidIdentifier) 
        };
        Ok(World {
            players: Vec::with_capacity(crate::THE_SERVER.get_max_players() as usize),
            namespace: namespace.to_string(),
            name: world_name.last().unwrap().to_string(),
        })
    }
}