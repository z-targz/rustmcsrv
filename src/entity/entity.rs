
use serde::{Serialize, Deserialize};

server_macros::generate_entity_id_enum!{}

server_macros::create_entity_enum!{}

pub trait TickableEntity {
    fn tick(&mut self);
}